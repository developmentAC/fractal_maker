//! Image saving and favorite export/import logic for the fractal visualizer.
//! Handles PNG output and JSON serialization of favorite views.

use crate::types::{ViewRect, Palette, FavoriteSetting, FractalType};
use crate::fractal::{render_mandelbrot, render_julia};
use chrono::Local;

/// Save a PNG of the current fractal view in the `0_fractals/` directory with a unique filename.
///
/// # Arguments
/// * `width`, `height` - Output image size in pixels
/// * `view` - Complex plane region to render
/// * `palette` - Color palette
/// * `user_palette` - User-defined gradient colors
/// * `fractal_type` - Mandelbrot or Julia
/// * `julia_param` - Julia set parameter (ignored for Mandelbrot)
/// * `high_res` - If true, filename includes 'highres'
///
/// Returns Ok(path) if successful, or Err(message) on failure.
pub fn save_fractal_serialized(
	width: usize,
	height: usize,
	view: ViewRect,
	palette: Palette,
	user_palette: &[(u8, u8, u8); 2],
	fractal_type: FractalType,
	julia_param: (f64, f64),
	high_res: bool,
) -> Result<String, String> {
	// Ensure the output directory exists
	let dir = "0_fractals";
	if !std::path::Path::new(dir).exists() {
		std::fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {e}"))?;
	}

	// Generate a unique filename with timestamp
	let now = Local::now();
	let ts = now.format("%Y%m%d_%H%M%S");
	let palette_name = match palette {
		Palette::Classic => "classic",
		Palette::Fire => "fire",
		Palette::Ocean => "ocean",
		Palette::Forest => "forest",
		Palette::Rainbow => "rainbow",
		Palette::Pastel => "pastel",
		Palette::Sunset => "sunset",
		Palette::Ice => "ice",
		Palette::Neon => "neon",
		Palette::Grayscale => "grayscale",
		Palette::UserDefined => "userdefined",
	};
	let res = if high_res { "highres" } else { "std" };
	let filename = format!("{}/mandelbrot_{}_{}_{}x{}_{}.png", dir, palette_name, ts, width, height, res);

	// Render and save
	let pixels = match fractal_type {
		FractalType::Mandelbrot => render_mandelbrot(width, height, view, palette, user_palette),
		FractalType::Julia => render_julia(width, height, view, palette, user_palette, julia_param),
	};
	let buffer = image::RgbImage::from_raw(width as u32, height as u32, pixels)
		.ok_or("Failed to create image buffer")?;
	buffer.save(&filename).map_err(|e| e.to_string())?;
	Ok(filename)
}

/// Export the current favorite settings to a JSON file in `0_fractals/`.
/// The file can be imported later to restore the view and palette.
pub fn export_favorite(fav: &FavoriteSetting) -> Result<String, String> {
	let json = serde_json::to_string_pretty(fav).map_err(|e| e.to_string())?;
	let dir = "0_fractals";
	if !std::path::Path::new(dir).exists() {
		std::fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {e}"))?;
	}
	let now = Local::now();
	let ts = now.format("%Y%m%d_%H%M%S");
	let filename = format!("{}/favorite_{}.json", dir, ts);
	std::fs::write(&filename, json).map_err(|e| e.to_string())?;
	Ok(filename)
}

/// Import favorite settings from a JSON file (as written by `export_favorite`).
/// Returns the FavoriteSetting struct on success.
pub fn import_favorite(path: &str) -> Result<FavoriteSetting, String> {
	let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
	let fav: FavoriteSetting = serde_json::from_str(&data).map_err(|e| e.to_string())?;
	Ok(fav)
}
