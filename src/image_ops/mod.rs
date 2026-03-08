//! Image discovery and processing utilities.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use image::{DynamicImage, ImageFormat, RgbImage};
use rand::seq::SliceRandom;
use walkdir::WalkDir;

use crate::settings::StitchOrientation;

/// Clean up temporary cache files.
pub fn cleanup_temp_files() {
    if let Ok(cache_path) = cache_file_path() {
        let _ = std::fs::remove_file(&cache_path);
    }
    // Also clean any old temp files in cache directory
    if let Some(dirs) = ProjectDirs::from("dev", "wallpaper_manager", "wallpaper_manager") {
        let cache_dir = dirs.cache_dir();
        if let Ok(entries) = std::fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .map_or(false, |ext| ext == "bmp" || ext == "tmp")
                {
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
    }
}

/// Folder input definition used by the UI and settings.
#[derive(Clone, Debug)]
pub struct FolderSource {
    /// Folder path to scan.
    pub path: PathBuf,
    /// Whether to include subfolders when scanning.
    pub include_subfolders: bool,
}

/// Collect all matching images from folders and an optional single file.
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

/// Return true when the file extension is a supported image type.
pub fn is_supported_image(path: &Path) -> bool {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "bmp" | "gif" | "tif" | "tiff" | "webp"
        ),
        None => false,
    }
}

/// Pick a random image, avoiding the previous image when possible.
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

/// Load, optionally rotate, and cache an image as a BMP for Windows.
pub fn process_image(path: &Path, auto_rotate: bool) -> Result<PathBuf> {
    let mut img =
        image::open(path).with_context(|| format!("failed to open {}", path.display()))?;
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

/// Resolve the cached wallpaper path used for reapplying styles.
pub fn cached_wallpaper_path() -> Result<PathBuf> {
    cache_file_path()
}

/// Resolve the cache path used to store the BMP wallpaper.
fn cache_file_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "wallpaper_manager", "wallpaper_manager")
        .ok_or_else(|| anyhow!("cannot determine cache directory"))?;
    let cache_dir = dirs.cache_dir();
    std::fs::create_dir_all(cache_dir)?;
    Ok(cache_dir.join("current.bmp"))
}

/// Stitch multiple images together into a single image.
pub fn stitch_images(
    paths: &[PathBuf],
    auto_rotate: bool,
    orientation: StitchOrientation,
    crop_enabled: bool,
    crop_width: u32,
    crop_height: u32,
) -> Result<PathBuf> {
    if paths.is_empty() {
        return Err(anyhow!("no images to stitch"));
    }
    if paths.len() == 1 {
        return process_image(&paths[0], auto_rotate);
    }

    // Load all images
    let mut images: Vec<DynamicImage> = Vec::with_capacity(paths.len());
    for path in paths {
        let img =
            image::open(path).with_context(|| format!("failed to open {}", path.display()))?;
        images.push(img);
    }

    // Apply smart rotation based on orientation and count
    let rotated_images = apply_smart_rotation(&images, orientation, auto_rotate);

    // Stitch based on layout
    let stitched = stitch_with_layout(&rotated_images, orientation);

    let final_image = if crop_enabled {
        crop_center(&stitched, crop_width, crop_height)
    } else {
        stitched
    };

    let cache_path = cache_file_path()?;
    final_image
        .save_with_format(&cache_path, ImageFormat::Bmp)
        .with_context(|| format!("failed to write {}", cache_path.display()))?;
    Ok(cache_path)
}

/// Target orientation for each image slot.
#[derive(Clone, Copy, PartialEq)]
enum TargetOrientation {
    /// Portrait: width < height
    Vertical,
    /// Landscape: width > height
    Horizontal,
}

/// Apply smart rotation to images based on stitch orientation and count.
fn apply_smart_rotation(
    images: &[DynamicImage],
    orientation: StitchOrientation,
    auto_rotate: bool,
) -> Vec<DynamicImage> {
    if !auto_rotate {
        return images.to_vec();
    }

    let count = images.len();
    let targets = get_rotation_pattern(orientation, count);

    images
        .iter()
        .enumerate()
        .map(|(i, img)| {
            let target = targets
                .get(i)
                .copied()
                .unwrap_or(TargetOrientation::Vertical);
            rotate_to_target(img, target)
        })
        .collect()
}

