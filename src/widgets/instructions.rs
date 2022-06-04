use egui::Widget;

#[derive(Default)]
pub struct Instructions;

impl Widget for Instructions {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.collapsing("Instructions", |ui| {
            ui.label("Drag and drop samples from instrument notes on this window.");
            ui.label("Make sure the file contain the note in their name, such as \"Violin-C3.wav\" or \"Flute-43.wav\"");
            ui.label("This software will find appropriate note range for each sample.");
            ui.label("When you are done, choose a name and click \"Save\". The XPM file must be saved in the same folder as the samples.");
            ui.label("Open the XPM with the MPC Software, or any of the MPC Live, One, X...");
        }).header_response
    }
}
