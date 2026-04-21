use crate::game::GameState;
use crate::helpers::action_helpers::{mutate_game, mutate_game_index};
use crate::helpers::callback_helpers::{make_event_toast_callback, make_optional_toast_callback, make_toast_callback};
use crate::helpers::web_utils::{alert_dialog, confirm_dialog, prompt_dialog};
use yew::prelude::*;
use serde_json;

pub fn create_app_callbacks(
    game: UseStateHandle<GameState>,
    toast: UseStateHandle<String>,
) -> AppCallbacks {
    AppCallbacks {
        // Game actions
        on_click_loot: mutate_game(game.clone(), |g| { g.click_loot(); }),
        on_recruit_kobold: mutate_game(game.clone(), |g| { g.recruit_kobold(); }),
        on_assign_mining: mutate_game(game.clone(), |g| { g.assign_mining(); }),
        on_unassign_mining: mutate_game(game.clone(), |g| { g.unassign_mining(); }),
        on_assign_farming: mutate_game(game.clone(), |g| { g.assign_farming(); }),
        on_unassign_farming: mutate_game(game.clone(), |g| { g.unassign_farming(); }),
        on_assign_digging: mutate_game(game.clone(), |g| { g.assign_digging(); }),
        on_unassign_digging: mutate_game(game.clone(), |g| { g.unassign_digging(); }),
        on_upgrade_kobolds: mutate_game(game.clone(), |g| { g.upgrade_kobold_efficiency(); }),
        on_buy_training: mutate_game(game.clone(), |g| { g.buy_training(); }),
        on_buy_vault: mutate_game(game.clone(), |g| { g.buy_vault(); }),
        on_designate_housing: mutate_game(game.clone(), |g| { g.designate_storage_to_housing(); }),
        on_reclaim_housing: mutate_game(game.clone(), |g| { g.reclaim_housing_to_storage(); }),
        on_designate_furniture: mutate_game(game.clone(), |g| { g.designate_storage_to_furniture(); }),
        on_reclaim_furniture: mutate_game(game.clone(), |g| { g.reclaim_furniture_to_storage(); }),

        // Save/load actions
        on_export: make_toast_callback({
            let game = game.clone();
            move || {
                if let Ok(data) = serde_json::to_string_pretty(&*game) {
                    let _ = prompt_dialog("Export your save data:", &data);
                }
            }
        }, toast.clone(), "Export ready. Copy it from the prompt."),

        on_import: make_optional_toast_callback({
            let game = game.clone();
            move || {
                if let Some(payload) = prompt_dialog("Paste save data to import:", "") {
                    if payload.trim().is_empty() {
                        return None;
                    }
                    match serde_json::from_str::<GameState>(&payload) {
                        Ok(imported) => {
                            imported.save();
                            game.set(imported);
                            return Some("Import successful.".to_string());
                        }
                        Err(_) => {
                            alert_dialog("Import failed: invalid save data.");
                        }
                    }
                }
                None
            }
        }, toast.clone()),

        on_save: make_toast_callback({
            let game = game.clone();
            move || {
                (*game).save();
            }
        }, toast.clone(), "Saved game."),

        on_load: make_optional_toast_callback({
            let game = game.clone();
            move || {
                if let Some(loaded) = GameState::load() {
                    game.set(loaded);
                    return Some("Save loaded.".to_string());
                }
                alert_dialog("No saved game found.");
                None
            }
        }, toast.clone()),

        on_reset: make_optional_toast_callback({
            let game = game.clone();
            move || {
                if confirm_dialog("Reset all progress? This cannot be undone.") {
                    let mut reset_state = (*game).clone();
                    reset_state.reset();
                    reset_state.save();
                    game.set(reset_state);
                    return Some("Progress reset.".to_string());
                }
                None
            }
        }, toast.clone()),

        // Magic actions
        on_learn_magic: mutate_game(game.clone(), |g| { g.learn_magic(); }),
        on_craft_enchant: mutate_game(game.clone(), |g| { g.craft_enchantment(); }),
        on_sell_enchant: mutate_game_index(game.clone(), |g, idx| { g.sell_enchantment(idx); }),

        // Adventure actions
        on_conquer_town: make_event_toast_callback({
            let game = game.clone();
            move |idx: usize| {
                let mut g2 = (*game).clone();
                let (_ok, msg) = g2.try_conquer_town(idx);
                g2.save();
                game.set(g2);
                msg
            }
        }, toast.clone()),

        on_trade_town: make_event_toast_callback({
            let game = game.clone();
            move |idx: usize| {
                let mut g2 = (*game).clone();
                let (_ok, msg) = g2.try_trade_town(idx);
                g2.save();
                game.set(g2);
                msg
            }
        }, toast.clone()),

        on_explore_dungeon: make_event_toast_callback({
            let game = game.clone();
            move |idx: usize| {
                let mut g2 = (*game).clone();
                let (_ok, msg) = g2.explore_dungeon(idx);
                g2.save();
                game.set(g2);
                msg
            }
        }, toast.clone()),
    }
}

#[derive(Clone)]
pub struct AppCallbacks {
    pub on_click_loot: Callback<MouseEvent>,
    pub on_recruit_kobold: Callback<MouseEvent>,
    pub on_assign_mining: Callback<MouseEvent>,
    pub on_unassign_mining: Callback<MouseEvent>,
    pub on_assign_farming: Callback<MouseEvent>,
    pub on_unassign_farming: Callback<MouseEvent>,
    pub on_assign_digging: Callback<MouseEvent>,
    pub on_unassign_digging: Callback<MouseEvent>,
    pub on_upgrade_kobolds: Callback<MouseEvent>,
    pub on_buy_training: Callback<MouseEvent>,
    pub on_buy_vault: Callback<MouseEvent>,
    pub on_designate_housing: Callback<MouseEvent>,
    pub on_reclaim_housing: Callback<MouseEvent>,
    pub on_designate_furniture: Callback<MouseEvent>,
    pub on_reclaim_furniture: Callback<MouseEvent>,
    pub on_export: Callback<MouseEvent>,
    pub on_import: Callback<MouseEvent>,
    pub on_save: Callback<MouseEvent>,
    pub on_load: Callback<MouseEvent>,
    pub on_reset: Callback<MouseEvent>,
    pub on_learn_magic: Callback<MouseEvent>,
    pub on_craft_enchant: Callback<MouseEvent>,
    pub on_sell_enchant: Callback<usize>,
    pub on_conquer_town: Callback<usize>,
    pub on_trade_town: Callback<usize>,
    pub on_explore_dungeon: Callback<usize>,
}