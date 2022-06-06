use std::collections::HashMap;

use crate::widgets::Keyboard;
use crate::KeygroupProgram;
use anyhow::Result;
use egui::Visuals;
use egui::{Layout, Vec2};

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

    fn keyboard_ui(&self, ui: &mut egui::Ui) {
        let mut colors = HashMap::new();
        let mut texts = HashMap::new();

        for (kg, root, range) in self
            .program
            .keygroups
            .iter()
            .filter_map(|kg| Some((kg, kg.layers[0].root?, kg.range.as_ref()?)))
        {
            for note in range.low.into_byte()..=range.high.into_byte() {
                let mut color = kg.color();
                if note != root.into_byte() {
                    color = color.linear_multiply(0.5);
                }
                colors.insert(note, color);
                texts.insert(note, kg.layers[0].file.clone());
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
                        ui.add(crate::widgets::Header::default());
                        ui.separator();
                        ui.add(crate::widgets::SamplesArea::new(
                            &mut self.program,
                            &mut self.pitch_preference,
                        ));
                        ui.separator();
                        self.keyboard_ui(ui);
                        ui.separator();
                        self.save_ui(ui);
                        ui.add(crate::widgets::Instructions::default());
                        ui.add(crate::widgets::Footer::new(&self.last_error));
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
