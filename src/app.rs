use crate::keygroup::{make_keygroups, KeyGroup};

pub struct TemplateApp {
    keygroups: Vec<KeyGroup>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            keygroups: Vec::new(),
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");
            egui::warn_if_debug_build(ui);

            if self.keygroups.is_empty() {
                ui.label("Drag-and-drop files onto the window!");
            } else {
                for keygroup in &self.keygroups {
                    ui.label(format!(
                        "{}: {}{} ({}{}-{}{})",
                        keygroup.file,
                        keygroup.root.pitch(),
                        keygroup.root.octave(),
                        keygroup.range.low.pitch(),
                        keygroup.range.low.octave(),
                        keygroup.range.high.pitch(),
                        keygroup.range.high.octave(),
                    ));
                }
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

            self.keygroups = make_keygroups(filenames);
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
