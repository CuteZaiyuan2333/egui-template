use eframe::egui;

pub struct Application {
    // Empty
}impl Application{
    pub fn init() -> Self{
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
