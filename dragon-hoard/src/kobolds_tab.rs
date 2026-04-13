use crate::game::GameState;
use crate::ui::{ActionButton, BuyRow, Panel};
use yew::prelude::*;

/// Kobolds tab UI block.
///
/// Handles kobold recruitment, assignment, and efficiency upgrades.
#[derive(Properties, PartialEq)]
pub struct KoboldsTabProps {
    pub game: GameState,
    pub on_recruit_kobold: Callback<MouseEvent>,
    pub on_assign_mining: Callback<MouseEvent>,
    pub on_unassign_mining: Callback<MouseEvent>,
    pub on_assign_farming: Callback<MouseEvent>,
    pub on_unassign_farming: Callback<MouseEvent>,
    pub on_assign_digging: Callback<MouseEvent>,
    pub on_unassign_digging: Callback<MouseEvent>,
    pub on_upgrade_kobolds: Callback<MouseEvent>,
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
                    label={format!("Recruit Kobold (cost {:.2}g + 5 food)", kobold_cost)}
                    onclick={props.on_recruit_kobold.clone()}
                    disabled={g.gold < kobold_cost || g.food < 5.0 || g.kobolds >= g.housing}
                />
                <div class="muted">{ format!("Housing: {} kobolds / {} rooms", g.kobolds, g.housing) }</div>
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Assign mining kobold ({})", g.assigned_mining)}
                    onclick={props.on_assign_mining.clone()}
                    disabled={g.free_kobolds() == 0}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_mining.clone()}
                    disabled={g.assigned_mining == 0}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Assign farming kobold ({})", g.assigned_farming)}
                    onclick={props.on_assign_farming.clone()}
                    disabled={g.free_kobolds() == 0}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_farming.clone()}
                    disabled={g.assigned_farming == 0}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Assign digging kobold ({})", g.assigned_digging)}
                    onclick={props.on_assign_digging.clone()}
                    disabled={g.free_kobolds() == 0}
                />
                <ActionButton
                    label={"Unassign".to_string()}
                    onclick={props.on_unassign_digging.clone()}
                    disabled={g.assigned_digging == 0}
                />
            </BuyRow>
            <BuyRow>
                <ActionButton
                    label={format!("Upgrade Kobold Efficiency ({:.0}g / {:.0}m)", upgrade_gold, upgrade_mana)}
                    onclick={props.on_upgrade_kobolds.clone()}
                    disabled={g.gold < upgrade_gold || g.mana < upgrade_mana || g.magic_level == 0}
                    class={"big".to_string()}
                />
            </BuyRow>
            <BuyRow>
                <div class="muted">{ format!("Efficiency level {} (+{:.0}% output, requires magic)", g.kobold_upgrade_level, (g.kobold_efficiency - 1.0) * 100.0) }</div>
                <div class="muted">{ format!("Food upkeep: {:.1}/s", g.kobold_upkeep()) }</div>
            </BuyRow>
            <div class="muted">{"Farming kobolds grow food, mining kobolds collect gold, and digging kobolds expand cave housing."}</div>
        </Panel>
    }
}
