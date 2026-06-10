mod app;
use eframe::egui;

fn main() -> eframe::Result {
    let viewport_options = egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 720.0])
        .with_min_inner_size([640.0, 360.0]);
    let options = eframe::NativeOptions{
        viewport: viewport_options,
        ..Default::default()
    };
    eframe::run_native(
        "Application",
        options,
        Box::new(|_cc| Ok(Box::new(app::app::Application::init()))),
    )
}
