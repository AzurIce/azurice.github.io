//! Conservative, in-place repair of known non-standard EXIF encodings.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Context;
use little_exif::filetype::FileExtension;
use little_exif::metadata::Metadata;
use walkdir::WalkDir;

const USER_COMMENT_TAG: u16 = 0x9286;
const TIFF_TYPE_ASCII: u16 = 2;
const TIFF_TYPE_LONG: u16 = 4;
const TIFF_TYPE_UNDEFINED: u16 = 7;

#[derive(Debug, Clone, Copy)]
enum Endian {
    Little,
    Big,
}

#[derive(Debug, Clone, Copy)]
enum IfdKind {
    Generic,
    Exif,
    Gps,
    Interop,
}

#[derive(Debug, Clone, Copy)]
struct Repair {
    tag: u16,
    old_type: u16,
    new_type: u16,
}

pub fn run(path: &Path, dry_run: bool) -> anyhow::Result<()> {
    let images = collect_jpegs(path)?;
    let show_unchanged = path.is_file();
    let mut repaired = 0usize;
    let mut unchanged = 0usize;
    let mut errors = 0usize;

    for image in &images {
        match repair_file(image, dry_run) {
            Ok(repairs) if repairs.is_empty() => {
                if show_unchanged {
                    println!("[OK]       {}", image.display());
                }
                unchanged += 1;
            }
            Ok(repairs) => {
                let action = if dry_run { "[DRY RUN]" } else { "[FIXED]  " };
                for repair in repairs {
                    println!(
                        "{} {} - {} (0x{:04X}) type {} -> {}",
                        action,
                        image.display(),
                        tag_name(repair.tag),
                        repair.tag,
                        tiff_type_name(repair.old_type),
                        tiff_type_name(repair.new_type)
                    );
                }
                repaired += 1;
            }
            Err(error) => {
                eprintln!("[ERROR]    {} - {:#}", image.display(), error);
                errors += 1;
            }
        }
    }

    println!();
    println!(
        "Processed {} JPEG image(s): {} repaired, {} unchanged, {} errors",
        images.len(),
        repaired,
        unchanged,
        errors
    );

    if errors > 0 {
        return Err(anyhow::anyhow!("Failed to repair {} image(s)", errors));
    }

    Ok(())
}

fn collect_jpegs(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if path.is_file() {
        if is_jpeg(path) {
            return Ok(vec![path.to_path_buf()]);
        }
        return Err(anyhow::anyhow!("Not a JPEG image: {}", path.display()));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
    }

    let mut images = WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| is_jpeg(path))
        .collect::<Vec<_>>();
    images.sort();
    Ok(images)
}

fn is_jpeg(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase()),
        Some(extension) if extension == "jpg" || extension == "jpeg"
    )
}

fn repair_file(path: &Path, dry_run: bool) -> anyhow::Result<Vec<Repair>> {
    let original = std::fs::read(path)
        .with_context(|| format!("read image {}", path.display()))?;
    let mut updated = original.clone();
    let repairs = standardize_jpeg_exif(&mut updated)?;

    if repairs.is_empty() {
        return Ok(repairs);
    }

    Metadata::new_from_vec(&updated, FileExtension::JPEG)
        .with_context(|| "repaired EXIF still cannot be decoded by little_exif")?;

    if dry_run {
        return Ok(repairs);
    }

    std::fs::write(path, &updated)
        .with_context(|| format!("write repaired image {}", path.display()))?;

    if let Err(error) = Metadata::new_from_path(path) {
        std::fs::write(path, &original)
            .with_context(|| format!("restore original image {}", path.display()))?;
        return Err(error).with_context(|| "verification failed; original image was restored");
    }

    Ok(repairs)
}

