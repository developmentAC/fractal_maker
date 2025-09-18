//! Fractal rendering functions for Mandelbrot and Julia sets.
//! Each function returns a flat RGB pixel buffer for the image.

use crate::types::{ViewRect, Palette};
use crate::palette::palette_color;

/// Render the Mandelbrot set for the given view and palette.
///
/// * `width`, `height` - Output image size in pixels
/// * `view` - Complex plane region to render
/// * `palette` - Color palette
/// * `user_palette` - User-defined gradient colors
///
/// Returns a flat RGB buffer (row-major order).
pub fn render_mandelbrot(
	width: usize,
	height: usize,
	view: ViewRect,
	palette: Palette,
	user_palette: &[(u8, u8, u8); 2],
) -> Vec<u8> {
	let mut pixels = vec![0u8; width * height * 3];
	for y in 0..height {
		for x in 0..width {
			// Map pixel to complex plane
			let cx = view.min_x + x as f64 / width as f64 * (view.max_x - view.min_x);
			let cy = view.min_y + y as f64 / height as f64 * (view.max_y - view.min_y);
			let mut zx = 0.0;
			let mut zy = 0.0;
			let mut i = 0;
			// Iterate z = z^2 + c until escape or max iterations
			while zx * zx + zy * zy < 4.0 && i < 255 {
				let tmp = zx * zx - zy * zy + cx;
				zy = 2.0 * zx * zy + cy;
				zx = tmp;
				i += 1;
			}
			let idx = (y * width + x) * 3;
			let color = palette_color(i, palette, user_palette);
			pixels[idx..idx + 3].copy_from_slice(&color);
		}
	}
	pixels
}

/// Render the Julia set for the given view, palette, and parameter c.
///
/// * `width`, `height` - Output image size in pixels
/// * `view` - Complex plane region to render
/// * `palette` - Color palette
/// * `user_palette` - User-defined gradient colors
/// * `c` - Julia set parameter (re, im)
///
/// Returns a flat RGB buffer (row-major order).
pub fn render_julia(
	width: usize,
	height: usize,
	view: ViewRect,
	palette: Palette,
	user_palette: &[(u8, u8, u8); 2],
	c: (f64, f64),
) -> Vec<u8> {
	let mut pixels = vec![0u8; width * height * 3];
	for y in 0..height {
		for x in 0..width {
			// Map pixel to complex plane
			let zx0 = view.min_x + x as f64 / width as f64 * (view.max_x - view.min_x);
			let zy0 = view.min_y + y as f64 / height as f64 * (view.max_y - view.min_y);
			let (cx, cy) = c;
			let mut zx = zx0;
			let mut zy = zy0;
			let mut i = 0;
			// Iterate z = z^2 + c until escape or max iterations
			while zx * zx + zy * zy < 4.0 && i < 255 {
				let tmp = zx * zx - zy * zy + cx;
				zy = 2.0 * zx * zy + cy;
				zx = tmp;
				i += 1;
			}
			let idx = (y * width + x) * 3;
			let color = palette_color(i, palette, user_palette);
			pixels[idx..idx + 3].copy_from_slice(&color);
		}
	}
	pixels
}
