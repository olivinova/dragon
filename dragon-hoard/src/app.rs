use crate::adventure_tab::AdventureTab;
use crate::enchantments_tab::EnchantmentsTab;
use crate::game::GameState;
use crate::hoard_tab::HoardTab;
use crate::kobolds_tab::KoboldsTab;
use crate::ui::{ActionButton, Panel, StatRow, TabButton};
use gloo_timers::callback::Interval;
use yew::prelude::*;

fn mutate_game<E: 'static>(
    game: UseStateHandle<GameState>,
    action: impl Fn(&mut GameState) + 'static,
) -> Callback<E> {
    Callback::from(move |_| {
        game.set({
            let mut g = (*game).clone();
            action(&mut g);
            g.save();
            g
        });
    })
}

fn mutate_game_index(
    game: UseStateHandle<GameState>,
    action: impl Fn(&mut GameState, usize) + 'static,
) -> Callback<usize> {
    Callback::from(move |idx| {
        game.set({
            let mut g = (*game).clone();
            action(&mut g, idx);
            g.save();
            g
        });
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

    // TODO: Consider moving these action callbacks into a dedicated helper module
    // once the number of game actions grows beyond the current set.
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

    let on_save = {
        let game = game.clone();
        Callback::from(move |_| {
            (*game).save();
        })
    };

    let on_load = {
        let game = game.clone();
        Callback::from(move |_| {
            if let Some(loaded) = GameState::load() {
                game.set(loaded);
            }
        })
    };

    let on_reset = mutate_game(game.clone(), |g| g.reset());

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

    let gold_rate = g.gold_per_sec + g.assigned_mining as f64 * 0.6 * g.kobold_efficiency;
    let food_rate = g.assigned_farming as f64 * 0.35 * g.kobold_efficiency - g.kobold_upkeep();
    let mana_rate = if g.mana < g.mana_capacity { g.mana_regen_per_sec } else { 0.0 };

    let rate_hint = |rate: f64| {
        if rate >= 0.0 {
            format!("+{:.1}/s", rate)
        } else {
            format!("{:.1}/s", rate)
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
                    <ActionButton label={"Save".to_string()} onclick={on_save.clone()} class={"compact".to_string()} />
                    <ActionButton label={"Load".to_string()} onclick={on_load.clone()} class={"compact".to_string()} />
                    <ActionButton label={"Reset".to_string()} onclick={on_reset.clone()} class={"compact danger".to_string()} />
                </div>
            </div>

            <div class="main-layout">
                <aside class="resource-sidebar">
                    <Panel class={"resource-panel".to_string()}>
                        <h2 class="panel-title">{"Resources"}</h2>
                        <StatRow
                            label={"Gold".to_string()}
                            value={format!("{:.1}", g.gold)}
                            hint={rate_hint(gold_rate)}
                            hint_class={rate_class(gold_rate)}
                        />
                        <StatRow
                            label={"Food".to_string()}
                            value={format!("{:.1}", g.food)}
                            hint={rate_hint(food_rate)}
                            hint_class={rate_class(food_rate)}
                        />
                        <StatRow
                            label={"Mana".to_string()}
                            value={format!("{:.1}/{:.1}", g.mana, g.mana_capacity)}
                            hint={rate_hint(mana_rate)}
                            hint_class={rate_class(mana_rate)}
                        />
                    </Panel>
                    <Panel class={"resource-panel".to_string()}>
                        <h2 class="panel-title">{"Kobolds"}</h2>
                        <StatRow label={"Population".to_string()} value={format!("{}/{}", g.kobolds, g.housing)} />
                        <StatRow label={"Free".to_string()} value={format!("{}", g.free_kobolds())} />
                        <StatRow label={"Upkeep".to_string()} value={format!("{:.1}/s", g.kobold_upkeep())} />
                    </Panel>
                </aside>

                <main class="right-panel">
                    <Panel class={"toolbar".to_string()}>
                        <ActionButton label={"Loot (click) +1".to_string()} onclick={on_click_loot.clone()} class={"big".to_string()} />
/*                         <div class="controls gap-large">
                            <ActionButton label={"Save".to_string()} onclick={on_save.clone()} class={"compact".to_string()} />
                            <ActionButton label={"Load".to_string()} onclick={on_load.clone()} class={"compact".to_string()} />
                            <ActionButton label={"Reset".to_string()} onclick={on_reset.clone()} class={"compact danger".to_string()} />
                        </div> */
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
                                    on_buy_training={on_buy_training.clone()}
                                    on_buy_vault={on_buy_vault.clone()}
                                />
                            }
                        } else if *tab == "Kobolds" {
                            html! {
                                <KoboldsTab
                                    game={g.clone()}
                                    on_recruit_kobold={on_recruit_kobold.clone()}
                                    on_assign_mining={on_assign_mining.clone()}
                                    on_unassign_mining={on_unassign_mining.clone()}
                                    on_assign_farming={on_assign_farming.clone()}
                                    on_unassign_farming={on_unassign_farming.clone()}
                                    on_assign_digging={on_assign_digging.clone()}
                                    on_unassign_digging={on_unassign_digging.clone()}
                                    on_upgrade_kobolds={on_upgrade_kobolds.clone()}
                                />
                            }
                        } else if *tab == "Enchantments" {
                            html! {
                                <EnchantmentsTab
                                    game={g.clone()}
                                    on_learn_magic={on_learn_magic.clone()}
                                    on_craft_enchant={on_craft_enchant.clone()}
                                    on_sell_enchant={on_sell_enchant.clone()}
                                />
                            }
                        } else {
                            html! {
                                <AdventureTab
                                    game={g.clone()}
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
