mod fix_exif;
mod sync_rrdata;

use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use regex::Regex;
use walkdir::WalkDir;

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "tif"];

#[derive(Parser)]
#[command(name = "exif-tool", about = "Check and fix EXIF dates for gallery images")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum DateSource {
    /// Use the file's last modification time.
    Metadata,
    /// Parse a complete date and time from the filename.
    Filename,
}

#[derive(Subcommand)]
enum Commands {
    /// Show EXIF tags and XMP metadata for one image.
    Show {
        /// Path to an image file.
        path: PathBuf,
    },
    /// Check whether images have an EXIF date tag.
    Check {
        /// Path to an image file or a directory to scan recursively.
        path: PathBuf,
    },
    /// Recursively list images without EXIF DateTimeOriginal or ModifyDate.
    MissingDates {
        /// Directory containing images to scan recursively.
        path: PathBuf,
    },
    /// Repair known non-standard EXIF encodings without re-encoding images.
    Fix {
        /// JPEG image or directory containing JPEG images to repair.
        path: PathBuf,
        /// Print repairs without writing files.
        #[arg(long)]
        dry_run: bool,
    },
    /// Set EXIF dates for images that are missing them.
    Set {
        /// Path to an image file or a directory to scan recursively.
        path: PathBuf,
        /// Overwrite existing EXIF dates.
        #[arg(long)]
        force: bool,
        /// Dry run: print what would be changed without writing.
        #[arg(long)]
        dry_run: bool,
        /// Date source: metadata (file modification time) or filename.
        #[arg(long, value_enum, default_value_t = DateSource::Metadata)]
        date_source: DateSource,
    },
    /// Sync RapidRAW .rrdata sidecar rating/label into target images.
    #[command(name = "sync-rrdata")]
    SyncRrData {
        /// Source directory containing .rrdata sidecar files.
        source_dir: PathBuf,
        /// Target directory with images to update.
        target_dir: PathBuf,
        /// Matching strategy: stem | datetime | auto.
        #[arg(long, default_value = "auto")]
        match_mode: sync_rrdata::MatchMode,
        /// Suffix to strip from target filenames when matching (e.g. "_gallery").
        #[arg(long, default_value = "_gallery")]
        suffix: String,
        /// Overwrite existing rating/label.
        #[arg(long)]
        force: bool,
        /// Dry run: print what would be changed without writing.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Show { path } => show(path),
        Commands::Check { path } => check(path),
        Commands::MissingDates { path } => missing_dates(path),
        Commands::Fix { path, dry_run } => fix_exif::run(&path, dry_run),
        Commands::Set {
            path,
            force,
            dry_run,
            date_source,
        } => set(path, force, dry_run, date_source),
        Commands::SyncRrData {
            source_dir,
            target_dir,
            match_mode,
            suffix,
            force,
            dry_run,
        } => sync_rrdata::run(&source_dir, &target_dir, match_mode, &suffix, dry_run, force),
    }
}

fn show(path: PathBuf) -> anyhow::Result<()> {
    if !path.is_file() {
        return Err(anyhow::anyhow!("Image is not a file: {}", path.display()));
    }
    if !is_image(&path) {
        return Err(anyhow::anyhow!("Unsupported image type: {}", path.display()));
    }

    println!("File: {}", path.display());
    println!();
    println!("EXIF:");

    let metadata = read_metadata(&path)?;
    let mut tag_count = 0usize;
    for tag in &metadata {
        println!(
            "  [{:?}] 0x{:04X} {}",
            tag.get_group(),
            tag.as_u16(),
            truncate_text(&format!("{:?}", tag), 300)
        );
        tag_count += 1;
    }
    if tag_count == 0 {
        println!("  (none)");
    }

    println!();
    println!("XMP:");

    let mut xmp_count = 0usize;
    if matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "jpg" || ext == "jpeg"
    ) {
        let bytes = std::fs::read(&path)
            .with_context(|| format!("read image {}", path.display()))?;
        if let Some(packet) = sync_rrdata::extract_xmp_packet(&bytes) {
            println!("  Source: embedded");
            print_xmp_packet(&packet);
            xmp_count += 1;
        }
    }

    let sidecar_path = path.with_extension("xmp");
    if sidecar_path.is_file() {
        let packet = std::fs::read(&sidecar_path)
            .with_context(|| format!("read XMP sidecar {}", sidecar_path.display()))?;
        if xmp_count > 0 {
            println!();
        }
        println!("  Source: sidecar ({})", sidecar_path.display());
        print_xmp_packet(&packet);
        xmp_count += 1;
    }

    if xmp_count == 0 {
        println!("  (none)");
    }

    Ok(())
}

