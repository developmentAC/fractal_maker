//! Shared types for fractal rendering and UI state.
//! Includes fractal view rectangle, palette and fractal type enums, and favorite settings for export/import.

use serde::{Serialize, Deserialize};

/// A rectangle in the complex plane representing the visible fractal region.
/// Used for both Mandelbrot and Julia sets.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ViewRect {
	/// Minimum real value (left edge)
	pub min_x: f64,
	/// Maximum real value (right edge)
	pub max_x: f64,
	/// Minimum imaginary value (bottom edge)
	pub min_y: f64,
	/// Maximum imaginary value (top edge)
	pub max_y: f64,
}

/// Color palette options for fractal rendering.
/// UserDefined allows a custom two-color gradient.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Palette {
	/// Blue-to-magenta classic palette
	Classic,
	/// Red/yellow/black fire palette
	Fire,
	/// Blue/teal ocean palette
	Ocean,
	/// Green forest palette
	Forest,
	/// Rainbow palette
	Rainbow,
	/// Pastel colors
	Pastel,
	/// Orange/purple sunset palette
	Sunset,
	/// Blue/white ice palette
	Ice,
	/// Neon green/pink palette
	Neon,
	/// Grayscale
	Grayscale,
	/// User-defined two-color gradient
	UserDefined,
}

/// Which fractal to render: Mandelbrot or Julia set.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractalType {
    Mandelbrot,
    Julia,
}

/// A favorite fractal view and settings, for export/import as JSON.
#[derive(Serialize, Deserialize)]
pub struct FavoriteSetting {
	/// The visible region in the complex plane
	pub view: ViewRect,
	/// The selected color palette
	pub palette: Palette,
	/// Mandelbrot or Julia
	pub fractal_type: FractalType,
	/// Julia set parameter (only used if fractal_type == Julia)
	pub julia_param: (f64, f64),
}

/// List of built-in palette names and variants for the UI dropdown.
/// The string is the label shown in the UI, the Palette is the enum variant.
pub const PALETTE_NAMES: &[(&str, Palette)] = &[
    ("Classic", Palette::Classic),
    ("Fire", Palette::Fire),
    ("Ocean", Palette::Ocean),
    ("Forest", Palette::Forest),
    ("Rainbow", Palette::Rainbow),
    ("Pastel", Palette::Pastel),
    ("Sunset", Palette::Sunset),
    ("Ice", Palette::Ice),
    ("Neon", Palette::Neon),
    ("Grayscale", Palette::Grayscale),
    ("User Defined", Palette::UserDefined),
];
