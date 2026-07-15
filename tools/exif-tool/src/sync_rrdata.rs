//! sync-rrdata：把 RapidRAW 的 .rrdata sidecar 里的评分/色标同步到图片。
//!
//! 设计原则：
//! - EXIF 是相机原始数据的地盘，不动它。
//! - 评分（rating）和色标（color label）只写入 XMP：
//!   - JPEG：嵌入图片内部的 XMP APP1 segment（xmp:Rating + xmp:Label）。
//!   - 非 JPEG：生成同目录 .xmp sidecar。
//! - 只读 .rrdata 里的 `rating` 和 `tags` 中 `color:*` 标签；
//!   不读 `adjustments`，也不读 `exif` 里的相机参数。
//! - 目录结构按约定一致，支持按文件名 stem 或 EXIF DateTimeOriginal 匹配。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use serde::Deserialize;
use walkdir::WalkDir;

const RRDATA_EXT: &str = "rrdata";
const XMP_SIGNATURE: &[u8] = b"http://ns.adobe.com/xap/1.0/\0";

/// RapidRAW .rrdata 的 JSON 结构。
/// 我们只关心 `rating` 和 `tags` 里的 color 标签；
/// `adjustments` 和 `exif` 被忽略。
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RrData {
    rating: u8,
    #[serde(rename = "adjustments")]
    _adjustments: serde_json::Value,
    tags: Option<Vec<String>>,
    exif: Option<HashMap<String, String>>,
}

/// 从 .rrdata 提取出的、后续匹配要用的信息。
#[derive(Debug, Clone)]
struct RrDataInfo {
    relative_dir: PathBuf,
    _source_stem: String,
    rating: Option<u8>,
    label: Option<String>,
    datetime: Option<String>,
}

/// 目标图片与 .rrdata 的匹配策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchMode {
    /// 按去掉 suffix 后的文件名 stem 匹配。
    Stem,
    /// 按 EXIF DateTimeOriginal 匹配（用于文件名带序号/日期的模板）。
    Datetime,
    /// 先尝试 stem，失败再回退到 datetime。
    Auto,
}

impl std::str::FromStr for MatchMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stem" => Ok(MatchMode::Stem),
            "datetime" => Ok(MatchMode::Datetime),
            "auto" => Ok(MatchMode::Auto),
            _ => Err(format!("unknown match mode: {}", s)),
        }
    }
}

/// 入口函数：递归遍历 `source_dir` 的 .rrdata 和 `target_dir` 的图片，
/// 按匹配策略配对后把 rating/label 写入 XMP。
pub fn run(
    source_dir: &Path,
    target_dir: &Path,
    match_mode: MatchMode,
    suffix: &str,
    dry_run: bool,
    force: bool,
) -> anyhow::Result<()> {
    if !source_dir.is_dir() {
        return Err(anyhow::anyhow!("Source is not a directory: {}", source_dir.display()));
    }
    if !target_dir.is_dir() {
        return Err(anyhow::anyhow!("Target is not a directory: {}", target_dir.display()));
    }

    let rrdata_map = collect_rrdata(source_dir)?;
    if rrdata_map.is_empty() {
        println!("No .rrdata files found in {}", source_dir.display());
        return Ok(());
    }

    let target_images = collect_images(target_dir);
    if target_images.is_empty() {
        println!("No images found in {}", target_dir.display());
        return Ok(());
    }

    let mut matched = 0usize;
    let mut skipped = 0usize;
    let mut no_match = 0usize;
    let mut errors = 0usize;

    for target_path in &target_images {
        let relative_dir = target_path
            .parent()
            .and_then(|p| p.strip_prefix(target_dir).ok())
            .unwrap_or(Path::new(""))
            .to_path_buf();

        let target_stem = target_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let target_datetime = read_target_datetime(target_path);

        let info = match find_matching_rrdata(
            &rrdata_map,
            &relative_dir,
            &target_stem,
            suffix,
            target_datetime.as_deref(),
            match_mode,
        ) {
            Some(info) => info,
            None => {
                println!("[NO MATCH] {}", target_path.display());
                no_match += 1;
                continue;
            }
        };

        if info.rating.is_none() && info.label.is_none() {
            println!("[SKIP]     {} (no rating/label in rrdata)", target_path.display());
            skipped += 1;
            continue;
        }

        let has_existing = !force && has_existing_rating_or_label(target_path)?;
        if has_existing {
            println!(
                "[SKIP]     {} (already has rating/label; use --force to overwrite)",
                target_path.display()
            );
            skipped += 1;
            continue;
        }

        if dry_run {
            println!(
                "[DRY RUN]  {} <- rating={:?}, label={:?}",
                target_path.display(),
                info.rating,
                info.label
            );
            matched += 1;
            continue;
        }

        match apply_metadata(target_path, info.rating, info.label.as_deref()) {
            Ok(_) => {
                println!(
                    "[SET]      {} <- rating={:?}, label={:?}",
                    target_path.display(),
                    info.rating,
                    info.label
                );
                matched += 1;
            }
            Err(e) => {
                println!("[ERROR]    {} - {:#}", target_path.display(), e);
                errors += 1;
            }
        }
    }

    println!();
    println!(
        "Processed {} image(s): {} set, {} skipped, {} no match, {} errors",
        target_images.len(),
        matched,
        skipped,
        no_match,
        errors
    );
    Ok(())
}