fn print_xmp_packet(packet: &[u8]) {
    let text = String::from_utf8_lossy(packet);
    for line in text.trim().lines() {
        println!("  {}", line);
    }
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        format!("{}...", truncated)
    } else {
        truncated
    }
}

fn check(path: PathBuf) -> anyhow::Result<()> {
    let images = collect_images(&path);
    if images.is_empty() {
        println!("No images found at {}", path.display());
        return Ok(());
    }

    let mut ok = 0usize;
    let mut missing = 0usize;
    let mut errors = 0usize;

    for img in &images {
        match has_exif_date(img) {
            Ok(true) => {
                println!("[OK]      {}", img.display());
                ok += 1;
            }
            Ok(false) => {
                println!("[MISSING] {}", img.display());
                missing += 1;
            }
            Err(e) => {
                println!("[ERROR]   {} - {:#}", img.display(), e);
                errors += 1;
            }
        }
    }

    println!();
    println!(
        "Checked {} image(s): {} ok, {} missing, {} errors",
        images.len(),
        ok,
        missing,
        errors
    );
    Ok(())
}

fn missing_dates(path: PathBuf) -> anyhow::Result<()> {
    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", path.display()));
    }

    let mut images = collect_images(&path);
    images.sort();

    let mut errors = 0usize;
    for image in &images {
        match has_exif_date(image) {
            Ok(false) => println!("{}", image.display()),
            Ok(true) => {}
            Err(error) => {
                eprintln!("[ERROR] {} - {:#}", image.display(), error);
                errors += 1;
            }
        }
    }

    if errors > 0 {
        return Err(anyhow::anyhow!(
            "Failed to read EXIF metadata from {} image(s)",
            errors
        ));
    }

    Ok(())
}

fn set(
    path: PathBuf,
    force: bool,
    dry_run: bool,
    date_source: DateSource,
) -> anyhow::Result<()> {
    let images = collect_images(&path);
    if images.is_empty() {
        println!("No images found at {}", path.display());
        return Ok(());
    }

    let mut skipped = 0usize;
    let mut set = 0usize;
    let mut no_source = 0usize;
    let mut errors = 0usize;

    for img in &images {
        let has_date = match has_exif_date(img) {
            Ok(v) => v,
            Err(e) => {
                println!("[ERROR]   {} - {}", img.display(), e);
                errors += 1;
                continue;
            }
        };

        if has_date && !force {
            println!("[SKIP]    {} (already has EXIF date)", img.display());
            skipped += 1;
            continue;
        }

        let Some(datetime) = datetime_from_source(img, date_source) else {
            let reason = match date_source {
                DateSource::Metadata => "could not read file modification time",
                DateSource::Filename => "filename does not contain a complete date and time",
            };
            println!("[NO DATE] {} ({})", img.display(), reason);
            no_source += 1;
            continue;
        };

        let exif_str = format_exif_datetime(&datetime);

        if dry_run {
            println!("[DRY RUN] {} -> {}", img.display(), exif_str);
            set += 1;
            continue;
        }

        match write_exif_date(img, &exif_str) {
            Ok(_) => {
                println!("[SET]     {} -> {}", img.display(), exif_str);
                set += 1;
            }
            Err(e) => {
                println!("[ERROR]   {} - {:#}", img.display(), e);
                errors += 1;
            }
        }
    }

    println!();
    println!(
        "Processed {} image(s): {} set, {} skipped, {} no source, {} errors",
        images.len(),
        set,
        skipped,
        no_source,
        errors
    );
    Ok(())
}

