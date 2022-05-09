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
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");
            egui::warn_if_debug_build(ui);

            ui.text_edit_singleline(&mut self.program.name);

            if self.program.keygroups.is_empty() {
                ui.label("Drag-and-drop files onto the window!");
            }

            TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
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

            if ui.button("Save").clicked() {
                let file_name = format!("{}.xpm", self.program.name);
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use rfd::FileDialog;
                    use std::fs::File;

                    if let Some(dest) = FileDialog::new()
                        .set_directory(&self.sample_dir)
                        .add_filter("MPC Program", &["xpm"])
                        .set_file_name(file_name.as_str())
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
                {
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
                    element
                        .set_attribute("download", file_name.as_str())
                        .unwrap();
                    document.body().unwrap().append_child(&element).unwrap();
                    element.click();
                    document.body().unwrap().remove_child(&element).unwrap();
                }
            }
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
                .map(|drop| drop.name.to_string())
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