fn collect_rrdata(source_dir: &Path) -> anyhow::Result<HashMap<PathBuf, RrDataInfo>> {
    let mut map = HashMap::new();

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext != RRDATA_EXT {
            continue;
        }

        let relative_dir = path
            .parent()
            .and_then(|p| p.strip_prefix(source_dir).ok())
            .unwrap_or(Path::new(""))
            .to_path_buf();

        let source_stem = rrdata_source_stem(path);

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("read rrdata {}", path.display()))?;
        let rrdata: RrData = serde_json::from_str(&content)
            .with_context(|| format!("parse rrdata {}", path.display()))?;

        let label = rrdata
            .tags
            .as_ref()
            .and_then(|tags| {
                tags.iter().find_map(|t| {
                    t.strip_prefix("color:").map(|c| {
                        let mut chars = c.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    })
                })
            });

        let rating = if rrdata.rating > 0 {
            Some(rrdata.rating)
        } else {
            None
        };

        let datetime = rrdata
            .exif
            .as_ref()
            .and_then(|m| m.get("DateTimeOriginal").cloned());

        map.insert(
            relative_dir.join(&source_stem),
            RrDataInfo {
                relative_dir,
                _source_stem: source_stem,
                rating,
                label,
                datetime,
            },
        );
    }

    Ok(map)
}

/// RapidRAW sidecars are named `<image filename>.<image ext>.rrdata`.
/// Strip both the sidecar extension and the original image extension so the
/// result can be compared with an exported target image's stem.
fn rrdata_source_stem(path: &Path) -> String {
    let sidecar_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    Path::new(sidecar_stem)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(sidecar_stem)
        .to_string()
}

fn collect_images(dir: &Path) -> Vec<PathBuf> {
    const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "tiff", "tif"];

    WalkDir::new(dir)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| IMAGE_EXTENSIONS.contains(&e.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect()
}

fn find_matching_rrdata<'a>(
    rrdata_map: &'a HashMap<PathBuf, RrDataInfo>,
    relative_dir: &Path,
    target_stem: &str,
    suffix: &str,
    target_datetime: Option<&str>,
    mode: MatchMode,
) -> Option<&'a RrDataInfo> {
    // Try stem matching first if mode allows.
    if mode == MatchMode::Stem || mode == MatchMode::Auto {
        let source_stem = if !suffix.is_empty() && target_stem.ends_with(suffix) {
            &target_stem[..target_stem.len() - suffix.len()]
        } else {
            target_stem
        };
        let key = relative_dir.join(source_stem);
        if let Some(info) = rrdata_map.get(&key) {
            return Some(info);
        }

        // Also try the plain target stem in case suffix stripping was wrong.
        let key2 = relative_dir.join(target_stem);
        if let Some(info) = rrdata_map.get(&key2) {
            return Some(info);
        }
    }

    // Fall back to datetime matching within the same relative directory.
    if mode == MatchMode::Datetime || mode == MatchMode::Auto {
        let dt = target_datetime?;
        for info in rrdata_map.values() {
            if info.relative_dir == relative_dir
                && let Some(ref info_dt) = info.datetime
                && normalize_datetime(info_dt) == normalize_datetime(dt)
            {
                return Some(info);
            }
        }
    }

    None
}

fn normalize_datetime(s: &str) -> String {
    s.replace(['-', ':', ' ', 'T'], "")
        .trim()
        .to_string()
}

fn read_target_datetime(path: &Path) -> Option<String> {
    let metadata = Metadata::new_from_path(path).ok()?;
    metadata
        .get_tag(&ExifTag::DateTimeOriginal(String::new()))
        .next()
        .and_then(|tag| match tag {
            ExifTag::DateTimeOriginal(s) => Some(s.clone()),
            _ => None,
        })
}

