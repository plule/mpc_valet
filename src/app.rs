use std::collections::HashMap;

use crate::widgets::Keyboard;
use crate::KeygroupProgram;
use anyhow::Result;
use egui::Visuals;
use egui::{FontId, Layout, RichText, Vec2};

pub struct TemplateApp {
    pub program: KeygroupProgram,

    pub pitch_preference: f32,

    pub last_error: Result<()>,

    #[cfg(not(target_arch = "wasm32"))]
    pub sample_dir: std::path::PathBuf,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            program: Default::default(),
            pitch_preference: 0.5,
            last_error: Ok(()),
            #[cfg(not(target_arch = "wasm32"))]
            sample_dir: Default::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }

    fn main_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(crate::widgets::Header::default());
        ui.separator();
        self.samples_area_ui(ui);
        ui.separator();
        self.keyboard_ui(ui);
        ui.separator();
        self.save_ui(ui);
        ui.add(crate::widgets::Instructions::default());
        ui.add(crate::widgets::Footer::new(&self.last_error));
    }

    fn save_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("Instrument Name:");
                ui.text_edit_singleline(&mut self.program.name);
                let mut save_button =
                    crate::widgets::SaveProgramButton::new(&mut self.program, &mut self.last_error);

                #[cfg(not(target_arch = "wasm32"))]
                {
                    save_button.sample_dir = self.sample_dir.clone();
                }

                ui.add(save_button);
            });
        });
    }

    fn samples_area_ui(&mut self, ui: &mut egui::Ui) {
        if self.program.keygroups.is_empty() {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("⮊ Drag-and-drop you samples here! ⮈")
                        .font(FontId::proportional(20.0)),
                );
            });
        } else {
            ui.vertical(|ui| {
                ui.set_max_height(ui.available_height() / 2.0);
                let mut keygroups = self.program.keygroups.clone();
                if ui
                    .add(crate::widgets::SamplesTable::new(&mut keygroups))
                    .changed()
                {
                    self.program.update(keygroups, self.pitch_preference);
                }
            });
            if ui
                .add(crate::widgets::PitchSlider::new(&mut self.pitch_preference))
                .changed()
            {
                self.program.guess_ranges(self.pitch_preference);
            }
        }
    }

    fn keyboard_ui(&self, ui: &mut egui::Ui) {
        let mut colors = HashMap::new();
        let mut texts = HashMap::new();

        for kg in &self.program.keygroups {
            if let Some(settings) = &kg.settings {
                for note in settings.range.low.into_byte()..=settings.range.high.into_byte() {
                    let mut color = kg.color();
                    if note != settings.root.into_byte() {
                        color = color.linear_multiply(0.5);
                    }
                    colors.insert(note, color);
                    texts.insert(note, kg.file.clone());
                }
            }
        }
        ui.add(Keyboard::new(colors, texts));
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::from([512.0, ui.available_height()]),
                    Layout::top_down(egui::Align::Center),
                    |ui| {
                        self.main_ui(ui);
                    },
                );
            });
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path) = &ctx.input().raw.dropped_files[0].path {
                if let Some(dir) = path.parent() {
                    self.sample_dir = dir.to_path_buf();
                }
            }
            let filenames: Vec<String> = ctx
                .input()
                .raw
                .dropped_files
                .iter()
                .map(|drop| {
                    if let Some(path) = &drop.path {
                        if let Some(path) = path.file_name() {
                            if let Some(path) = path.to_str() {
                                return path.to_string();
                            }
                        }
                    }
                    drop.name.to_string()
                })
                .collect();
            self.program.add_files(filenames);
            self.program.guess_roots();
            self.program.guess_ranges(self.pitch_preference);
        }
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
