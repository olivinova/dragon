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
                <div class="muted">{ format!("Conquered: {} (need {} military for upkeep)", g.conquered_towns, g.conquered_towns) }</div>
                { for (0..g.towns.len()).filter_map(|i| {
                    let t = &g.towns[i];
                    if t.conquered {
                        return None;
                    }
                    let conquer_cb = props.on_conquer_town.clone();
                    let trade_cb = props.on_trade_town.clone();
                    let town_cost = g.town_cost(i);
                    let conquer_label = format!("Conquer ({})", cost_label(ICON_GOLD, town_cost, props.number_style));

                    Some(html! {
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
                                    disabled={g.gold < town_cost}
                                    title={format!("Conquer this town to gain {:.1} gold/sec passive income and +1000 space soft cap. Requires military and strength. Needs 1 military kobold per town to maintain benefits.", t.reward_gold_per_sec)}
                                />
                                <ActionButton
                                    label={"Trade".to_string()}
                                    onclick={Callback::from(move |_| trade_cb.emit(i))}
                                    disabled=false
                                    title={format!("Exchange {} {} for {} {} with this town.", t.wants_amount, t.wants, t.offers_amount, t.offers)}
                                />
                            </div>
                        </div>
                    })
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
                                title={format!("Explore this dungeon to earn {:.0} gold and unique enchantments. Requires strength, gold cost, and mana. You can find legendary artifacts here.", d.reward_gold)}
                            />
                        </div>
                    }
                }) }
            </Panel>
        </>
    }
}
