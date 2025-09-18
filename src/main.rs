mod app;
mod types;
mod palette;
mod fractal;
mod save;

// Driver Program entry point

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mandelbrot Fractal Visualizer",
        options,
        Box::new(|cc| Box::new(app::FractalApp::new(&cc.egui_ctx))),
    )
}
