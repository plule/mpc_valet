use crate::KeygroupProgram;
use egui::{FontId, RichText, Widget};
pub struct SamplesArea<'a> {
    pub program: &'a mut KeygroupProgram,
    pub pitch_preference: &'a mut f32,
    pub current_layer: &'a mut usize,
}

impl<'a> SamplesArea<'a> {
    pub fn new(
        program: &'a mut KeygroupProgram,
        pitch_preference: &'a mut f32,
        current_layer: &'a mut usize,
    ) -> Self {
        Self {
            program,
            pitch_preference,
            current_layer,
        }
    }
}

impl<'a> Widget for SamplesArea<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let resp;
        if self.program.keygroups.is_empty() {
            *self.current_layer = 0;
            resp = ui
                .vertical_centered(|ui| {
                    ui.label(
                        RichText::new("⮊ Drag-and-drop your samples here! ⮈")
                            .font(FontId::proportional(20.0)),
                    )
                })
                .inner;
        } else {
            resp = ui
                .vertical(|ui| {
                    ui.set_max_height(ui.available_height() / 2.0);
                    let mut keygroups = self.program.keygroups.clone();
                    let table = ui.add(crate::ui::SamplesTable::new(
                        &mut keygroups,
                        self.current_layer,
                    ));
                    if table.changed() {
                        self.program
                            .update(*self.current_layer, keygroups, *self.pitch_preference);
                    }
                    table
                })
                .inner;
            if ui
                .add(crate::ui::PitchSlider::new(self.pitch_preference))
                .changed()
            {
                self.program.guess_ranges(*self.pitch_preference);
            }
        }
        resp
    }
}
