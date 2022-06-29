use egui::{Color32, RichText, Widget};
use music_note::{
    midi::{MidiNote, Octave},
    Pitch,
};

use crate::{Keygroup, StaticIterable};

use super::Colored;

pub struct SamplesTable<'a> {
    pub keygroups: &'a mut Vec<Keygroup>,
    pub current_layer: &'a mut usize,
}

impl<'a> SamplesTable<'a> {
    pub fn new(keygroups: &'a mut Vec<Keygroup>, current_layer: &'a mut usize) -> Self {
        Self {
            keygroups,
            current_layer,
        }
    }
}

impl<'a> Widget for SamplesTable<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = egui::ScrollArea::vertical()
            .show(ui, |ui| {
                egui::Grid::new("sample_grid")
                    .num_columns(3)
                    .striped(true)
                    .show(ui, |ui| {
                        let mut delete_index = None;

                        // Range
                        let mut resp = ui
                            .horizontal(|ui| {
                                ui.set_min_width(100.0);
                                ui.heading("Range")
                            })
                            .inner;

                        // Sample
                        let r = ui
                            .horizontal(|ui| {
                                ui.set_min_width(200.0);
                                ui.heading("Sample")
                            })
                            .inner;
                        resp = resp.union(r);

                        // Root Note
                        let r = ui.heading("Root Note");
                        resp = resp.union(r);

                        ui.end_row();

                        // Grid
                        for (index, keygroup) in self.keygroups.iter_mut().enumerate() {
                            let color = keygroup.color();

                            // Range
                            let r = ui.horizontal(|ui| match &keygroup.range {
                                Some(range) => ui.label(format!(
                                    "{}{} to {}{}",
                                    range.start().pitch(),
                                    range.start().octave(),
                                    range.end().pitch(),
                                    range.end().octave(),
                                )),
                                None => ui.label("⚠ ???").on_hover_text("Unknown range."),
                            });
                            resp = resp.union(r.response);

                            // Sample
                            ui.horizontal(|ui| {
                                let layer = keygroup.layers[*self.current_layer].as_mut();
                                if let Some(layer) = layer {
                                    // Delete Button
                                    if ui
                                        .button(RichText::new("❌").color(Color32::GRAY))
                                        .clicked()
                                    {
                                        resp.mark_changed();
                                        delete_index = Some(index);
                                    }

                                    let sample_text = layer.file.clone();
                                    let r = if sample_text.ends_with(".wav")
                                        || sample_text.ends_with(".WAV")
                                    {
                                        ui.label(
                                            RichText::new(format!("🎵 {}", sample_text))
                                                .color(color),
                                        )
                                    } else {
                                        ui.label(format!("⚠ {}", sample_text)).on_hover_text(
                                            "Programs should be done from .wav samples.",
                                        )
                                    };
                                    resp = resp.union(r);
                                }
                            });

                            // Root Note button
                            let layer = keygroup.layers[*self.current_layer].as_mut();
                            if let Some(layer) = layer {
                                let root_note_text = match &layer.root {
                                    Some(root) => {
                                        format!("🎵 {}{}", root.pitch(), root.octave(),)
                                    }
                                    None => "⚠ ???".to_string(),
                                };
                                ui.menu_button(root_note_text, |ui| {
                                    for octave in Octave::iter() {
                                        ui.menu_button(format!("Octave {}", octave), |ui| {
                                            for pitch in Pitch::iter() {
                                                if ui
                                                    .button(format!("{}{}", pitch, octave))
                                                    .clicked()
                                                {
                                                    let root = MidiNote::new(*pitch, *octave);
                                                    layer.root = Some(root);
                                                    resp.mark_changed();
                                                    ui.close_menu();
                                                }
                                            }
                                        });
                                    }
                                });
                            }

                            ui.end_row();
                        }

                        if let Some(delete_index) = delete_index {
                            self.keygroups[delete_index].layers[*self.current_layer] = None;
                        }

                        resp
                    })
                    .inner
            })
            .inner;

        ui.horizontal(|ui| {
            for i in 0..4 {
                let layer_name = i + 1;
                ui.selectable_value(self.current_layer, i, format!("Layer {layer_name}"));
            }
        });
        response
    }
}