fn collect_images(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        if is_image(path) {
            return vec![path.to_path_buf()];
        }
        return Vec::new();
    }

    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| is_image(p))
        .collect()
}

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| IMAGE_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn is_no_metadata_error(e: &std::io::Error) -> bool {
    e.to_string().contains("No metadata found")
}

fn read_metadata(path: &Path) -> anyhow::Result<Metadata> {
    match Metadata::new_from_path(path) {
        Ok(m) => Ok(m),
        Err(e) if is_no_metadata_error(&e) => Ok(Metadata::new()),
        Err(e) => Err(e).with_context(|| format!("read metadata from {}", path.display())),
    }
}

fn has_exif_date(path: &Path) -> anyhow::Result<bool> {
    let metadata = match read_metadata(path) {
        Ok(metadata) => metadata,
        Err(primary_error) => {
            return has_exif_date_fallback(path).with_context(|| {
                format!(
                    "little_exif failed ({:#}); fallback EXIF parser also failed",
                    primary_error
                )
            });
        }
    };

    let has_original = metadata
        .get_tag(&ExifTag::DateTimeOriginal(String::new()))
        .next()
        .is_some();
    if has_original {
        return Ok(true);
    }

    let has_datetime = metadata
        .get_tag(&ExifTag::ModifyDate(String::new()))
        .next()
        .is_some();
    Ok(has_datetime)
}

fn has_exif_date_fallback(path: &Path) -> anyhow::Result<bool> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("open image {}", path.display()))?;
    let mut reader = std::io::BufReader::new(file);
    let exif = match exif::Reader::new().read_from_container(&mut reader) {
        Ok(exif) => exif,
        Err(exif::Error::NotFound(_)) => return Ok(false),
        Err(error) => {
            return Err(error).with_context(|| format!("read EXIF from {}", path.display()));
        }
    };

    Ok(exif.fields().any(|field| {
        field.tag == exif::Tag::DateTimeOriginal || field.tag == exif::Tag::DateTime
    }))
}