fn standardize_jpeg_exif(jpeg: &mut [u8]) -> anyhow::Result<Vec<Repair>> {
    if jpeg.len() < 2 || jpeg[0..2] != [0xFF, 0xD8] {
        return Err(anyhow::anyhow!("Invalid JPEG: missing SOI marker"));
    }

    let mut repairs = Vec::new();
    let mut offset = 2usize;

    while offset + 4 <= jpeg.len() {
        if jpeg[offset] != 0xFF {
            break;
        }

        let marker = jpeg[offset + 1];
        if marker == 0xDA || marker == 0xD9 {
            break;
        }
        if marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            offset += 2;
            continue;
        }

        let length = u16::from_be_bytes([jpeg[offset + 2], jpeg[offset + 3]]) as usize;
        if length < 2 {
            return Err(anyhow::anyhow!("Invalid JPEG segment length"));
        }
        let segment_end = offset
            .checked_add(2 + length)
            .filter(|end| *end <= jpeg.len())
            .ok_or_else(|| anyhow::anyhow!("JPEG segment exceeds file bounds"))?;
        let payload_start = offset + 4;

        if marker == 0xE1
            && payload_start + 6 <= segment_end
            && &jpeg[payload_start..payload_start + 6] == b"Exif\0\0"
        {
            repairs.extend(standardize_tiff(
                &mut jpeg[payload_start + 6..segment_end],
            )?);
        }

        offset = segment_end;
    }

    Ok(repairs)
}

fn standardize_tiff(tiff: &mut [u8]) -> anyhow::Result<Vec<Repair>> {
    if tiff.len() < 8 {
        return Err(anyhow::anyhow!("Invalid EXIF TIFF header"));
    }

    let endian = match &tiff[0..2] {
        b"II" => Endian::Little,
        b"MM" => Endian::Big,
        _ => return Err(anyhow::anyhow!("Invalid EXIF byte order")),
    };
    if read_u16(tiff, 2, endian)? != 42 {
        return Err(anyhow::anyhow!("Invalid EXIF TIFF magic number"));
    }

    let first_ifd = read_u32(tiff, 4, endian)?;
    let mut pending = vec![(first_ifd, IfdKind::Generic)];
    let mut visited = HashSet::new();
    let mut repairs = Vec::new();

    while let Some((ifd_offset, ifd_kind)) = pending.pop() {
        if ifd_offset == 0 || !visited.insert(ifd_offset) {
            continue;
        }
        if visited.len() > 64 {
            return Err(anyhow::anyhow!("Too many EXIF IFDs"));
        }

        let ifd_offset = usize::try_from(ifd_offset)?;
        let entry_count = read_u16(tiff, ifd_offset, endian)? as usize;
        let entries_start = ifd_offset
            .checked_add(2)
            .ok_or_else(|| anyhow::anyhow!("EXIF IFD offset overflow"))?;
        let entries_end = entries_start
            .checked_add(entry_count.checked_mul(12).ok_or_else(|| {
                anyhow::anyhow!("EXIF IFD entry count overflow")
            })?)
            .ok_or_else(|| anyhow::anyhow!("EXIF IFD size overflow"))?;
        if entries_end + 4 > tiff.len() {
            return Err(anyhow::anyhow!("EXIF IFD exceeds segment bounds"));
        }

        for index in 0..entry_count {
            let entry = entries_start + index * 12;
            let tag = read_u16(tiff, entry, endian)?;
            let field_type = read_u16(tiff, entry + 2, endian)?;
            let component_count = read_u32(tiff, entry + 4, endian)?;

            if expects_undefined(ifd_kind, tag) && field_type == TIFF_TYPE_ASCII {
                write_u16(tiff, entry + 2, TIFF_TYPE_UNDEFINED, endian)?;
                repairs.push(Repair {
                    tag,
                    old_type: field_type,
                    new_type: TIFF_TYPE_UNDEFINED,
                });
            }

            if field_type == TIFF_TYPE_LONG && component_count == 1 {
                let child_kind = match tag {
                    0x8769 => Some(IfdKind::Exif),
                    0x8825 => Some(IfdKind::Gps),
                    0xA005 => Some(IfdKind::Interop),
                    _ => None,
                };
                if let Some(child_kind) = child_kind {
                    pending.push((read_u32(tiff, entry + 8, endian)?, child_kind));
                }
            }
        }

        pending.push((read_u32(tiff, entries_end, endian)?, ifd_kind));
    }

    Ok(repairs)
}

fn read_u16(data: &[u8], offset: usize, endian: Endian) -> anyhow::Result<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| anyhow::anyhow!("EXIF read exceeds segment bounds"))?;
    Ok(match endian {
        Endian::Little => u16::from_le_bytes([bytes[0], bytes[1]]),
        Endian::Big => u16::from_be_bytes([bytes[0], bytes[1]]),
    })
}

fn read_u32(data: &[u8], offset: usize, endian: Endian) -> anyhow::Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| anyhow::anyhow!("EXIF read exceeds segment bounds"))?;
    Ok(match endian {
        Endian::Little => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        Endian::Big => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
    })
}

