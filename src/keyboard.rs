use std::collections::HashMap;

use egui::{Color32, Widget};
use music_note::Pitch;

pub struct Keyboard {
    /// List of keys.
    ///
    /// Should contain all the notes
    pub note_colors: HashMap<u8, Color32>,
}

impl Keyboard {
    pub fn new(note_colors: HashMap<u8, Color32>) -> Self {
        Self { note_colors }
    }
}

impl Widget for Keyboard {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let key_dimension = egui::vec2(
            ui.spacing().interact_size.x / 4.0,
            ui.spacing().interact_size.y,
        );
        let keyboard_size = egui::vec2(
            (key_dimension.x + 1.0) * 70.0,
            (key_dimension.y + 1.0) * 2.0,
        );
        ui.spacing_mut().item_spacing.x = 1.0;
        ui.allocate_ui(keyboard_size, |ui| {
            // Black keys
            ui.horizontal(|ui| {
                // Half key offset
                ui.allocate_exact_size(
                    egui::vec2(key_dimension.x / 2.0, key_dimension.y),
                    egui::Sense::focusable_noninteractive(),
                );

                for note in crate::MIDI_NOTES.iter() {
                    let color = self
                        .note_colors
                        .get(&note.into_byte())
                        .unwrap_or(&Color32::BLACK);
                    match note.pitch() {
                        Pitch::B | Pitch::E => {
                            // Space between keys
                            ui.allocate_exact_size(
                                key_dimension,
                                egui::Sense::focusable_noninteractive(),
                            );
                        }
                        Pitch::CSharp
                        | Pitch::DSharp
                        | Pitch::FSharp
                        | Pitch::GSharp
                        | Pitch::ASharp => {
                            let (rect, mut response) =
                                ui.allocate_exact_size(key_dimension, egui::Sense::click());
                            if response.clicked() {
                                response.mark_changed();
                            }

                            let visuals = ui.style().interact_selectable(&response, true);

                            if ui.is_rect_visible(rect) {
                                // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
                                let rect = rect.expand(visuals.expansion);
                                let radius = 0.1 * rect.height();
                                ui.painter().rect(rect, radius, *color, visuals.bg_stroke);
                            }
                        }
                        _ => {}
                    }
                }
            });

            // White keys
            ui.horizontal(|ui| {
                for note in crate::MIDI_NOTES.iter() {
                    let color = self
                        .note_colors
                        .get(&note.into_byte())
                        .unwrap_or(&Color32::WHITE);
                    match note.pitch() {
                        Pitch::C
                        | Pitch::D
                        | Pitch::E
                        | Pitch::F
                        | Pitch::G
                        | Pitch::A
                        | Pitch::B => {
                            let (rect, mut response) =
                                ui.allocate_exact_size(key_dimension, egui::Sense::click());
                            if response.clicked() {
                                response.mark_changed();
                            }

                            let visuals = ui.style().interact_selectable(&response, true);

                            if ui.is_rect_visible(rect) {
                                let rect = rect.expand(visuals.expansion);
                                let radius = 0.1 * rect.height();
                                ui.painter().rect(rect, radius, *color, visuals.bg_stroke);
                            }
                        }
                        _ => {}
                    }
                }
            });
        })
        .response
    }
}
