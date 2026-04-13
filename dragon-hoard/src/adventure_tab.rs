use crate::game::GameState;
use crate::ui::{ActionButton, Panel};
use yew::prelude::*;

/// Adventure tab UI block.
///
/// Shows nearby towns and dungeon exploration options.
#[derive(Properties, PartialEq)]
pub struct AdventureTabProps {
    pub game: GameState,
    pub on_conquer_town: Callback<usize>,
    pub on_explore_dungeon: Callback<usize>,
}

#[function_component(AdventureTab)]
pub fn adventure_tab(props: &AdventureTabProps) -> Html {
    let g = &props.game;

    html! {
        <>
            <Panel class={"section-panel".to_string()}>
                <h2 class="section-title">{"Nearby Towns"}</h2>
                { for g.towns.iter().enumerate().map(|(i, t)| {
                    let cb = props.on_conquer_town.clone();
                    let town_cost = g.town_cost(i);
                    let label = if t.conquered {
                        "Owned".to_string()
                    } else {
                        format!("Conquer (cost {:.0})", town_cost)
                    };

                    html! {
                        <div class="buy-row">
                            <div>
                                <strong>{ &t.name }</strong>
                                <div class="muted">{ format!("difficulty {:.1}", t.difficulty) }</div>
                            </div>
                            <div class="muted">{ if t.conquered { "Conquered" } else { "Free" } }</div>
                            <ActionButton
                                label={label}
                                onclick={Callback::from(move |_| cb.emit(i))}
                                disabled={t.conquered || g.gold < town_cost}
                            />
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
                        format!("Explore ({:.0}g/{:.0}m)", gold_cost, mana_req)
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
                            />
                        </div>
                    }
                }) }
            </Panel>
        </>
    }
}
