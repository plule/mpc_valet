use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::keyboard::Keyboard;
use crate::{KeygroupProgram, KeygroupSettings, Range};
use anyhow::Context;
use anyhow::Result;
use egui::Visuals;
use egui::{Color32, FontId, Layout, RichText, TextStyle, Vec2};
use egui_extras::{Size, TableBuilder};
use music_note::midi::MidiNote;
use random_color::{Luminosity, RandomColor};

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
        header_ui(ui);
        ui.separator();
        self.samples_area_ui(ui);
        ui.separator();
        self.keyboard_ui(ui);
        ui.separator();
        self.save_ui(ui);
        instructions_ui(ui);
        self.footer_ui(ui);
    }

    fn save_ui(&mut self, ui: &mut egui::Ui) {
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
                        if let Err(e) = self.export_program_dialog(&file_name) {
                            self.last_error = Err(e)
                        }
                    }
                });
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn export_program_dialog(&self, file_name: &str) -> Result<()> {
        use rfd::FileDialog;
        use std::fs::File;

        if let Some(dest) = FileDialog::new()
            .set_directory(&self.sample_dir)
            .add_filter("MPC Program", &["xpm"])
            .set_file_name(file_name)
            .save_file()
        {
            let f = File::create(dest).context("Failed to create the instrument file")?;
            self.program.export(f)?;
        }

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    fn export_program_dialog(&self, file_name: &str) -> Result<()> {
        // look mom i do the web
        use anyhow::bail;
        use eframe::wasm_bindgen::JsCast;
        use js_sys::encode_uri_component;

        let mut file_content = Vec::<u8>::new();
        self.program.export(&mut file_content)?;
        let file_content = String::from_utf8(file_content)
            .context("Failed to convert the instrument file to UTF8")?;
        let file_content = encode_uri_component(&file_content);

        let window = web_sys::window().context("Failed to get the browser window")?;
        let document = window
            .document()
            .context("Failed to get the window document")?;
        let body = document.body().context("Failed to get the document body")?;
        let element = document
            .create_element("a")
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to insert a link in the document")?;
        let element = element
            .dyn_into::<web_sys::HtmlElement>()
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to convert the element to an HTML element")?;
        element
            .set_attribute(
                "href",
                format!("data:text/plain;charset=utf-8,{}", file_content).as_str(),
            )
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to set the element destination")?;
        element
            .set_attribute("download", file_name)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to create the download file name")?;
        body.append_child(&element)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to insert the element in the document")?;
        element.click();
        body.remove_child(&element)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to remove the element from the document")?;

        Ok(())
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
                self.samples_table(ui);
            });
            ui.horizontal(|ui| {
                ui.label("pitch down");
                if ui
                    .add(
                        egui::Slider::new(&mut self.pitch_preference, 0.0..=1.0)
                            .clamp_to_range(true)
                            .show_value(false),
                    )
                    .changed()
                {
                    self.program.guess_ranges(self.pitch_preference);
                }
                ui.label("pitch up");
            });
        }
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
                let mut guess_ranges = false;
                for (index, keygroup) in self.program.keygroups.iter_mut().enumerate() {
                    let color = note_color(&keygroup.file);
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
                                ui.label(RichText::new(format!("🎵 {}", text)).color(color));
                            } else {
                                ui.label(format!("⚠ {}", text))
                                    .on_hover_text("Programs should be done from .wav samples.");
                            }
                        });
                        row.col(|ui| {
                            let text = match &keygroup.settings {
                                Some(settings) => {
                                    format!(
                                        "🎵 {}{}",
                                        settings.root.pitch(),
                                        settings.root.octave(),
                                    )
                                }
                                None => "⚠ ???".to_string(),
                            };

                            ui.menu_button(text, |ui| {
                                for octave in crate::OCTAVES {
                                    ui.menu_button(format!("Octave {}", octave), |ui| {
                                        for pitch in crate::PITCHES {
                                            if ui.button(format!("{}{}", pitch, octave)).clicked() {
                                                let root = MidiNote::new(pitch, octave);
                                                keygroup.settings = Some(KeygroupSettings::new(
                                                    root,
                                                    Range::default(),
                                                ));
                                                guess_ranges = true;
                                                ui.close_menu();
                                            }
                                        }
                                    });
                                }
                            });
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
                    self.program.guess_ranges(self.pitch_preference);
                }

                if guess_ranges {
                    self.program.guess_ranges(self.pitch_preference);
                }
            });
    }

    fn keyboard_ui(&self, ui: &mut egui::Ui) {
        let mut colors = HashMap::new();
        let mut texts = HashMap::new();

        for kg in &self.program.keygroups {
            if let Some(settings) = &kg.settings {
                for note in settings.range.low.into_byte()..=settings.range.high.into_byte() {
                    let mut color = note_color(&kg.file);
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

    fn footer_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            let width = ui
                .fonts()
                .glyph_width(&TextStyle::Body.resolve(ui.style()), ' ');
            ui.spacing_mut().item_spacing.x = width;
            const VERSION: &str = env!("CARGO_PKG_VERSION");
            ui.small(format!("MPC Valet v{VERSION}."));
            ui.small("Made by");
            ui.hyperlink_to(
                egui::RichText::new("plule").small(),
                "https://plule.github.io/",
            );
            ui.small("with");
            ui.hyperlink_to(egui::RichText::new("egui.").small(), "https://www.egui.rs");
            ui.spacing();
            ui.hyperlink_to(
                egui::RichText::new("Source code.").small(),
                "https://github.com/plule/mpc_valet",
            );
        });
        if let Err(e) = &self.last_error {
            ui.label(RichText::new(format!("{:?}", e)).color(Color32::RED));
        }
        egui::warn_if_debug_build(ui);
    }
}

fn instructions_ui(ui: &mut egui::Ui) {
    ui.collapsing("Instructions", |ui| {
        ui.label("Drag and drop samples from instrument notes on this window.");
        ui.label("Make sure the file contain the note in their name, such as \"Violin-C3.wav\" or \"Flute-43.wav\"");
        ui.label("This software will find appropriate note range for each sample.");
        ui.label("When you are done, choose a name and click \"Save\". The XPM file must be saved in the same folder as the samples.");
        ui.label("Open the XPM with the MPC Software, or any of the MPC Live, One, X...");
    });
}

fn header_ui(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("MPC Valet");

        for _ in 0..4 {
            ui.heading(egui::RichText::new("▣ ▣ ▣ ▣").text_style(egui::TextStyle::Monospace));
        }
    });
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

fn note_color(sample: &str) -> Color32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    sample.hash(&mut hasher);
    let color = RandomColor::new()
        .seed(hasher.finish())
        .luminosity(Luminosity::Light)
        .to_rgb_array();
    Color32::from_rgb(color[0], color[1], color[2])
}