fn write_u16(
    data: &mut [u8],
    offset: usize,
    value: u16,
    endian: Endian,
) -> anyhow::Result<()> {
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    data.get_mut(offset..offset + 2)
        .ok_or_else(|| anyhow::anyhow!("EXIF write exceeds segment bounds"))?
        .copy_from_slice(&bytes);
    Ok(())
}

fn tag_name(tag: u16) -> &'static str {
    match tag {
        0x001B => "GPSProcessingMethod",
        0x001C => "GPSAreaInformation",
        0x8828 => "OECF",
        0x9000 => "ExifVersion",
        0x9101 => "ComponentsConfiguration",
        0x927C => "MakerNote",
        USER_COMMENT_TAG => "UserComment",
        0xA000 => "FlashpixVersion",
        0xA300 => "FileSource",
        0xA301 => "SceneType",
        0xA302 => "CFAPattern",
        0xA40B => "DeviceSettingDescription",
        0xA462 => "CompositeImageExposureTimes",
        _ => "Unknown",
    }
}

fn expects_undefined(ifd_kind: IfdKind, tag: u16) -> bool {
    match ifd_kind {
        IfdKind::Exif => matches!(
            tag,
            0x8828
                | 0x9000
                | 0x9101
                | 0x927C
                | USER_COMMENT_TAG
                | 0xA000
                | 0xA300
                | 0xA301
                | 0xA302
                | 0xA40B
                | 0xA462
        ),
        IfdKind::Gps => matches!(tag, 0x001B | 0x001C),
        IfdKind::Interop => tag == 0x0002,
        IfdKind::Generic => false,
    }
}

fn tiff_type_name(field_type: u16) -> &'static str {
    match field_type {
        TIFF_TYPE_ASCII => "ASCII",
        TIFF_TYPE_LONG => "LONG",
        TIFF_TYPE_UNDEFINED => "UNDEFINED",
        _ => "UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IfdKind, TIFF_TYPE_UNDEFINED, expects_undefined, standardize_jpeg_exif,
    };

    #[test]
    fn repairs_ascii_user_comment_type() {
        let mut jpeg = malformed_user_comment_jpeg();
        let repairs = standardize_jpeg_exif(&mut jpeg).unwrap();

        assert_eq!(repairs.len(), 1);
        assert_eq!(u16::from_le_bytes([jpeg[42], jpeg[43]]), TIFF_TYPE_UNDEFINED);
    }

    #[test]
    fn undefined_tag_rules_are_ifd_specific() {
        assert!(expects_undefined(IfdKind::Exif, 0x9286));
        assert!(expects_undefined(IfdKind::Gps, 0x001B));
        assert!(expects_undefined(IfdKind::Interop, 0x0002));
        assert!(!expects_undefined(IfdKind::Generic, 0x0002));
        assert!(!expects_undefined(IfdKind::Exif, 0x0002));
    }

    fn malformed_user_comment_jpeg() -> Vec<u8> {
        let mut tiff = Vec::new();
        tiff.extend_from_slice(b"II");
        tiff.extend_from_slice(&42u16.to_le_bytes());
        tiff.extend_from_slice(&8u32.to_le_bytes());

        tiff.extend_from_slice(&1u16.to_le_bytes());
        tiff.extend_from_slice(&0x8769u16.to_le_bytes());
        tiff.extend_from_slice(&4u16.to_le_bytes());
        tiff.extend_from_slice(&1u32.to_le_bytes());
        tiff.extend_from_slice(&26u32.to_le_bytes());
        tiff.extend_from_slice(&0u32.to_le_bytes());

        tiff.extend_from_slice(&1u16.to_le_bytes());
        tiff.extend_from_slice(&0x9286u16.to_le_bytes());
        tiff.extend_from_slice(&2u16.to_le_bytes());
        tiff.extend_from_slice(&1u32.to_le_bytes());
        tiff.extend_from_slice(&[b'x', 0, 0, 0]);
        tiff.extend_from_slice(&0u32.to_le_bytes());

        let mut payload = b"Exif\0\0".to_vec();
        payload.extend_from_slice(&tiff);

        let mut jpeg = vec![0xFF, 0xD8, 0xFF, 0xE1];
        jpeg.extend_from_slice(&((payload.len() + 2) as u16).to_be_bytes());
        jpeg.extend_from_slice(&payload);
        jpeg.extend_from_slice(&[0xFF, 0xD9]);
        jpeg
    }
}