/// 检查目标图片是否已经有 XMP Rating/Label。
/// 注意：不检查 EXIF Rating，因为 EXIF 应保留给相机原始数据。
fn has_existing_rating_or_label(path: &Path) -> anyhow::Result<bool> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "jpg" || ext == "jpeg" {
        let bytes = std::fs::read(path)?;
        if has_embedded_xmp_rating_or_label(&bytes) {
            return Ok(true);
        }
    }

    // 检查是否存在同名的 .xmp sidecar。
    let xmp_path = path.with_extension("xmp");
    if xmp_path.exists() {
        let content = std::fs::read_to_string(&xmp_path).unwrap_or_default();
        if content.contains("xmp:Rating") || content.contains("xmp:Label") {
            return Ok(true);
        }
    }

    Ok(false)
}

fn has_embedded_xmp_rating_or_label(jpeg_bytes: &[u8]) -> bool {
    let Some(packet) = extract_xmp_packet(jpeg_bytes) else {
        return false;
    };
    let text = String::from_utf8_lossy(&packet);
    text.contains("xmp:Rating") || text.contains("xmp:Label")
}

/// 将 rating/label 写入目标图片的 XMP。
/// JPEG 直接嵌入 XMP APP1 segment；其他格式生成 .xmp sidecar。
/// 整个过程不修改 EXIF。
fn apply_metadata(
    path: &Path,
    rating: Option<u8>,
    label: Option<&str>,
) -> anyhow::Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let xmp_packet = build_xmp_packet(rating, label);

    match ext.as_str() {
        "jpg" | "jpeg" => {
            // Keep the original EXIF untouched; only embed/refresh the XMP APP1
            // segment with rating and label.
            let jpeg_bytes = std::fs::read(path)?;
            let embedded = embed_xmp_in_jpeg(&jpeg_bytes, &xmp_packet)?;
            std::fs::write(path, embedded)?;
        }
        _ => {
            // For non-JPEG, write an XMP sidecar.
            let xmp_path = path.with_extension("xmp");
            std::fs::write(&xmp_path, xmp_packet)
                .with_context(|| format!("write XMP sidecar {}", xmp_path.display()))?;
        }
    }

    Ok(())
}

/// 构造仅含 xmp:Rating 和 xmp:Label 的最小 XMP packet。
/// 如果 rating 和 label 都为空，返回空字符串，调用方跳过写入。
fn build_xmp_packet(rating: Option<u8>, label: Option<&str>) -> String {
    let mut body = String::new();
    if let Some(r) = rating {
        body.push_str(&format!("    <xmp:Rating>{}</xmp:Rating>\n", r));
    }
    if let Some(l) = label {
        body.push_str(&format!("    <xmp:Label>{}</xmp:Label>\n", l));
    }

    if body.is_empty() {
        return String::new();
    }

    format!(
        r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="RapidRAW Sync">
 <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <rdf:Description rdf:about=""
    xmlns:xmp="http://ns.adobe.com/xap/1.0/">
{}
  </rdf:Description>
 </rdf:RDF>
</x:xmpmeta>
<?xpacket end="w"?>"#,
        body
    )
}

/// 从 JPEG 字节流里提取已有的内嵌 XMP packet。
pub(crate) fn extract_xmp_packet(jpeg_bytes: &[u8]) -> Option<Vec<u8>> {
    let mut i = 0;
    if jpeg_bytes.len() < 2 || jpeg_bytes[0] != 0xFF || jpeg_bytes[1] != 0xD8 {
        return None;
    }
    i += 2;

    while i + 4 < jpeg_bytes.len() {
        if jpeg_bytes[i] != 0xFF {
            return None;
        }
        let marker = jpeg_bytes[i + 1];

        // Standalone markers.
        if marker == 0xD9 || marker == 0xD8 || marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            i += 2;
            continue;
        }

        let len = u16::from_be_bytes([jpeg_bytes[i + 2], jpeg_bytes[i + 3]]) as usize;
        let segment_end = i + 2 + len;
        if segment_end > jpeg_bytes.len() {
            return None;
        }

        if marker == 0xE1 && len > 29 {
            let payload_start = i + 4;
            let sig_end = payload_start + 29;
            if sig_end <= segment_end && &jpeg_bytes[payload_start..sig_end] == XMP_SIGNATURE {
                return Some(jpeg_bytes[sig_end..segment_end].to_vec());
            }
        }

        i = segment_end;
    }

    None
}

