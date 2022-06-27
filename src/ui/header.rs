use egui::Widget;

#[derive(Default)]
pub struct Header;

impl Widget for Header {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered(|ui| {
            ui.heading("MPC Valet");

            for _ in 0..4 {
                ui.heading(egui::RichText::new("▣ ▣ ▣ ▣").text_style(egui::TextStyle::Monospace));
            }
        })
        .response
    }
}
