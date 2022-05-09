use egui_extras::{Size, TableBuilder};

use crate::{keygroup::make_keygroups, KeygroupProgram};

pub struct TemplateApp {
    pub program: KeygroupProgram,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            program: KeygroupProgram::default(),
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
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");
            egui::warn_if_debug_build(ui);

            if self.program.keygroups.is_empty() {
                ui.label("Drag-and-drop files onto the window!");
            } else {
                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(
                        egui::Layout::left_to_right().with_cross_align(egui::Align::Center),
                    )
                    .columns(Size::initial(120.0), 5)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Sample");
                        });
                        header.col(|ui| {
                            ui.heading("Root note");
                        });
                        header.col(|ui| {
                            ui.heading("Low note");
                        });
                        header.col(|ui| {
                            ui.heading("High note");
                        });
                        header.col(|ui| {
                            if ui.button("Clear").clicked() {
                                self.program.keygroups.clear();
                            }
                        });
                    })
                    .body(|mut body| {
                        let mut delete_index = None;
                        for (index, keygroup) in self.program.keygroups.iter().enumerate() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(keygroup.file.clone());
                                });
                                row.col(|ui| match keygroup.root {
                                    Some(root) => {
                                        ui.label(format!("{}{}", root.pitch(), root.octave(),));
                                    }
                                    None => {
                                        ui.label("???");
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{}{}",
                                        keygroup.range.low.pitch(),
                                        keygroup.range.low.octave(),
                                    ));
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{}{}",
                                        keygroup.range.high.pitch(),
                                        keygroup.range.high.octave(),
                                    ));
                                });
                                row.col(|ui| {
                                    if ui.button("Delete").clicked() {
                                        delete_index = Some(index);
                                    }
                                });
                            });
                        }

                        if let Some(delete_index) = delete_index {
                            self.program.keygroups.remove(delete_index);
                            self.program.guess_roots();
                            self.program.guess_ranges();
                        }
                    });
            }
        });

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            let filenames: Vec<String> = ctx
                .input()
                .raw
                .dropped_files
                .iter()
                .map(|drop| {
                    drop.path
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect();
            self.program.add_files(filenames);
            self.program.guess_roots();
            self.program.guess_ranges();
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
