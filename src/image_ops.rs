use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use image::ImageFormat;
use rand::seq::SliceRandom;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct FolderSource {
    pub path: PathBuf,
    pub include_subfolders: bool,
}

pub fn collect_images(
    folders: &[FolderSource],
    single_image: Option<&Path>,
) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();
    for folder in folders {
        let mut walker = WalkDir::new(&folder.path).follow_links(true);
        if !folder.include_subfolders {
            walker = walker.max_depth(1);
        }
        for entry in walker {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            if is_supported_image(entry.path()) {
                images.push(entry.path().to_path_buf());
            }
        }
    }

    if let Some(single) = single_image {
        if is_supported_image(single) {
            images.push(single.to_path_buf());
        } else {
            return Err(anyhow!(
                "selected file is not a supported image: {}",
                single.display()
            ));
        }
    }

    images.sort();
    images.dedup();
    Ok(images)
}

pub fn is_supported_image(path: &Path) -> bool {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "bmp" | "gif" | "tif" | "tiff" | "webp"
        ),
        None => false,
    }
}

pub fn pick_random(images: &[PathBuf], last: Option<&PathBuf>) -> Result<PathBuf> {
    let mut rng = rand::thread_rng();
    if images.len() == 1 {
        return Ok(images[0].clone());
    }
    for _ in 0..5 {
        let candidate = images
            .choose(&mut rng)
            .ok_or_else(|| anyhow!("no images available"))?;
        if Some(candidate) != last {
            return Ok(candidate.clone());
        }
    }
    Ok(images
        .choose(&mut rng)
        .ok_or_else(|| anyhow!("no images available"))?
        .clone())
}

pub fn process_image(path: &Path, auto_rotate: bool) -> Result<PathBuf> {
    let mut img = image::open(path)
        .with_context(|| format!("failed to open {}", path.display()))?;
    if auto_rotate && img.width() < img.height() {
        img = img.rotate90();
    }

    let cache_path = cache_file_path()?;
    // Windows wallpaper APIs are most reliable with BMP input.
    let rgb = img.to_rgb8();
    rgb.save_with_format(&cache_path, ImageFormat::Bmp)
        .with_context(|| format!("failed to write {}", cache_path.display()))?;
    Ok(cache_path)
}

fn cache_file_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "wallpaper_manager", "wallpaper_manager")
        .ok_or_else(|| anyhow!("cannot determine cache directory"))?;
    let cache_dir = dirs.cache_dir();
    std::fs::create_dir_all(cache_dir)?;
    Ok(cache_dir.join("current.bmp"))
}