/// Get the rotation pattern based on orientation and image count.
fn get_rotation_pattern(orientation: StitchOrientation, count: usize) -> Vec<TargetOrientation> {
    use TargetOrientation::{Horizontal as H, Vertical as V};

    match orientation {
        StitchOrientation::Horizontal => match count {
            2 => vec![V, V],
            3 => vec![V, H, V],
            4 => vec![H, H, H, H],    // 2x2 grid of H
            5 => vec![V, V, V, H, H], // 3V top + 2H bottom
            _ => vec![V; count],
        },
        StitchOrientation::Vertical => match count {
            2 => vec![H, H],
            3 => vec![H, H, H],
            4 => vec![V, V, V, V],    // 2x2 grid of V
            5 => vec![H, H, H, V, V], // 3H top + 2V bottom
            _ => vec![H; count],
        },
    }
}

/// Rotate image to match target orientation.
fn rotate_to_target(img: &DynamicImage, target: TargetOrientation) -> DynamicImage {
    let is_portrait = img.width() < img.height();
    let needs_portrait = target == TargetOrientation::Vertical;

    if is_portrait != needs_portrait {
        img.rotate90()
    } else {
        img.clone()
    }
}

/// Stitch images using the appropriate layout.
fn stitch_with_layout(images: &[DynamicImage], orientation: StitchOrientation) -> RgbImage {
    let count = images.len();

    match orientation {
        StitchOrientation::Horizontal => match count {
            4 => stitch_grid(images, 2, 2),     // 2x2 grid
            5 => stitch_two_rows(images, 3, 2), // 3 top + 2 bottom
            _ => stitch_horizontal(images),
        },
        StitchOrientation::Vertical => match count {
            4 => stitch_grid(images, 2, 2),     // 2x2 grid
            5 => stitch_two_rows(images, 3, 2), // 3 top + 2 bottom
            _ => stitch_vertical(images),
        },
    }
}

/// Stitch images in a grid layout (rows x cols).
fn stitch_grid(images: &[DynamicImage], rows: usize, cols: usize) -> RgbImage {
    // Calculate cell dimensions (use max dimensions per cell)
    let mut row_heights = vec![0u32; rows];
    let mut col_widths = vec![0u32; cols];

    for (i, img) in images.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        if row < rows {
            row_heights[row] = row_heights[row].max(img.height());
            col_widths[col] = col_widths[col].max(img.width());
        }
    }

    let total_width: u32 = col_widths.iter().sum();
    let total_height: u32 = row_heights.iter().sum();

    let mut result = RgbImage::new(total_width, total_height);

    for (i, img) in images.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        if row >= rows {
            break;
        }

        let x_offset: u32 = col_widths[..col].iter().sum();
        let y_offset: u32 = row_heights[..row].iter().sum();

        // Center image in its cell
        let cell_width = col_widths[col];
        let cell_height = row_heights[row];
        let img_x = x_offset + (cell_width - img.width()) / 2;
        let img_y = y_offset + (cell_height - img.height()) / 2;

        let rgb = img.to_rgb8();
        for (x, y, pixel) in rgb.enumerate_pixels() {
            let px = img_x + x;
            let py = img_y + y;
            if px < total_width && py < total_height {
                result.put_pixel(px, py, *pixel);
            }
        }
    }

    result
}

