//! Main application logic and UI for the fractal visualizer.
//! Handles user interaction, rendering, and state management.

use crate::types::{ViewRect, Palette, FractalType, FavoriteSetting, PALETTE_NAMES};
// palette_color is not used directly here
use crate::fractal::{render_mandelbrot, render_julia};
use crate::save::{save_fractal_serialized, export_favorite, import_favorite};
use std::sync::{Arc, Mutex};
use eframe::egui;

/// The main application struct, holding all state for the fractal visualizer UI.
pub struct FractalApp {
	/// Handle to the current fractal image texture (for display)
	pub texture_handle: Option<egui::TextureHandle>,
	/// Current image width in pixels
	pub width: usize,
	/// Current image height in pixels
	pub height: usize,
	/// Current view rectangle in the complex plane
	pub view: ViewRect,
	/// Is the user currently dragging to select a zoom region?
	pub dragging: bool,
	/// Drag start position (if dragging)
	pub drag_start: Option<egui::Pos2>,
	/// Drag end position (if dragging)
	pub drag_end: Option<egui::Pos2>,
	/// Selected color palette
	pub palette: Palette,
	/// Last palette used (for detecting changes)
	pub last_palette: Palette,
	/// Message to display after save/export actions
	pub save_message: Option<String>,
	/// Which fractal to render (Mandelbrot or Julia)
	pub fractal_type: FractalType,
	/// Julia set parameter (re, im)
	pub julia_param: (f64, f64),
	/// Is a high-res save in progress?
	pub highres_in_progress: bool,
	// pub highres_progress: f32, // unused
	/// Result of the high-res save thread (shared via Arc<Mutex<..>>)
	pub highres_result: Arc<Mutex<Option<Result<String, String>>>>,
	/// User-defined palette colors (for gradient)
	pub user_palette: [(u8, u8, u8); 2],
	/// Should the import favorite dialog be shown?
	pub show_import_dialog: bool,
}

impl FractalApp {
	/// Helper: List all favorite JSON files in the 0_fractals/ directory.
	fn list_favorite_files() -> Vec<String> {
		let dir = "0_fractals";
		let mut files = Vec::new();
		if let Ok(entries) = std::fs::read_dir(dir) {
			for entry in entries.flatten() {
				let path = entry.path();
				if let Some(ext) = path.extension() {
					if ext == "json" {
						if let Some(path_str) = path.to_str() {
							files.push(path_str.to_string());
						}
					}
				}
			}
			files.sort();
		}
		files
	}

	/// Show a popup dialog to select a favorite JSON file to import.
	pub fn show_import_favorite_dialog(&mut self, ctx: &egui::Context) {
		let mut open = true;
		egui::Window::new("Import Favorite")
			.collapsible(false)
			.open(&mut open)
			.show(ctx, |ui| {
				let files = Self::list_favorite_files();
				if files.is_empty() {
					ui.label("No favorite JSON files found in 0_fractals/");
				} else {
					for file in files {
						if ui.button(&file).clicked() {
							match self.import_favorite(&file, ctx) {
								Ok(()) => self.save_message = Some(format!("Imported favorite from {}", file)),
								Err(e) => self.save_message = Some(format!("Failed to import: {e}")),
							}
							self.show_import_dialog = false;
						}
					}
				}
				if ui.button("Cancel").clicked() {
					self.show_import_dialog = false;
				}
			});
	}
	/// Create a new FractalApp with default view and palette.
	pub fn new(ctx: &egui::Context) -> Self {
		let width = 800;
		let height = 600;
		// Default Mandelbrot view
		let view = ViewRect {
			min_x: -2.5,
			max_x: 1.0,
			min_y: -1.0,
			max_y: 1.0,
		};
		let palette = Palette::Classic;
		// Render initial Mandelbrot image
		let pixels = render_mandelbrot(width, height, view, palette, &[(0, 255, 255), (255, 0, 255)]);
		let color_image = egui::ColorImage::from_rgb([width, height], &pixels);
		let texture_handle = Some(ctx.load_texture(
			"mandelbrot",
			color_image,
			egui::TextureOptions::default(),
		));
		Self {
			texture_handle,
			width,
			height,
			view,
			dragging: false,
			drag_start: None,
			drag_end: None,
			palette,
			last_palette: palette,
			save_message: None,
			fractal_type: FractalType::Mandelbrot,
			julia_param: (-0.8, 0.156),
			highres_in_progress: false,
			// highres_progress: 0.0, // removed
			highres_result: Arc::new(Mutex::new(None)),
			user_palette: [(0, 255, 255), (255, 0, 255)],
			show_import_dialog: false,
		}
	}

