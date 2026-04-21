mod app;
mod game;
mod ui;

mod helpers {
    pub mod action_helpers;
    pub mod web_utils;
    pub mod callback_helpers;
    pub mod rate_helpers;
    pub mod app_callbacks;
}

mod tabs {
    pub mod adventure_tab;
    pub mod enchantments_tab;
    pub mod hoard_tab;
    pub mod kobolds_tab;
}

use console_error_panic_hook::set_once;
use wasm_bindgen::prelude::*;
use yew::Renderer;

#[wasm_bindgen(start)]
pub fn run() {
    set_once();
    Renderer::<app::App>::new().render();
}
