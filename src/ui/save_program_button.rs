use crate::KeygroupProgram;
use crate::LayerVelocityMode;
use anyhow::Context;
use anyhow::Result;
use egui::{FontId, RichText, Widget};
pub struct SaveProgramButton<'a> {
    pub program: &'a mut KeygroupProgram,

    pub layer_mode: &'a LayerVelocityMode,

    pub last_error: &'a mut Result<()>,

    #[cfg(not(target_arch = "wasm32"))]
    pub sample_dir: std::path::PathBuf,
}

impl<'a> SaveProgramButton<'a> {
    pub fn new(
        program: &'a mut KeygroupProgram,
        layer_mode: &'a LayerVelocityMode,
        last_error: &'a mut Result<()>,
    ) -> Self {
        Self {
            program,
            last_error,
            layer_mode,
            #[cfg(not(target_arch = "wasm32"))]
            sample_dir: Default::default(),
        }
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
}

impl<'a> Widget for SaveProgramButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui
            .button(RichText::new("Save").font(FontId::proportional(20.0)))
            .on_disabled_hover_text("Add samples first")
            .on_hover_text("Make sure to save the file in the same folder as the samples!");
        if button.clicked() {
            self.program.set_velocity_layer_mode(self.layer_mode);
            let file_name = format!("{}.xpm", self.program.name);
            if let Err(e) = self.export_program_dialog(&file_name) {
                *self.last_error = Err(e)
            }
        }
        button
    }
}