	/// Rerender the fractal image and update the texture.
	pub fn rerender(&mut self, ctx: &egui::Context) {
		let pixels = match self.fractal_type {
			FractalType::Mandelbrot => render_mandelbrot(self.width, self.height, self.view, self.palette, &self.user_palette),
			FractalType::Julia => render_julia(self.width, self.height, self.view, self.palette, &self.user_palette, self.julia_param),
		};
		let color_image = egui::ColorImage::from_rgb([self.width, self.height], &pixels);
		self.texture_handle = Some(ctx.load_texture(
			"mandelbrot",
			color_image,
			egui::TextureOptions::default(),
		));
	}

	/// Export the current view and settings as a favorite (JSON file).
	pub fn export_favorite(&self) -> Result<String, String> {
		let fav = FavoriteSetting {
			view: self.view,
			palette: self.palette,
			fractal_type: self.fractal_type,
			julia_param: self.julia_param,
		};
		export_favorite(&fav)
	}

	/// Import a favorite view and settings from a JSON file.
	pub fn import_favorite(&mut self, path: &str, ctx: &egui::Context) -> Result<(), String> {
		let fav = import_favorite(path)?;
		self.view = fav.view;
		self.palette = fav.palette;
		self.fractal_type = fav.fractal_type;
		self.julia_param = fav.julia_param;
		self.rerender(ctx);
		Ok(())
	}
}

