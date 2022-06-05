use egui::{Color32, RichText, Widget};
use music_note::midi::MidiNote;

use crate::{Keygroup, KeygroupSettings, Range};

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
                            let r = ui.horizontal(|ui| match &keygroup.settings {
                                Some(settings) => ui.label(format!(
                                    "{}{} to {}{}",
                                    settings.range.low.pitch(),
                                    settings.range.low.octave(),
                                    settings.range.high.pitch(),
                                    settings.range.high.octave(),
                                )),
                                None => ui.label("⚠ ???").on_hover_text(
                                    "Unknown root note. This sample will be ignored.",
                                ),
                            });
                            resp = resp.union(r.response);

                            // Layer 1
                            let sample_text = keygroup.file.clone();
                            let r = if keygroup.file.ends_with(".wav")
                                || keygroup.file.ends_with(".WAV")
                            {
                                ui.label(RichText::new(format!("🎵 {}", sample_text)).color(color))
                            } else {
                                ui.label(format!("⚠ {}", sample_text))
                                    .on_hover_text("Programs should be done from .wav samples.")
                            };
                            resp = resp.union(r);

                            // Layer 1 Root Note
                            let root_note_text = match &keygroup.settings {
                                Some(settings) => {
                                    format!(
                                        "🎵 {}{}",
                                        settings.root.pitch(),
                                        settings.root.octave(),
                                    )
                                }
                                None => "⚠ ???".to_string(),
                            };
                            ui.menu_button(root_note_text, |ui| {
                                for octave in crate::OCTAVES {
                                    ui.menu_button(format!("Octave {}", octave), |ui| {
                                        for pitch in crate::PITCHES {
                                            if ui.button(format!("{}{}", pitch, octave)).clicked() {
                                                let root = MidiNote::new(pitch, octave);
                                                keygroup.settings = Some(KeygroupSettings::new(
                                                    root,
                                                    Range::default(),
                                                ));
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
