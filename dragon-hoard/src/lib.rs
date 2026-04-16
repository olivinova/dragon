mod adventure_tab;
mod app;
mod action_helpers;
mod enchantments_tab;
mod game;
mod hoard_tab;
mod kobolds_tab;
mod ui;

use console_error_panic_hook::set_once;
use wasm_bindgen::prelude::*;
use yew::Renderer;

#[wasm_bindgen(start)]
pub fn run() {
    set_once();
    Renderer::<app::App>::new().render();
}