impl eframe::App for FractalApp {
	/// Main update loop for the egui application.
	/// Handles all UI controls, rendering, and user interaction.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
	// show_import_dialog is now a struct field
		egui::CentralPanel::default().show(ctx, |ui| {
			// Layout: vertical stack, with a horizontal toolbar for controls
			ui.vertical(|ui| {
				// Toolbar: palette, fractal type, Julia params, buttons
				ui.horizontal(|ui| {
					egui::ComboBox::from_label("")
						.selected_text(
							PALETTE_NAMES.iter().find(|(_, p)| *p == self.palette).map(|(n, _)| *n).unwrap_or("Unknown")
						)
						.show_ui(ui, |cb| {
							for (name, pal) in PALETTE_NAMES {
								cb.selectable_value(&mut self.palette, *pal, *name);
							}
						});

					ui.label("Fractal:");
					ui.selectable_value(&mut self.fractal_type, FractalType::Mandelbrot, "Mandelbrot");
					ui.selectable_value(&mut self.fractal_type, FractalType::Julia, "Julia");

					if self.fractal_type == FractalType::Julia {
						ui.label("c (re, im):");
						let mut re = self.julia_param.0;
						let mut im = self.julia_param.1;
						if ui.add(egui::DragValue::new(&mut re).speed(0.01)).changed() ||
						   ui.add(egui::DragValue::new(&mut im).speed(0.01)).changed() {
							self.julia_param = (re, im);
							self.rerender(ctx);
						}
					}

					if ui.button("Reset View").clicked() {
						self.view = ViewRect {
							min_x: -2.5,
							max_x: 1.0,
							min_y: -1.0,
							max_y: 1.0,
						};
						self.rerender(ctx);
					}
					if ui.button("Zoom Out").clicked() {
						let cx = (self.view.min_x + self.view.max_x) / 2.0;
						let cy = (self.view.min_y + self.view.max_y) / 2.0;
						let scale_x = (self.view.max_x - self.view.min_x) * 2.0;
						let scale_y = (self.view.max_y - self.view.min_y) * 2.0;
						self.view = ViewRect {
							min_x: cx - scale_x / 2.0,
							max_x: cx + scale_x / 2.0,
							min_y: cy - scale_y / 2.0,
							max_y: cy + scale_y / 2.0,
						};
						self.rerender(ctx);
					}
					if ui.button("Redraw").clicked() || self.palette != self.last_palette {
						self.last_palette = self.palette;
						self.rerender(ctx);
					}

					if ui.button("Save PNG").clicked() {
						match save_fractal_serialized(self.width, self.height, self.view, self.palette, &self.user_palette, self.fractal_type, self.julia_param, false) {
							Ok(path) => self.save_message = Some(format!("Saved as {}", path)),
							Err(e) => self.save_message = Some(format!("Failed to save: {e}")),
						}
					}
					if self.highres_in_progress {
						ui.add(egui::Spinner::new());
						ui.label("Rendering high-res...");
						match self.highres_result.lock() {
							Ok(mut lock) => {
								if let Some(res) = lock.take() {
									self.highres_in_progress = false;
									match res {
										Ok(path) => self.save_message = Some(format!("Saved as {}", path)),
										Err(e) => self.save_message = Some(format!("Failed to save: {e}")),
									}
								}
							}
							Err(_) => {
								self.highres_in_progress = false;
								self.save_message = Some("Error: Save thread panicked (mutex poisoned)".to_string());
							}
						}
					} else if ui.button("Save High-Res PNG").clicked() {
						self.highres_in_progress = true;
						self.save_message = None;
						let width = 3200;
						let height = 2400;
						let view = self.view;
						let palette = self.palette;
						let fractal_type = self.fractal_type;
						let julia_param = self.julia_param;
						let user_palette = self.user_palette;
						let result_arc = self.highres_result.clone();
						std::thread::spawn(move || {
							let result = save_fractal_serialized(width, height, view, palette, &user_palette, fractal_type, julia_param, true);
							if let Ok(mut lock) = result_arc.lock() {
								*lock = Some(result);
							}
						});
					}

					if ui.button("Export Favorite").clicked() {
						match self.export_favorite() {
							Ok(path) => self.save_message = Some(format!("Favorite saved as {}", path)),
							Err(e) => self.save_message = Some(format!("Failed to export: {e}")),
						}
					}
					if ui.button("Import Favorite").clicked() {
						self.show_import_dialog = true;
					}
				});

				if self.palette == Palette::UserDefined {
					ui.horizontal(|ui| {
						ui.label("User Palette: Pick two colors for the gradient");
						let mut color1 = [self.user_palette[0].0 as f32 / 255.0, self.user_palette[0].1 as f32 / 255.0, self.user_palette[0].2 as f32 / 255.0];
						let mut color2 = [self.user_palette[1].0 as f32 / 255.0, self.user_palette[1].1 as f32 / 255.0, self.user_palette[1].2 as f32 / 255.0];
						let changed1 = ui.color_edit_button_rgb(&mut color1).changed();
						let changed2 = ui.color_edit_button_rgb(&mut color2).changed();
						if changed1 {
							self.user_palette[0] = ((color1[0] * 255.0) as u8, (color1[1] * 255.0) as u8, (color1[2] * 255.0) as u8);
							self.rerender(ctx);
						}
						if changed2 {
							self.user_palette[1] = ((color2[0] * 255.0) as u8, (color2[1] * 255.0) as u8, (color2[2] * 255.0) as u8);
							self.rerender(ctx);
						}
					});
				}
				// Show the import favorites dialog if requested (now global, not just for user palette)
				if self.show_import_dialog {
					self.show_import_favorite_dialog(ctx);
				}

				if let Some(msg) = &self.save_message {
					ui.label(msg);
				}

				let image_size = egui::vec2(self.width as f32, self.height as f32);
				let (rect, response) = ui.allocate_exact_size(image_size, egui::Sense::drag());
				if let Some(texture) = &self.texture_handle {
					ui.painter().image(
						texture.id(),
						rect,
						egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
						egui::Color32::WHITE,
					);
				}

				if response.drag_started() {
					if let Some(pos) = response.interact_pointer_pos() {
						if rect.contains(pos) {
							self.dragging = true;
							self.drag_start = Some(pos);
							self.drag_end = Some(pos);
						}
					}
				}
				if self.dragging {
					if let Some(pos) = response.interact_pointer_pos() {
						self.drag_end = Some(pos);
					}
					if let (Some(start), Some(end)) = (self.drag_start, self.drag_end) {
						let rect_sel = egui::Rect::from_two_pos(start, end);
						ui.painter().rect_stroke(rect_sel, 0.0, (2.0, egui::Color32::YELLOW));
					}
				}
				if response.drag_stopped() && self.dragging {
					self.dragging = false;
					if let (Some(start), Some(end)) = (self.drag_start, self.drag_end) {
						let start = start.clamp(rect.min, rect.max);
						let end = end.clamp(rect.min, rect.max);
						let min = start.min(end);
						let max = start.max(end);
						if (max.x - min.x).abs() > 5.0 && (max.y - min.y).abs() > 5.0 {
							let to_fractal = |pos: egui::Pos2| {
								let x = self.view.min_x + ((pos.x - rect.min.x) / rect.width()) as f64 * (self.view.max_x - self.view.min_x);
								let y = self.view.min_y + ((pos.y - rect.min.y) / rect.height()) as f64 * (self.view.max_y - self.view.min_y);
								(x, y)
							};
							let (min_x, min_y) = to_fractal(min);
							let (max_x, max_y) = to_fractal(max);
							self.view = ViewRect {
								min_x,
								max_x,
								min_y,
								max_y,
							};
							self.rerender(ctx);
						}
					}
					self.drag_start = None;
					self.drag_end = None;
				}
			});
		});
	}
}
