use crate::game::GameState;
use crate::ui::{ActionButton, cost_label, cost_pair, ICON_GOLD, NumberFormat, Panel};
use yew::prelude::*;

/// Adventure tab UI block.
///
/// Shows nearby towns and dungeon exploration options.
#[derive(Properties, PartialEq)]
pub struct AdventureTabProps {
    pub game: GameState,
    pub number_style: NumberFormat,
    pub on_conquer_town: Callback<usize>,
    pub on_trade_town: Callback<usize>,
    pub on_explore_dungeon: Callback<usize>,
}

#[function_component(AdventureTab)]
pub fn adventure_tab(props: &AdventureTabProps) -> Html {
    let g = &props.game;

    html! {
        <>
            <Panel class={"section-panel".to_string()}>
                <h2 class="section-title">{"Nearby Towns"}</h2>
                { for (0..10.min(g.towns.len())).map(|i| {
                    let t = &g.towns[i];
                    let conquer_cb = props.on_conquer_town.clone();
                    let trade_cb = props.on_trade_town.clone();
                    let town_cost = g.town_cost(i);
                    let conquer_label = if t.conquered {
                        "Owned".to_string()
                    } else {
                        format!("Conquer ({})", cost_label(ICON_GOLD, town_cost, props.number_style))
                    };

                    html! {
                        <div class="buy-row">
                            <div style="flex-grow: 1;">
                                <strong>{ &t.name }</strong>
                                <div class="muted">{ format!("difficulty {:.1}", t.difficulty) }</div>
                                <div class="muted" style="font-size: 0.85em;">
                                    {"Trade: "} {&t.wants_amount} {&t.wants} {" ↔ "} {&t.offers_amount} {&t.offers}
                                </div>
                            </div>
                            <div style="display: flex; gap: 8px;">
                                <ActionButton
                                    label={conquer_label}
                                    onclick={Callback::from(move |_| conquer_cb.emit(i))}
                                    disabled={t.conquered || g.gold < town_cost}
                                    title={"Conquer this town for gold rewards and new passive income.".to_string()}
                                />
                                <ActionButton
                                    label={"Trade".to_string()}
                                    onclick={Callback::from(move |_| trade_cb.emit(i))}
                                    disabled=false
                                    title={"Trade resources with this town.".to_string()}
                                />
                            </div>
                        </div>
                    }
                }) }
            </Panel>

            <Panel class={"section-panel".to_string()}>
                <h2 class="section-title">{"Dungeons"}</h2>
                { for g.dungeons.iter().enumerate().map(|(i, d)| {
                    let cb = props.on_explore_dungeon.clone();
                    let (gold_cost, mana_req) = g.dungeon_cost(i);
                    let label = if d.cleared {
                        "Cleared".to_string()
                    } else {
                        format!("Explore ({})", cost_pair(gold_cost, mana_req, props.number_style))
                    };

                    html! {
                        <div class="buy-row">
                            <div>
                                <strong>{ &d.name }</strong>
                                <div class="muted">{ format!("difficulty {:.1}", d.difficulty) }</div>
                            </div>
                            <div class="muted">{ if d.cleared { "Cleared" } else { "Unexplored" } }</div>
                            <ActionButton
                                label={label}
                                onclick={Callback::from(move |_| cb.emit(i))}
                                disabled={d.cleared || g.gold < gold_cost || g.mana < mana_req}
                                title={"Explore the dungeon to earn gold and mana rewards if you meet the cost.".to_string()}
                            />
                        </div>
                    }
                }) }
            </Panel>
        </>
    }
}
