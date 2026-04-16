use crate::game::GameState;
use crate::ui::{ActionButton, BuyRow, Panel, ICON_GOLD, NumberFormat, labeled_cost};
use yew::prelude::*;

/// Hoard tab UI block.
///
/// Contains buttons for training claws and purchasing the vault.
#[derive(Properties, PartialEq)]
pub struct HoardTabProps {
    pub game: GameState,
    pub number_style: NumberFormat,
    pub on_buy_training: Callback<MouseEvent>,
    pub on_buy_vault: Callback<MouseEvent>,
}

#[function_component(HoardTab)]
pub fn hoard_tab(props: &HoardTabProps) -> Html {
    let g = &props.game;
    let training_cost = g.training_cost();
    let vault_cost = GameState::vault_cost();

    html! {
        <Panel class={"section-panel".to_string()}>
            <h2 class="section-title">{"Hoard Upgrades"}</h2>
            <BuyRow>
                <ActionButton
                    label={format!("Train Claws (cost {})", labeled_cost(ICON_GOLD, training_cost, props.number_style))}
                    onclick={props.on_buy_training.clone()}
                    disabled={g.gold < training_cost}
                    title={"Upgrade kobold training to boost click power and passive output.".to_string()}
                />
                <div class="muted">{ format!("Level: {}", g.training_level) }</div>
            </BuyRow>
            if !g.vault_unlocked {
                <BuyRow>
                    <ActionButton
                        label={format!("Buy Treasure Vault (cost {})", labeled_cost(ICON_GOLD, vault_cost, props.number_style))}
                        onclick={props.on_buy_vault.clone()}
                        disabled={g.gold < vault_cost}
                        title={"Purchase a vault to unlock steady passive gold income.".to_string()}
                    />
                    <div class="muted">{"Unlocks a powerful passive income"}</div>
                </BuyRow>
            }
        </Panel>
    }
}
