use egui::{Color32, FontId, Layout, RichText, Vec2};
use egui_extras::{Size, TableBuilder};

use crate::KeygroupProgram;

#[derive(Default)]
pub struct TemplateApp {
    pub program: KeygroupProgram,

    #[cfg(not(target_arch = "wasm32"))]
    pub sample_dir: std::path::PathBuf,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
    }

    fn main_ui(&mut self, ui: &mut egui::Ui) {
        // Header
        ui.vertical_centered(|ui| {
            ui.heading("MPC Keygroup Instrument Creator");

            for _ in 0..4 {
                ui.heading(egui::RichText::new("▣ ▣ ▣ ▣").text_style(egui::TextStyle::Monospace));
            }
        });

        ui.separator();

        // Sample table
        if self.program.keygroups.is_empty() {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("⮊ Drag-and-drop you samples here! ⮈")
                        .font(FontId::proportional(20.0)),
                );
            });
        } else {
            self.samples_table(ui);
        }
        ui.separator();

        // Save
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("Instrument Name:");
                ui.text_edit_singleline(&mut self.program.name);

                ui.add_enabled_ui(self.program.can_export(), |ui| {
                    let button = ui
                        .button(RichText::new("Save").font(FontId::proportional(20.0)))
                        .on_disabled_hover_text("Add samples first")
                        .on_hover_text(
                            "Make sure to save the file in the same folder as the samples!",
                        );
                    if button.clicked() {
                        let file_name = format!("{}.xpm", self.program.name);
                        self.export_program_dialog(ui, &file_name);
                    }
                });
            });
        });

        // Instructions
        ui.collapsing("Instructions", |ui| {
            ui.label("Drag and drop samples from instrument notes on this window.");
            ui.label("Make sure the file contain the note in their name, such as \"Violin-C3.wav\".");
            ui.label("This software will find appropriate note range for each sample.");
            ui.label("When you are done, choose a name and click \"Save\". The XPM file must be saved in the same folder as the samples.");
            ui.label("Open the XPM with the MPC Software, or any of the MPC Live, One, X...");
        });

        egui::warn_if_debug_build(ui);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn export_program_dialog(&self, ui: &mut egui::Ui, file_name: &str) {
        use rfd::FileDialog;
        use std::fs::File;

        if let Some(dest) = FileDialog::new()
            .set_directory(&self.sample_dir)
            .add_filter("MPC Program", &["xpm"])
            .set_file_name(file_name)
            .save_file()
        {
            match File::create(dest) {
                Ok(f) => {
                    self.program.export(f);
                }
                Err(e) => {
                    ui.label(format!("Failed to create the program: {}", e));
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn export_program_dialog(&self, _ui: &mut egui::Ui, file_name: &str) {
        // look mom i do the web
        use eframe::wasm_bindgen::JsCast;
        use js_sys::encode_uri_component;

        let mut file_content = Vec::<u8>::new();
        self.program.export(&mut file_content);
        let file_content = String::from_utf8(file_content).unwrap();
        let file_content = encode_uri_component(&file_content);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element("a").unwrap();
        let element = element.dyn_into::<web_sys::HtmlElement>().unwrap();
        element
            .set_attribute(
                "href",
                format!("data:text/plain;charset=utf-8,{}", file_content).as_str(),
            )
            .unwrap();
        element.set_attribute("download", file_name).unwrap();
        document.body().unwrap().append_child(&element).unwrap();
        element.click();
        document.body().unwrap().remove_child(&element).unwrap();
    }

    fn samples_table(&mut self, ui: &mut egui::Ui) {
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::remainder())
            .column(Size::relative(0.30)) // sample
            .column(Size::relative(0.20)) // Root Note
            .column(Size::relative(0.40)) // Range // delete
            .header(20.0, |mut header| {
                header.col(|_| {});
                header.col(|ui| {
                    ui.heading("Sample");
                });
                header.col(|ui| {
                    ui.heading("Root Note");
                });
                header.col(|ui| {
                    ui.heading("Range");
                });
            })
            .body(|mut body| {
                let mut delete_index = None;
                for (index, keygroup) in self.program.keygroups.iter().enumerate() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            if ui
                                .button(RichText::new("❌").color(Color32::GRAY))
                                .clicked()
                            {
                                delete_index = Some(index);
                            }
                        });
                        row.col(|ui| {
                            let text = keygroup.file.clone();
                            if keygroup.file.ends_with(".wav") || keygroup.file.ends_with(".WAV") {
                                ui.label(format!("🎵 {}", text));
                            } else {
                                ui.label(format!("⚠ {}", text))
                                    .on_hover_text("Programs should be done from .wav samples.");
                            }
                        });
                        row.col(|ui| match &keygroup.settings {
                            Some(settings) => {
                                ui.label(format!(
                                    "🎵 {}{}",
                                    settings.root.pitch(),
                                    settings.root.octave(),
                                ));
                            }
                            None => {
                                ui.label("⚠ ???").on_hover_text(
                                    "Unknown root note. This sample will be ignored.",
                                );
                            }
                        });
                        row.col(|ui| match &keygroup.settings {
                            Some(settings) => {
                                ui.label(format!(
                                    "{}{} to {}{}",
                                    settings.range.low.pitch(),
                                    settings.range.low.octave(),
                                    settings.range.high.pitch(),
                                    settings.range.high.octave(),
                                ));
                            }
                            None => {
                                ui.label("⚠ ???").on_hover_text(
                                    "Unknown root note. This sample will be ignored.",
                                );
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
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    self.sample_dir = dir.clone().to_path_buf();
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
                    return drop.name.to_string();
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