fn write_exif_date(path: &Path, datetime: &str) -> anyhow::Result<()> {
    let original = std::fs::read(path)
        .with_context(|| format!("read original image {}", path.display()))?;
    let embedded_xmp = sync_rrdata::extract_xmp_packet(&original);

    let mut metadata = read_metadata(path)?;
    metadata.set_tag(ExifTag::DateTimeOriginal(datetime.to_string()));
    let write_result = metadata
        .write_to_file(path)
        .with_context(|| format!("write metadata to {}", path.display()))
        .and_then(|_| {
            let Some(packet) = embedded_xmp else {
                return Ok(());
            };
            let current = std::fs::read(path)
                .with_context(|| format!("read updated image {}", path.display()))?;
            let packet = String::from_utf8(packet)
                .context("embedded XMP packet is not valid UTF-8")?;
            let restored = sync_rrdata::embed_xmp_in_jpeg(&current, &packet)?;
            std::fs::write(path, restored)
                .with_context(|| format!("restore embedded XMP to {}", path.display()))
        });

    if let Err(error) = write_result {
        std::fs::write(path, original)
            .with_context(|| format!("restore original image {}", path.display()))?;
        return Err(error).context("EXIF update failed; original image was restored");
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct DateTimeParts {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl DateTimeParts {
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<Self> {
        let month_value = time::Month::try_from(u8::try_from(month).ok()?).ok()?;
        time::Date::from_calendar_date(year, month_value, u8::try_from(day).ok()?).ok()?;
        time::Time::from_hms(
            u8::try_from(hour).ok()?,
            u8::try_from(minute).ok()?,
            u8::try_from(second).ok()?,
        )
        .ok()?;

        Some(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

fn datetime_from_modified_time(path: &Path) -> Option<DateTimeParts> {
    let metadata = std::fs::metadata(path).ok()?;
    let modified = metadata.modified().ok()?;
    let duration = modified.duration_since(std::time::UNIX_EPOCH).ok()?;
    let secs = duration.as_secs() as i64;
    let datetime = time::OffsetDateTime::from_unix_timestamp(secs).ok()?;
    let datetime = time::UtcOffset::current_local_offset()
        .map(|offset| datetime.to_offset(offset))
        .unwrap_or(datetime);
    Some(DateTimeParts {
        year: datetime.year(),
        month: u8::from(datetime.month()) as u32,
        day: datetime.day() as u32,
        hour: datetime.hour() as u32,
        minute: datetime.minute() as u32,
        second: datetime.second() as u32,
    })
}

fn datetime_from_source(path: &Path, source: DateSource) -> Option<DateTimeParts> {
    match source {
        DateSource::Metadata => datetime_from_modified_time(path),
        DateSource::Filename => path
            .file_stem()
            .and_then(|name| name.to_str())
            .and_then(datetime_from_filename),
    }
}

fn datetime_from_filename(name: &str) -> Option<DateTimeParts> {
    static PATTERNS: std::sync::LazyLock<[Regex; 2]> = std::sync::LazyLock::new(|| {
        [
            Regex::new(r"(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})").unwrap(),
            Regex::new(r"(\d{4})-(\d{2})-(\d{2})[ _](\d{2})[-:]?(\d{2})[-:]?(\d{2})")
                .unwrap(),
        ]
    });

    for pattern in PATTERNS.iter() {
        let Some(captures) = pattern.captures(name) else {
            continue;
        };
        return DateTimeParts::new(
            captures.get(1)?.as_str().parse().ok()?,
            captures.get(2)?.as_str().parse().ok()?,
            captures.get(3)?.as_str().parse().ok()?,
            captures.get(4)?.as_str().parse().ok()?,
            captures.get(5)?.as_str().parse().ok()?,
            captures.get(6)?.as_str().parse().ok()?,
        );
    }

    None
}

fn format_exif_datetime(dt: &DateTimeParts) -> String {
    format!(
        "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
    )
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands, DateSource, datetime_from_filename, truncate_text};
    use clap::Parser;

    #[test]
    fn truncates_text_by_character_count() {
        assert_eq!(truncate_text("abcdef", 4), "abcd...");
        assert_eq!(truncate_text("月食照片", 2), "月食...");
        assert_eq!(truncate_text("short", 10), "short");
    }

    #[test]
    fn parses_missing_dates_subcommand() {
        let cli = Cli::try_parse_from(["exif-tool", "missing-dates", "gallery"]).unwrap();
        assert!(matches!(
            cli.command,
            Commands::MissingDates { path } if path.to_string_lossy() == "gallery"
        ));
    }

    #[test]
    fn parses_fix_dry_run_subcommand() {
        let cli = Cli::try_parse_from(["exif-tool", "fix", "image.jpg", "--dry-run"])
            .unwrap();
        assert!(matches!(
            cli.command,
            Commands::Fix { path, dry_run: true }
                if path.to_string_lossy() == "image.jpg"
        ));
    }

    #[test]
    fn set_date_source_defaults_to_metadata_and_accepts_filename() {
        let default_cli = Cli::try_parse_from(["exif-tool", "set", "gallery"]).unwrap();
        assert!(matches!(
            default_cli.command,
            Commands::Set {
                date_source: DateSource::Metadata,
                ..
            }
        ));

        let filename_cli = Cli::try_parse_from([
            "exif-tool",
            "set",
            "gallery",
            "--date-source",
            "filename",
        ])
        .unwrap();
        assert!(matches!(
            filename_cli.command,
            Commands::Set {
                date_source: DateSource::Filename,
                ..
            }
        ));
    }

    #[test]
    fn parses_supported_filename_datetimes() {
        let compact = datetime_from_filename("IMG_20251027_090722_gallery").unwrap();
        assert_eq!(
            (compact.year, compact.month, compact.day, compact.hour, compact.minute, compact.second),
            (2025, 10, 27, 9, 7, 22)
        );

        let separated = datetime_from_filename("photo 2025-10-27 09-07-22").unwrap();
        assert_eq!(
            (
                separated.year,
                separated.month,
                separated.day,
                separated.hour,
                separated.minute,
                separated.second,
            ),
            (2025, 10, 27, 9, 7, 22)
        );
        assert!(datetime_from_filename("photo_20259999_999999").is_none());
    }
}
