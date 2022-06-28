#![warn(clippy::all, rust_2018_idioms)]

pub mod export;
mod keygroup;
mod keygroup_program;
mod layer;
mod parse;
mod range;
mod static_iterable;
mod ui;

pub use keygroup::*;
pub use keygroup_program::*;
pub use layer::*;
pub use parse::*;
pub use range::*;
pub use static_iterable::*;
pub use ui::App;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(App::new(cc))))
}

/// Velocity range assignment mode.
#[derive(PartialEq)]
pub enum LayerVelocityMode {
    /// Set the full range to all the layers.
    Overlapping,

    /// Assign non overlapping ranges to each layer.
    Spread,
}
