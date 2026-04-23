use crate::tabs::adventure_tab::AdventureTab;
use crate::tabs::enchantments_tab::EnchantmentsTab;
use crate::tabs::hoard_tab::HoardTab;
use crate::tabs::kobolds_tab::KoboldsTab;
use crate::game::GameState;
use crate::helpers::app_callbacks::create_app_callbacks;
use crate::helpers::rate_helpers::{rate_class, rate_hint};
use crate::ui::{cost_label, format_number, ActionButton, ICON_FOOD, ICON_GOLD, ICON_MANA, NumberFormat, Panel, ResourceStatRow, StatRow, TabButton};
use gloo_timers::callback::Interval;
use yew::prelude::*;

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


    let callbacks = create_app_callbacks(game.clone(), toast.clone());

    let interval_handle = use_mut_ref(|| Option::<Interval>::None);

let onmousedown = {
    let interval_handle = interval_handle.clone();
    let on_click = callbacks.on_click_loot.clone();

    Callback::from(move |_| {
        if interval_handle.borrow().is_some() {
            return;
        }

        let on_click = on_click.clone();

        let handle = Interval::new(100, move || {
            on_click.emit(MouseEvent::new("mousedown").unwrap());
        });

        *interval_handle.borrow_mut() = Some(handle);
    })
};

    let stop = {
        let interval_handle = interval_handle.clone();

        Callback::from(move |_| {
            // drop the interval to stop it
            interval_handle.borrow_mut().take();
        })
    };


    let g = (*game).clone();
    let current_number_style = *number_style;

    let gold_rate = g.gold_per_sec + g.assigned_mining as f64 * 0.6 * g.kobold_efficiency;
    let food_rate = g.assigned_farming as f64 * 0.35 * g.kobold_efficiency - g.kobold_upkeep();
    let mana_rate = if g.mana < g.mana_capacity { g.mana_regen_per_sec } else { 0.0 };

    html! {
        <div class="container">
            <div class="page-header">
                <div>
                    <h1>{"Dragon Hoard"}</h1>
                </div>
                <div class="save-actions">
                    <ActionButton label={"Save".to_string()} onclick={callbacks.on_save.clone()} class={"compact".to_string()} title={"Save your current progress locally.".to_string()} />
                    <ActionButton label={"Load".to_string()} onclick={callbacks.on_load.clone()} class={"compact".to_string()} title={"Load your last saved game.".to_string()} />
                    <ActionButton label={"Export".to_string()} onclick={callbacks.on_export.clone()} class={"compact".to_string()} title={"Export save data to copy and backup.".to_string()} />
                    <ActionButton label={"Import".to_string()} onclick={callbacks.on_import.clone()} class={"compact".to_string()} title={"Import save data from a copied backup.".to_string()} />
                    <ActionButton label={"Reset".to_string()} onclick={callbacks.on_reset.clone()} class={"compact danger".to_string()} title={"Reset the entire game after confirmation.".to_string()} />
                </div>
            </div>

            <div class="main-layout">
                <aside class="resource-sidebar">
                    <details open={true} class="resource-section">
                        <summary class="resource-summary">{"Resources"}</summary>
                        <Panel class={"resource-panel".to_string()}>
                            <ResourceStatRow
                                label={"Gold".to_string()}
                                icon={ICON_GOLD.to_string()}
                                value={format!("{}/{}", format_number(g.gold, current_number_style, 1), format_number(g.gold_capacity, current_number_style, 1))}
                                hint={rate_hint(gold_rate, current_number_style)}
                                hint_class={rate_class(gold_rate)}
                            />
                            <ResourceStatRow
                                label={"Food".to_string()}
                                icon={ICON_FOOD.to_string()}
                                value={format_number(g.food, current_number_style, 1)}
                                hint={rate_hint(food_rate, current_number_style)}
                                hint_class={rate_class(food_rate)}
                            />
                            <ResourceStatRow
                                label={"Mana".to_string()}
                                icon={ICON_MANA.to_string()}
                                value={format!("{}/{}", format_number(g.mana, current_number_style, 1), format_number(g.mana_capacity, current_number_style, 1))}
                                hint={rate_hint(mana_rate, current_number_style)}
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
                        <div
                            onmousedown={onmousedown.clone()}
                            onmouseup={stop.clone()}
                            onmouseleave={stop.clone()}
                        >
                        <ActionButton label={format!("Loot (click) +{}", cost_label(ICON_GOLD, g.click_multiplier, current_number_style))} onclick={callbacks.on_click_loot.clone()} class={"big".to_string()} title={"Click to collect gold from your hoard. Power increases with click multiplier upgrades.".to_string()} />

                        </div>
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
                                    on_buy_training={callbacks.on_buy_training.clone()}
                                    on_buy_vault={callbacks.on_buy_vault.clone()}
                                />
                            }
                        } else if *tab == "Kobolds" {
                            html! {
                                <KoboldsTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_recruit_kobold={callbacks.on_recruit_kobold.clone()}
                                    on_assign_mining={callbacks.on_assign_mining.clone()}
                                    on_unassign_mining={callbacks.on_unassign_mining.clone()}
                                    on_assign_farming={callbacks.on_assign_farming.clone()}
                                    on_unassign_farming={callbacks.on_unassign_farming.clone()}
                                    on_assign_digging={callbacks.on_assign_digging.clone()}
                                    on_unassign_digging={callbacks.on_unassign_digging.clone()}
                                    on_assign_military={callbacks.on_assign_military.clone()}
                                    on_unassign_military={callbacks.on_unassign_military.clone()}
                                    on_assign_research={callbacks.on_assign_research.clone()}
                                    on_unassign_research={callbacks.on_unassign_research.clone()}
                                    on_upgrade_kobolds={callbacks.on_upgrade_kobolds.clone()}
                                    on_designate_housing={callbacks.on_designate_housing.clone()}
                                    on_reclaim_housing={callbacks.on_reclaim_housing.clone()}
                                    on_designate_furniture={callbacks.on_designate_furniture.clone()}
                                    on_reclaim_furniture={callbacks.on_reclaim_furniture.clone()}
                                />
                            }
                        } else if *tab == "Enchantments" {
                            html! {
                                <EnchantmentsTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_learn_magic={callbacks.on_learn_magic.clone()}
                                    on_learn_necromancy={callbacks.on_learn_necromancy.clone()}
                                    on_learn_alchemy={callbacks.on_learn_alchemy.clone()}
                                    on_learn_restoration={callbacks.on_learn_restoration.clone()}
                                    on_learn_elemental={callbacks.on_learn_elemental.clone()}
                                    on_learn_summoning={callbacks.on_learn_summoning.clone()}
                                    on_learn_enchanting={callbacks.on_learn_enchanting.clone()}
                                    on_craft_enchant={callbacks.on_craft_enchant.clone()}
                                    on_sell_enchant={callbacks.on_sell_enchant.clone()}
                                />
                            }
                        } else if *tab == "Adventure" {
                            html! {
                                <AdventureTab
                                    game={g.clone()}
                                    number_style={current_number_style}
                                    on_conquer_town={callbacks.on_conquer_town.clone()}
                                    on_trade_town={callbacks.on_trade_town.clone()}
                                    on_explore_dungeon={callbacks.on_explore_dungeon.clone()}
                                />
                            }
                        } else {
                            html! { <div>{"Unknown tab"}</div> }
                        } }
                    </Panel>
                </main>
            </div>

            if !toast.is_empty() {
                <div class="toast">{ &*toast }</div>
            }
        </div>
    }
}