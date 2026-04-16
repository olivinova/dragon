use crate::game::GameState;
use crate::ui::{ActionButton, BuyRow, Panel, ICON_FOOD, ICON_GOLD, ICON_MANA, NumberFormat, labeled_cost, paired_cost};
use yew::prelude::*;

/// Kobolds tab UI block.
///
/// Handles kobold recruitment, assignment, and efficiency upgrades.
#[derive(Properties, PartialEq)]
pub struct KoboldsTabProps {
    pub game: GameState,
    pub number_style: NumberFormat,
    pub on_recruit_kobold: Callback<MouseEvent>,
    pub on_assign_mining: Callback<MouseEvent>,
    pub on_unassign_mining: Callback<MouseEvent>,
    pub on_assign_farming: Callback<MouseEvent>,
    pub on_unassign_farming: Callback<MouseEvent>,
    pub on_assign_digging: Callback<MouseEvent>,
    pub on_unassign_digging: Callback<MouseEvent>,
    pub on_upgrade_kobolds: Callback<MouseEvent>,
    pub on_designate_housing: Callback<MouseEvent>,
    pub on_reclaim_housing: Callback<MouseEvent>,
    pub on_designate_furniture: Callback<MouseEvent>,
    pub on_reclaim_furniture: Callback<MouseEvent>,
}

#[function_component(KoboldsTab)]
pub fn kobolds_tab(props: &KoboldsTabProps) -> Html {
    let g = &props.game;
    let kobold_cost = g.kobold_cost();
    let (upgrade_gold, upgrade_mana) = g.kobold_upgrade_cost();

    html! {
        <Panel class={"section-panel".to_string()}>
            <h2 class="section-title">{"Kobold Management"}</h2>
            <BuyRow>
                <ActionButton
                    label={format!(
                        "Recruit Kobold (cost {} + {} 5)",
                        labeled_cost(ICON_GOLD, kobold_cost, props.number_style),
                        ICON_FOOD,
                    )}
                    onclick={props.on_recruit_kobold.clone()}
                    disabled={g.gold < kobold_cost || g.food < 5.0 || g.kobolds >= g.housing_slots}
                />
                <div class="muted">{ format!("Housing: {} kobolds / {} rooms", g.kobolds, g.housing_slots) }</div>
            </BuyRow>

            <BuyRow>
                <ActionButton
                    label={"Designate storage as housing".to_string()}
                    onclick={props.on_designate_housing.clone()}
                    disabled={g.storage_slots == 0}
                    title={"Move one storage slot into housing so you can recruit more kobolds.".to_string()}
                />
                <ActionButton
                    label={"Reclaim housing as storage".to_string()}
                    onclick={props.on_reclaim_housing.clone()}
                    disabled={g.housing_slots == 0}
                    title={"Convert a housing slot back into storage space.".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={"Designate storage as furniture".to_string()}
                    onclick={props.on_designate_furniture.clone()}
                    disabled={g.storage_slots == 0}
                    title={"Create furniture to improve your hoard’s comfort.".to_string()}
                />
                <ActionButton
                    label={"Reclaim furniture as storage".to_string()}
                    onclick={props.on_reclaim_furniture.clone()}
                    disabled={g.furniture_slots == 0}
                    title={"Remove furniture and recover storage space.".to_string()}
                />
            </BuyRow>
            <div class="muted">{ format!("Space: {} / {} units (Housing {}, Storage {}, Furniture {})", g.total_allocated_space(), g.space, g.housing_slots, g.storage_slots, g.furniture_slots) }</div>
            <BuyRow>
                <ActionButton
                    label={format!("Assign mining kobold ({})", g.assigned_mining)}
                    onclick={props.on_assign_mining.clone()}
                    disabled={g.free_kobolds() == 0}
                    title={"Mining kobolds collect gold every second.".to_string()}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_mining.clone()}
                    disabled={g.assigned_mining == 0}
                    title={"Remove a mining kobold from gold collection.".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Assign farming kobold ({})", g.assigned_farming)}
                    onclick={props.on_assign_farming.clone()}
                    disabled={g.free_kobolds() == 0}
                    title={"Farming kobolds grow food every second.".to_string()}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_farming.clone()}
                    disabled={g.assigned_farming == 0}
                    title={"Remove a farming kobold from food production.".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Assign digging kobold ({})", g.assigned_digging)}
                    onclick={props.on_assign_digging.clone()}
                    disabled={g.free_kobolds() == 0}
                    title={"Digging kobolds expand cave space over time.".to_string()}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_digging.clone()}
                    disabled={g.assigned_digging == 0}
                    title={"Remove a digging kobold from cave space expansion.".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!(
                        "Upgrade Kobold Efficiency ({})",
                        paired_cost(upgrade_gold, upgrade_mana, props.number_style),
                    )}
                    onclick={props.on_upgrade_kobolds.clone()}
                    disabled={g.gold < upgrade_gold || g.mana < upgrade_mana || g.magic_level == 0}
                    class={"big".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <div class="muted">{ format!("Efficiency level {} (+{:.0}% output, requires magic)", g.kobold_upgrade_level, (g.kobold_efficiency - 1.0) * 100.0) }</div>
                <div class="muted">{ format!("Food upkeep: {:.1}/s", g.kobold_upkeep()) }</div>
            </BuyRow>
            <div class="muted">{"Farming kobolds grow food, mining kobolds collect gold, and digging kobolds expand cave space."}</div>
        </Panel>
    }
}
