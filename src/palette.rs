//! Color palette logic for fractal rendering.
//! Includes built-in palettes and user-defined gradient support.
use crate::types::Palette;

/// Returns the RGB color for a given palette and iteration value.
///
/// * `i` - The iteration count for the pixel (0..=255)
/// * `palette` - The selected palette enum
/// * `user_palette` - The two RGB colors for the user-defined gradient
///
/// Returns [r, g, b] for the pixel color.
pub fn palette_color(i: u32, palette: Palette, user_palette: &[(u8, u8, u8); 2]) -> [u8; 3] {
	// Each palette maps the iteration count to a color.
	// UserDefined uses a linear gradient between two user-chosen colors.
	match palette {
		Palette::Classic => {
			if i < 255 {
				[i as u8, 0, 255 - i as u8]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Fire => {
			if i < 255 {
				[255, (i as f32 * 0.7) as u8, (i as f32 * 0.1) as u8]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Ocean => {
			if i < 255 {
				[0, (i as f32 * 0.5) as u8, (i as f32 * 0.9) as u8]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Forest => {
			if i < 255 {
				[(i as f32 * 0.2) as u8, (i as f32 * 0.8) as u8, (i as f32 * 0.3) as u8]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Rainbow => {
			if i < 255 {
				let t = i as f32 / 255.0;
				let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
				let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
				let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
				[r, g, b]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Pastel => {
			if i < 255 {
				[200, 200u8.saturating_sub(i as u8), 255u8.saturating_sub(i as u8 / 2)]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Sunset => {
			if i < 255 {
				let t = i as f32 / 255.0;
				[
					(255.0 * t) as u8,
					(100.0 * (1.0 - t) + 50.0 * t) as u8,
					(50.0 * (1.0 - t)) as u8,
				]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Ice => {
			if i < 255 {
				let t = i as f32 / 255.0;
				[
					(180.0 * (1.0 - t) + 200.0 * t) as u8,
					(220.0 * t) as u8,
					(255.0 * t) as u8,
				]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Neon => {
			if i < 255 {
				let t = i as f32 / 255.0;
				[
					(255.0 * (1.0 - t)) as u8,
					(255.0 * t) as u8,
					(255.0 * (1.0 - t) * t) as u8,
				]
			} else {
				[0, 0, 0]
			}
		}
		Palette::Grayscale => {
			if i < 255 {
				let v = i as u8;
				[v, v, v]
			} else {
				[0, 0, 0]
			}
		}
		Palette::UserDefined => {
			let (r1, g1, b1) = user_palette[0];
			let (r2, g2, b2) = user_palette[1];
			if i < 255 {
				let t = i as f32 / 255.0;
				[
					(r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8,
					(g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8,
					(b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8,
				]
			} else {
				[0, 0, 0]
			}
		}
	}
}
