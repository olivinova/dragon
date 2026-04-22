use crate::game::GameState;
use crate::ui::{ActionButton, BuyRow, cost_label, cost_pair, ICON_GOLD, NumberFormat, Panel};
use yew::prelude::*;

/// Enchantments tab UI block.
///
/// Displays magic progression and crafted enchantment cards.
#[derive(Properties, PartialEq)]
pub struct EnchantmentsTabProps {
    pub game: GameState,
    pub number_style: NumberFormat,
    pub on_learn_magic: Callback<MouseEvent>,
    pub on_learn_necromancy: Callback<MouseEvent>,
    pub on_learn_alchemy: Callback<MouseEvent>,
    pub on_learn_restoration: Callback<MouseEvent>,
    pub on_learn_elemental: Callback<MouseEvent>,
    pub on_learn_summoning: Callback<MouseEvent>,
    pub on_learn_enchanting: Callback<MouseEvent>,
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
                    label={format!("Study Magic (cost {})", cost_label(ICON_GOLD, magic_cost, props.number_style))}
                    onclick={props.on_learn_magic.clone()}
                    disabled={g.gold < magic_cost}
                    title={"Study arcane knowledge to increase magic level. Unlocks enchantments, kobold upgrades, and boosts conquest strength.".to_string()}
                />
                <div class="muted">{ format!("Magic Level: {}", g.magic_level) }</div>
            </BuyRow>
            if g.magic_level > 0 {
                <div>
                    <h3>{"Magic Specializations"}</h3>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Necromancy (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.necromancy_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_necromancy.clone()}
                            disabled={g.gold < g.specialization_cost(g.necromancy_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Unlock necromantic magic. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.necromancy_level) }</div>
                    </BuyRow>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Alchemy (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.alchemy_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_alchemy.clone()}
                            disabled={g.gold < g.specialization_cost(g.alchemy_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Master alchemical arts. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.alchemy_level) }</div>
                    </BuyRow>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Restoration (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.restoration_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_restoration.clone()}
                            disabled={g.gold < g.specialization_cost(g.restoration_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Learn restorative magic. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.restoration_level) }</div>
                    </BuyRow>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Elemental (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.elemental_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_elemental.clone()}
                            disabled={g.gold < g.specialization_cost(g.elemental_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Harness elemental forces. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.elemental_level) }</div>
                    </BuyRow>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Summoning (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.summoning_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_summoning.clone()}
                            disabled={g.gold < g.specialization_cost(g.summoning_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Master summoning magic. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.summoning_level) }</div>
                    </BuyRow>
                    <BuyRow>
                        <ActionButton
                            label={format!("Study Enchanting (cost {}, {} researchers)", cost_label(ICON_GOLD, g.specialization_cost(g.enchanting_level), props.number_style), g.specialization_research_cost())}
                            onclick={props.on_learn_enchanting.clone()}
                            disabled={g.gold < g.specialization_cost(g.enchanting_level) || g.assigned_research < g.specialization_research_cost()}
                            title={format!("Deepen enchantment crafting knowledge. Requires {} research kobolds to progress. Current total levels: {}", g.specialization_research_cost(), g.total_specialization_levels())}
                        />
                        <div class="muted">{ format!("Level: {}", g.enchanting_level) }</div>
                    </BuyRow>
                </div>
            }
            <BuyRow>
                <ActionButton
                    label={format!("Forging Enchantment ({})", cost_pair(enchant_cost.0, enchant_cost.1, props.number_style))}
                    onclick={props.on_craft_enchant.clone()}
                    disabled={g.gold < enchant_cost.0 || g.mana < enchant_cost.1}
                    title={"Forge a powerful enchantment to gain permanent passive bonuses. Each enchantment provides unique benefits like extra gold/sec or defensive power.".to_string()}
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
                            <button class="button" onclick={Callback::from(move |_| sell_cb.emit(i))} title={"Sell this enchantment for a partial gold refund.".to_string()}>{"Sell"}</button>
                        </div>
                    }
                }) }
            </div>
        </Panel>
    }
}