/// Stitch images in two rows with different counts.
fn stitch_two_rows(images: &[DynamicImage], top_count: usize, bottom_count: usize) -> RgbImage {
    let (top_images, bottom_images) = images.split_at(top_count.min(images.len()));

    let top_row = stitch_horizontal(top_images);
    let bottom_row = if !bottom_images.is_empty() {
        stitch_horizontal(&bottom_images[..bottom_count.min(bottom_images.len())])
    } else {
        RgbImage::new(0, 0)
    };

    // Stack vertically, centering the narrower row
    let total_width = top_row.width().max(bottom_row.width());
    let total_height = top_row.height() + bottom_row.height();

    let mut result = RgbImage::new(total_width, total_height);

    // Top row centered
    let top_x = (total_width - top_row.width()) / 2;
    for (x, y, pixel) in top_row.enumerate_pixels() {
        result.put_pixel(top_x + x, y, *pixel);
    }

    // Bottom row centered
    let bottom_x = (total_width - bottom_row.width()) / 2;
    let bottom_y = top_row.height();
    for (x, y, pixel) in bottom_row.enumerate_pixels() {
        result.put_pixel(bottom_x + x, bottom_y + y, *pixel);
    }

    result
}

/// Stitch images horizontally (side by side).
fn stitch_horizontal(images: &[DynamicImage]) -> RgbImage {
    let max_height = images.iter().map(|img| img.height()).max().unwrap_or(0);
    let total_width: u32 = images.iter().map(|img| img.width()).sum();

    let mut result = RgbImage::new(total_width, max_height);

    let mut x_offset = 0u32;
    for img in images {
        let rgb = img.to_rgb8();
        let y_offset = (max_height - img.height()) / 2;
        for (x, y, pixel) in rgb.enumerate_pixels() {
            if x_offset + x < total_width && y_offset + y < max_height {
                result.put_pixel(x_offset + x, y_offset + y, *pixel);
            }
        }
        x_offset += img.width();
    }

    result
}

/// Stitch images vertically (top to bottom).
fn stitch_vertical(images: &[DynamicImage]) -> RgbImage {
    let max_width = images.iter().map(|img| img.width()).max().unwrap_or(0);
    let total_height: u32 = images.iter().map(|img| img.height()).sum();

    let mut result = RgbImage::new(max_width, total_height);

    let mut y_offset = 0u32;
    for img in images {
        let rgb = img.to_rgb8();
        let x_offset = (max_width - img.width()) / 2;
        for (x, y, pixel) in rgb.enumerate_pixels() {
            if x_offset + x < max_width && y_offset + y < total_height {
                result.put_pixel(x_offset + x, y_offset + y, *pixel);
            }
        }
        y_offset += img.height();
    }

    result
}

/// Scale and crop an image to exactly fill the target dimensions.
/// First scales the image so one dimension matches the target (cover fit),
/// then crops the excess from the center.
fn crop_center(img: &RgbImage, target_width: u32, target_height: u32) -> RgbImage {
    let src_width = img.width();
    let src_height = img.height();

    if src_width == 0 || src_height == 0 || target_width == 0 || target_height == 0 {
        return img.clone();
    }

    // Calculate scale factors for both dimensions
    let scale_x = target_width as f64 / src_width as f64;
    let scale_y = target_height as f64 / src_height as f64;

    // Use the larger scale factor to ensure the image covers the target area
    let scale = scale_x.max(scale_y);

    let scaled_width = (src_width as f64 * scale).round() as u32;
    let scaled_height = (src_height as f64 * scale).round() as u32;

    // Scale the image using the image crate
    let dynamic_img = DynamicImage::ImageRgb8(img.clone());
    let scaled = dynamic_img.resize_exact(
        scaled_width,
        scaled_height,
        image::imageops::FilterType::Lanczos3,
    );
    let scaled_rgb = scaled.to_rgb8();

    // Crop from center to target dimensions
    let crop_x = (scaled_width.saturating_sub(target_width)) / 2;
    let crop_y = (scaled_height.saturating_sub(target_height)) / 2;

    let final_width = target_width.min(scaled_width);
    let final_height = target_height.min(scaled_height);

    let mut result = RgbImage::new(final_width, final_height);
    for y in 0..final_height {
        for x in 0..final_width {
            let pixel = scaled_rgb.get_pixel(crop_x + x, crop_y + y);
            result.put_pixel(x, y, *pixel);
        }
    }

    result
}
