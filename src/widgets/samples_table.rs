use egui::{Color32, RichText, Widget};
use music_note::midi::MidiNote;

use crate::Keygroup;

pub struct SamplesTable<'a> {
    pub keygroups: &'a mut Vec<Keygroup>,
}

impl<'a> SamplesTable<'a> {
    pub fn new(keygroups: &'a mut Vec<Keygroup>) -> Self {
        Self { keygroups }
    }
}

impl<'a> Widget for SamplesTable<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                egui::Grid::new("sample_grid")
                    .num_columns(1)
                    .striped(true)
                    .show(ui, |ui| {
                        let mut delete_index = None;

                        // Header
                        let mut resp = ui
                            .heading("Range")
                            .union(ui.heading("Layer 1"))
                            .union(ui.heading("")) // Layer 1 range
                            .union(ui.heading("")); // Layer 1 delete
                        ui.end_row();

                        // Grid
                        for (index, keygroup) in self.keygroups.iter_mut().enumerate() {
                            let color = keygroup.color();

                            // Range
                            let r = ui.horizontal(|ui| match &keygroup.range {
                                Some(range) => ui.label(format!(
                                    "{}{} to {}{}",
                                    range.low.pitch(),
                                    range.low.octave(),
                                    range.high.pitch(),
                                    range.high.octave(),
                                )),
                                None => ui.label("⚠ ???").on_hover_text("Unknown range."),
                            });
                            resp = resp.union(r.response);

                            // Layer 1
                            let sample_text = keygroup.layers[0].file.clone();
                            let r = if keygroup.layers[0].file.ends_with(".wav")
                                || keygroup.layers[0].file.ends_with(".WAV")
                            {
                                ui.label(RichText::new(format!("🎵 {}", sample_text)).color(color))
                            } else {
                                ui.label(format!("⚠ {}", sample_text))
                                    .on_hover_text("Programs should be done from .wav samples.")
                            };
                            resp = resp.union(r);

                            // Layer 1 Root Note
                            let root_note_text = match &keygroup.layers[0].root {
                                Some(root) => {
                                    format!("🎵 {}{}", root.pitch(), root.octave(),)
                                }
                                None => "⚠ ???".to_string(),
                            };
                            ui.menu_button(root_note_text, |ui| {
                                for octave in crate::OCTAVES {
                                    ui.menu_button(format!("Octave {}", octave), |ui| {
                                        for pitch in crate::PITCHES {
                                            if ui.button(format!("{}{}", pitch, octave)).clicked() {
                                                let root = MidiNote::new(pitch, octave);
                                                keygroup.layers[0].root = Some(root);
                                                resp.mark_changed();
                                                ui.close_menu();
                                            }
                                        }
                                    });
                                }
                            });

                            // Layer 1 Delete Button
                            if ui
                                .button(RichText::new("❌").color(Color32::GRAY))
                                .clicked()
                            {
                                resp.mark_changed();
                                delete_index = Some(index);
                            }

                            ui.end_row();
                        }

                        if let Some(delete_index) = delete_index {
                            self.keygroups.remove(delete_index);
                        }

                        resp
                    })
                    .inner
            })
            .inner
    }
}