/// 把 XMP packet 嵌入 JPEG：删除旧的 XMP APP1 segment，
/// 在非 APP 标记前插入新的 XMP APP1 segment。
/// 其他所有 segment（包括 EXIF）原样保留。
pub(crate) fn embed_xmp_in_jpeg(
    jpeg_bytes: &[u8],
    xmp_packet: &str,
) -> anyhow::Result<Vec<u8>> {
    if xmp_packet.is_empty() {
        return Ok(jpeg_bytes.to_vec());
    }

    let xmp_payload = [XMP_SIGNATURE, xmp_packet.as_bytes()].concat();

    if jpeg_bytes.len() < 2 || jpeg_bytes[0] != 0xFF || jpeg_bytes[1] != 0xD8 {
        return Err(anyhow::anyhow!("Invalid JPEG: missing SOI"));
    }

    let mut output = Vec::with_capacity(jpeg_bytes.len() + xmp_payload.len() + 100);
    let mut i = 0;

    // Copy SOI.
    output.extend_from_slice(&jpeg_bytes[0..2]);
    i += 2;

    let mut inserted_xmp = false;

    while i + 2 < jpeg_bytes.len() {
        if jpeg_bytes[i] != 0xFF {
            // Entropy-coded data: copy the rest.
            output.extend_from_slice(&jpeg_bytes[i..]);
            break;
        }

        let marker = jpeg_bytes[i + 1];

        // Standalone markers.
        if marker == 0xD8 {
            // SOI mid-file: shouldn't happen, but copy and continue.
            output.extend_from_slice(&jpeg_bytes[i..i + 2]);
            i += 2;
            continue;
        }
        if marker == 0xD9 {
            // EOI: insert XMP if not yet inserted, then EOI.
            if !inserted_xmp {
                append_xmp_app1(&mut output, &xmp_payload);
                inserted_xmp = true;
            }
            output.extend_from_slice(&[0xFF, 0xD9]);
            break;
        }
        if marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            output.extend_from_slice(&[0xFF, marker]);
            i += 2;
            continue;
        }

        // Marker with length.
        if i + 4 > jpeg_bytes.len() {
            return Err(anyhow::anyhow!("Invalid JPEG: truncated segment"));
        }
        let len = u16::from_be_bytes([jpeg_bytes[i + 2], jpeg_bytes[i + 3]]) as usize;
        let segment_end = i + 2 + len;
        if segment_end > jpeg_bytes.len() {
            return Err(anyhow::anyhow!("Invalid JPEG: segment length out of bounds"));
        }

        // Skip existing XMP APP1 segments.
        let is_xmp_app1 = marker == 0xE1
            && len > 29
            && i + 4 + 29 <= segment_end
            && &jpeg_bytes[i + 4..i + 4 + 29] == XMP_SIGNATURE;

        if !is_xmp_app1 {
            output.extend_from_slice(&jpeg_bytes[i..segment_end]);
        }

        // Insert XMP before the first non-APP marker for conventional ordering.
        if !inserted_xmp && !(0xE0..=0xEF).contains(&marker) {
            append_xmp_app1(&mut output, &xmp_payload);
            inserted_xmp = true;
        }

        i = segment_end;
    }

    if !inserted_xmp {
        append_xmp_app1(&mut output, &xmp_payload);
    }

    Ok(output)
}

fn append_xmp_app1(output: &mut Vec<u8>, xmp_payload: &[u8]) {
    let len = (xmp_payload.len() + 2) as u16;
    output.extend_from_slice(&[0xFF, 0xE1]);
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(xmp_payload);
}

#[cfg(test)]
mod tests {
    use super::rrdata_source_stem;
    use std::path::Path;

    #[test]
    fn strips_original_image_extension_from_rrdata_sidecar() {
        assert_eq!(
            rrdata_source_stem(Path::new("DSC02645-2.jpg.rrdata")),
            "DSC02645-2"
        );
        assert_eq!(
            rrdata_source_stem(Path::new("DSC01545.ARW.rrdata")),
            "DSC01545"
        );
    }

    #[test]
    fn preserves_complex_image_stems() {
        assert_eq!(
            rrdata_source_stem(Path::new("堆栈_stacked-堆栈 6400 0.25.jpg.rrdata")),
            "堆栈_stacked-堆栈 6400 0.25"
        );
        assert_eq!(
            rrdata_source_stem(Path::new(
                "堆栈-带楼前景_stacked-堆栈-带楼前景 6400 0.25-前景重合成.jpg.rrdata"
            )),
            "堆栈-带楼前景_stacked-堆栈-带楼前景 6400 0.25-前景重合成"
        );
    }
}
