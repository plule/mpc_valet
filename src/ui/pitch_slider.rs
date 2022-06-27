use egui::Widget;

pub struct PitchSlider<'a> {
    pub pitch_preference: &'a mut f32,
}

impl<'a> PitchSlider<'a> {
    pub fn new(pitch_preference: &'a mut f32) -> Self {
        Self { pitch_preference }
    }
}

impl<'a> Widget for PitchSlider<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("pitch down");
            let response = ui.add(
                egui::Slider::new(self.pitch_preference, 0.0..=1.0)
                    .clamp_to_range(true)
                    .show_value(false),
            );
            ui.label("pitch up");
            response
        })
        .inner
    }
}
