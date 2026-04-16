use crate::adventure_tab::AdventureTab;
use crate::action_helpers::{mutate_game, mutate_game_index};
use crate::enchantments_tab::EnchantmentsTab;
use crate::game::GameState;
use crate::hoard_tab::HoardTab;
use crate::kobolds_tab::KoboldsTab;
use crate::ui::{format_cost, format_number, labeled_cost, ActionButton, ICON_FOOD, ICON_GOLD, ICON_MANA, NumberFormat, Panel, StatRow, TabButton};
use gloo_timers::callback::Interval;
use serde_json;
use web_sys::Window;
use yew::prelude::*;

fn app_window() -> Window {
    web_sys::window().expect("window should be available")
}

fn alert_dialog(message: &str) {
    let _ = app_window().alert_with_message(message);
}

fn confirm_dialog(message: &str) -> bool {
    app_window().confirm_with_message(message).unwrap_or(false)
}

fn prompt_dialog(message: &str, default: &str) -> Option<String> {
    app_window().prompt_with_message_and_default(message, default).ok().flatten()
}

fn make_toast_callback(action: impl Fn() + 'static, toast: UseStateHandle<String>, message: &'static str) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        action();
        toast.set(message.to_string());
    })
}

/// Main app component for Dragon Hoard.
///
/// Manages the game state, selected tab, toast messages, and timer ticks.
#[function_component(App)]
pub fn app() -> Html {
    let game = use_state(|| GameState::load().unwrap_or_default());
    let toast = use_state(|| String::new());
    let tab = use_state(|| "Hoard".to_string());
    let number_style = use_state(|| NumberFormat::Compact);

    // Tick every second
    {
        let game = game.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                game.set({
                    let mut g = (*game).clone();
                    g.tick(1.0);
                    g
                });
            });
            move || drop(interval)
        });
    }

    // Action callbacks are centralized in action_helpers.rs for reuse and clarity.
    let on_click_loot = mutate_game(game.clone(), |g| { g.click_loot(); });
    let on_recruit_kobold = mutate_game(game.clone(), |g| { g.recruit_kobold(); });
    let on_assign_mining = mutate_game(game.clone(), |g| { g.assign_mining(); });
    let on_unassign_mining = mutate_game(game.clone(), |g| { g.unassign_mining(); });
    let on_assign_farming = mutate_game(game.clone(), |g| { g.assign_farming(); });
    let on_unassign_farming = mutate_game(game.clone(), |g| { g.unassign_farming(); });
    let on_assign_digging = mutate_game(game.clone(), |g| { g.assign_digging(); });
    let on_unassign_digging = mutate_game(game.clone(), |g| { g.unassign_digging(); });
    let on_upgrade_kobolds = mutate_game(game.clone(), |g| { g.upgrade_kobold_efficiency(); });
    let on_buy_training = mutate_game(game.clone(), |g| { g.buy_training(); });
    let on_buy_vault = mutate_game(game.clone(), |g| { g.buy_vault(); });
    let on_designate_housing = mutate_game(game.clone(), |g| { g.designate_storage_to_housing(); });
    let on_reclaim_housing = mutate_game(game.clone(), |g| { g.reclaim_housing_to_storage(); });
    let on_designate_furniture = mutate_game(game.clone(), |g| { g.designate_storage_to_furniture(); });
    let on_reclaim_furniture = mutate_game(game.clone(), |g| { g.reclaim_furniture_to_storage(); });

    let on_export = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            if let Ok(data) = serde_json::to_string_pretty(&*game) {
                let _ = prompt_dialog("Export your save data:", &data);
                toast.set("Export ready. Copy it from the prompt.".to_string());
            }
        })
    };

    let on_import = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            if let Some(payload) = prompt_dialog("Paste save data to import:", "") {
                if payload.trim().is_empty() {
                    return;
                }
                match serde_json::from_str::<GameState>(&payload) {
                    Ok(imported) => {
                        imported.save();
                        game.set(imported);
                        toast.set("Import successful.".to_string());
                    }
                    Err(_) => {
                        alert_dialog("Import failed: invalid save data.");
                    }
                }
            }
        })
    };

    let on_save = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            (*game).save();
            toast.set("Saved game.".to_string());
        })
    };

    let on_load = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            if let Some(loaded) = GameState::load() {
                game.set(loaded);
                toast.set("Save loaded.".to_string());
            } else {
                alert_dialog("No saved game found.");
            }
        })
    };

    let on_reset = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            if confirm_dialog("Reset all progress? This cannot be undone.") {
                let mut reset_state = (*game).clone();
                reset_state.reset();
                reset_state.save();
                game.set(reset_state);
                toast.set("Progress reset.".to_string());
            }
        })
    };

    // Magic / Enchantments actions
    let on_learn_magic = mutate_game(game.clone(), |g| { g.learn_magic(); });
    let on_craft_enchant = mutate_game(game.clone(), |g| { g.craft_enchantment(); });
    let on_sell_enchant = mutate_game_index(game.clone(), |g, idx| { g.sell_enchantment(idx); });

    // Conquer towns
    let on_conquer_town = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |idx: usize| {
            let mut g2 = (*game).clone();
            let (_ok, msg) = g2.try_conquer_town(idx);
            g2.save();
            game.set(g2);
            toast.set(msg);
        })
    };

    // Explore dungeons
    let on_explore_dungeon = {
        let game = game.clone();
        let toast = toast.clone();
        Callback::from(move |idx: usize| {
            let mut g2 = (*game).clone();
            let (_ok, msg) = g2.explore_dungeon(idx);
            g2.save();
            game.set(g2);
            toast.set(msg);
        })
    };

    let g = (*game).clone();
    let current_number_style = *number_style;

    let gold_rate = g.gold_per_sec + g.assigned_mining as f64 * 0.6 * g.kobold_efficiency;
    let food_rate = g.assigned_farming as f64 * 0.35 * g.kobold_efficiency - g.kobold_upkeep();
    let mana_rate = if g.mana < g.mana_capacity { g.mana_regen_per_sec } else { 0.0 };

    let rate_hint = |rate: f64| {
        let text = format_number(rate, current_number_style, 1);
        if rate >= 0.0 {
            format!("+{}/s", text)
        } else {
            format!("{}/s", text)
        }
    };

    let rate_class = |rate: f64| {
        if rate >= 0.0 {
            "positive".to_string()
        } else {
            "negative".to_string()
        }
    };

    html! {
        <div class="container">
            <div class="page-header">
                <div>
                    <h1>{"Dragon Hoard"}</h1>
                </div>
                <div class="save-actions">
                    <ActionButton label={"Save".to_string()} onclick={on_save.clone()} class={"compact".to_string()} title={"Save your current progress locally.".to_string()} />
                    <ActionButton label={"Load".to_string()} onclick={on_load.clone()} class={"compact".to_string()} title={"Load your last saved game.".to_string()} />
                    <ActionButton label={"Export".to_string()} onclick={on_export.clone()} class={"compact".to_string()} title={"Export save data to copy and backup.".to_string()} />
                    <ActionButton label={"Import".to_string()} onclick={on_import.clone()} class={"compact".to_string()} title={"Import save data from a copied backup.".to_string()} />
                    <ActionButton label={"Reset".to_string()} onclick={on_reset.clone()} class={"compact danger".to_string()} title={"Reset the entire game after confirmation.".to_string()} />
                </div>
            </div>

            <div class="main-layout">
                <aside class="resource-sidebar">
                    <details open={true} class="resource-section">
                        <summary class="resource-summary">{"Resources"}</summary>
                        <Panel class={"resource-panel".to_string()}>
                                    <StatRow
                                label={"Gold".to_string()}
                                icon={Some(ICON_GOLD.to_string())}
                                value={format!("{}/{}", format_number(g.gold, current_number_style, 1), format_number(g.gold_capacity, current_number_style, 1))}
                                hint={rate_hint(gold_rate)}
                                hint_class={rate_class(gold_rate)}
                            />
                            <StatRow
                                label={"Food".to_string()}
                                icon={Some(ICON_FOOD.to_string())}
                                value={format_number(g.food, current_number_style, 1)}
                                hint={rate_hint(food_rate)}
                                hint_class={rate_class(food_rate)}
                            />
                            <StatRow
                                label={"Mana".to_string()}
                                icon={Some(ICON_MANA.to_string())}
                                value={format!("{}/{}", format_number(g.mana, current_number_style, 1), format_number(g.mana_capacity, current_number_style, 1))}
                                hint={rate_hint(mana_rate)}
                                hint_class={rate_class(mana_rate)}
                            />
                        </Panel>
                    </details>
                    <details open={true} class="resource-section">
                        <summary class="resource-summary">{"Kobolds"}</summary>
                        <Panel class={"resource-panel".to_string()}>
                            <StatRow label={"Population".to_string()} value={format!("{}/{}", g.kobolds, g.housing_slots)} />
                            <StatRow label={"Free".to_string()} value={format!("{}", g.free_kobolds())} />
                            <StatRow label={"Upkeep".to_string()} value={format!("{:.1}/s", g.kobold_upkeep())} />
                        </Panel>
                    </details>
                </aside>

                <main class="right-panel">
                    <Panel class={"toolbar".to_string()}>
                        <ActionButton label={format!("Loot (click) +{}", labeled_cost(ICON_GOLD, g.click_multiplier, current_number_style))} onclick={on_click_loot.clone()} class={"big".to_string()} />
                        <div class="controls format-toggle">
                            { for [
                                (NumberFormat::Standard, "Standard"),
                                (NumberFormat::Compact, "Compact"),
                                (NumberFormat::Scientific, "Scientific"),
                            ].iter().map(|(style, title)| {
                                let style_value = *style;
                                let selected = current_number_style == style_value;
                                let class = if selected { "format-button active" } else { "format-button" };
                                let on_click = {
                                    let number_style = number_style.clone();
                                    Callback::from(move |_| number_style.set(style_value))
                                };

                                html! {
                                    <button class={classes!(class)} onclick={on_click}>{ title }</button>
                                }
                            }) }
                        </div>
                    </Panel>

                    <Panel class={"tabs-panel".to_string()}>
                        <div class="tab-strip">
                            { for ["Hoard", "Kobolds", "Enchantments", "Adventure"].iter().map(|label| {
                                let label = label.to_string();
                                let active = *tab == label;
                                let tab_state = tab.clone();
                                let onclick_label = label.clone();
                                let onclick = Callback::from(move |_| tab_state.set(onclick_label.clone()));
                                html! {
                                    <TabButton label={label.clone()} active={active} {onclick} />
                                }
                            }) }
                        </div>
                    </Panel>

                    <Panel class={"content-panel".to_string()}>
                        { if *tab == "Hoard" {
                            html! {
                                <HoardTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_buy_training={on_buy_training.clone()}
                                    on_buy_vault={on_buy_vault.clone()}
                                />
                            }
                        } else if *tab == "Kobolds" {
                            html! {
                                <KoboldsTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_recruit_kobold={on_recruit_kobold.clone()}
                                    on_assign_mining={on_assign_mining.clone()}
                                    on_unassign_mining={on_unassign_mining.clone()}
                                    on_assign_farming={on_assign_farming.clone()}
                                    on_unassign_farming={on_unassign_farming.clone()}
                                    on_assign_digging={on_assign_digging.clone()}
                                    on_unassign_digging={on_unassign_digging.clone()}
                                    on_upgrade_kobolds={on_upgrade_kobolds.clone()}
                                    on_designate_housing={on_designate_housing.clone()}
                                    on_reclaim_housing={on_reclaim_housing.clone()}
                                    on_designate_furniture={on_designate_furniture.clone()}
                                    on_reclaim_furniture={on_reclaim_furniture.clone()}
                                />
                            }
                        } else if *tab == "Enchantments" {
                            html! {
                                <EnchantmentsTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_learn_magic={on_learn_magic.clone()}
                                    on_craft_enchant={on_craft_enchant.clone()}
                                    on_sell_enchant={on_sell_enchant.clone()}
                                />
                            }
                        } else {
                            html! {
                                <AdventureTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_conquer_town={on_conquer_town.clone()}
                                    on_explore_dungeon={on_explore_dungeon.clone()}
                                />
                            }
                        } }
                    </Panel>
                </main>
            </div>

            if !(*toast).is_empty() {
                <div class="toast panel">{ &*toast }</div>
            }
        </div>
    }
}
