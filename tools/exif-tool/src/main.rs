use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
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

#[derive(Subcommand)]
enum Commands {
    /// Check whether images have an EXIF date tag.
    Check {
        /// Path to an image file or a directory to scan recursively.
        path: PathBuf,
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
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Check { path } => check(path),
        Commands::Set { path, force, dry_run } => set(path, force, dry_run),
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

fn set(path: PathBuf, force: bool, dry_run: bool) -> anyhow::Result<()> {
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

        let Some(datetime) = infer_datetime(img) else {
            println!("[NO DATE] {} (could not infer date from filename or filesystem)", img.display());
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
    let metadata = read_metadata(path)?;

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

fn write_exif_date(path: &Path, datetime: &str) -> anyhow::Result<()> {
    let mut metadata = read_metadata(path)?;
    metadata.set_tag(ExifTag::DateTimeOriginal(datetime.to_string()));
    metadata
        .write_to_file(path)
        .with_context(|| format!("write metadata to {}", path.display()))?;
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

fn infer_datetime(path: &Path) -> Option<DateTimeParts> {
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    datetime_from_filename(file_name).or_else(|| file_created_datetime(path))
}

fn datetime_from_filename(name: &str) -> Option<DateTimeParts> {
    let patterns = [
        Regex::new(r"(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})").unwrap(),
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})[ _](\d{2})[-:]?(\d{2})[-:]?(\d{2})").unwrap(),
    ];

    for pattern in &patterns {
        if let Some(caps) = pattern.captures(name) {
            let year: i32 = caps.get(1)?.as_str().parse().ok()?;
            let month: u32 = caps.get(2)?.as_str().parse().ok()?;
            let day: u32 = caps.get(3)?.as_str().parse().ok()?;
            let hour: u32 = caps.get(4)?.as_str().parse().ok()?;
            let minute: u32 = caps.get(5)?.as_str().parse().ok()?;
            let second: u32 = caps.get(6)?.as_str().parse().ok()?;
            return Some(DateTimeParts {
                year,
                month,
                day,
                hour,
                minute,
                second,
            });
        }
    }
    None
}

fn file_created_datetime(path: &Path) -> Option<DateTimeParts> {
    let metadata = std::fs::metadata(path).ok()?;
    let created = metadata.created().ok().or_else(|| metadata.modified().ok())?;
    let duration = created.duration_since(std::time::UNIX_EPOCH).ok()?;
    let secs = duration.as_secs() as i64;
    let datetime = time::OffsetDateTime::from_unix_timestamp(secs).ok()?;
    Some(DateTimeParts {
        year: datetime.year(),
        month: u8::from(datetime.month()) as u32,
        day: datetime.day() as u32,
        hour: datetime.hour() as u32,
        minute: datetime.minute() as u32,
        second: datetime.second() as u32,
    })
}

fn format_exif_datetime(dt: &DateTimeParts) -> String {
    format!(
        "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
    )
}
