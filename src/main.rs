use eframe::egui;

struct Application {
    // Empty
}impl Application{
    fn init() -> Self{
        Application{
            // Empty
        }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top-panel").show_inside(ui, |ui|{
            ui.menu_button("place-holder", |ui|{
                let _ = ui.button("place-holder1");
                ui.separator();
                let _ = ui.button("place-holder2");
            });
        });
        egui::Panel::bottom("btm-panel").show_inside(ui, |ui|{
            ui.label("place-holder");
        });
        egui::Panel::left("left-panel").resizable(true).size_range(100.0..=400.0).show_inside(ui, |ui|{
            ui.label("place-holder");
            ui.take_available_space()
        });
        egui::Panel::right("right-panel").resizable(true).size_range(100.0..=400.0).show_inside(ui, |ui|{
            ui.label("place-holder");
            ui.take_available_space()
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("place-holder");
        });
    }
}

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
        Box::new(|_cc| Ok(Box::new(Application::init()))),
    )
}
