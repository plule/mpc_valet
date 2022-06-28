use egui::Widget;

use crate::LayerVelocityMode;

pub struct LayeringModeSelector<'a> {
    pub layer_mode: &'a mut LayerVelocityMode,
    pub layer_count: usize,
}

impl<'a> LayeringModeSelector<'a> {
    pub fn new(layer_mode: &'a mut LayerVelocityMode, layer_count: usize) -> Self {
        Self {
            layer_mode,
            layer_count,
        }
    }
}

impl<'a> Widget for LayeringModeSelector<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let inner = ui.horizontal(|ui| {
            if self.layer_count < 2 {
                return None;
            }
            ui.label("Velocity Layering Mode:");
            let r1 = ui
                .selectable_value(
                    self.layer_mode,
                    LayerVelocityMode::Overlapping,
                    "Overlapping",
                )
                .on_hover_text("All the samples are always triggered.");
            let r2 = ui
                .selectable_value(self.layer_mode, LayerVelocityMode::Spread, "Spread")
                .on_hover_text("Each sample is only triggered for a certain velocity range.");
            Some(r1.union(r2))
        });
        inner.inner.unwrap_or(inner.response)
    }
}
