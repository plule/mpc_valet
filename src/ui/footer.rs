use egui::{Color32, RichText, TextStyle, Widget};

pub struct Footer<'a> {
    pub last_error: &'a anyhow::Result<()>,
}

impl<'a> Footer<'a> {
    pub fn new(last_error: &'a anyhow::Result<()>) -> Self {
        Self { last_error }
    }
}

impl<'a> Widget for Footer<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut response = ui
            .horizontal_wrapped(|ui| {
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
            })
            .response;
        if let Err(e) = &self.last_error {
            response =
                response.union(ui.label(RichText::new(format!("{:?}", e)).color(Color32::RED)));
        }
        egui::warn_if_debug_build(ui);
        response
    }
}
