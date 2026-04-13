use crate::game::GameState;
use crate::ui::{ActionButton, BuyRow, Panel};
use yew::prelude::*;

/// Enchantments tab UI block.
///
/// Displays magic progression and crafted enchantment cards.
#[derive(Properties, PartialEq)]
pub struct EnchantmentsTabProps {
    pub game: GameState,
    pub on_learn_magic: Callback<MouseEvent>,
    pub on_craft_enchant: Callback<MouseEvent>,
    pub on_sell_enchant: Callback<usize>,
}

#[function_component(EnchantmentsTab)]
pub fn enchantments_tab(props: &EnchantmentsTabProps) -> Html {
    let g = &props.game;
    let magic_cost = g.magic_cost();
    let enchant_cost = g.enchant_cost();

    html! {
        <Panel class={"section-panel".to_string()}>
            <h2 class="section-title">{"Enchantments"}</h2>
            <BuyRow>
                <ActionButton
                    label={format!("Study Magic (cost {:.0})", magic_cost)}
                    onclick={props.on_learn_magic.clone()}
                    disabled={g.gold < magic_cost}
                />
                <div class="muted">{ format!("Magic Level: {}", g.magic_level) }</div>
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Forging Enchantment ({:.0}g / {:.0}m)", enchant_cost.0, enchant_cost.1)}
                    onclick={props.on_craft_enchant.clone()}
                    disabled={g.gold < enchant_cost.0 || g.mana < enchant_cost.1}
                />
                <div class="muted">{"Use mana and gold to craft rarer items."}</div>
            </BuyRow>
            <div class="enchant-list">
                { for g.enchantments.iter().enumerate().map(|(i, e)| {
                    let sell_cb = props.on_sell_enchant.clone();
                    html! {
                        <div class="enchant-card">
                            <div class="enchant-header">
                                <strong>{ &e.name }</strong>
                                <span>{ format!("+{:.1} power", e.power) }</span>
                            </div>
                            <div class="enchant-meta">{ format!("{} — value {:.0}", e.kind, e.value) }</div>
                            <div class="muted">{ &e.effect }</div>
                            <button onclick={Callback::from(move |_| sell_cb.emit(i))}>{"Sell"}</button>
                        </div>
                    }
                }) }
            </div>
        </Panel>
    }
}
