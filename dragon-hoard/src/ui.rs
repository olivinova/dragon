use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BuyRowProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(BuyRow)]
pub fn buy_row(props: &BuyRowProps) -> Html {
    html! {
        <div class="buy-row">{ for props.children.iter() }</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionButtonProps {
    pub label: String,
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
}

#[function_component(ActionButton)]
pub fn action_button(props: &ActionButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} disabled={props.disabled} class={classes!(props.class.clone())}>
            { &props.label }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    html! {
        <section class={classes!("panel", props.class.clone())}>
            { for props.children.iter() }
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatRowProps {
    pub label: String,
    pub value: String,
    #[prop_or_default]
    pub hint: String,
    #[prop_or_default]
    pub hint_class: String,
}

#[function_component(StatRow)]
pub fn stat_row(props: &StatRowProps) -> Html {
    html! {
        <div class="stat-row">
            <div>
                <span class="stat-label">{ &props.label }</span>
                if !props.hint.is_empty() {
                    <div class={classes!("stat-hint", props.hint_class.clone())}>{ &props.hint }</div>
                }
            </div>
            <strong>{ &props.value }</strong>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabButtonProps {
    pub label: String,
    pub active: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(TabButton)]
pub fn tab_button(props: &TabButtonProps) -> Html {
    html! {
        <button class={classes!("tab-button", if props.active { Some("active") } else { None })} onclick={props.onclick.clone()}>
            { &props.label }
        </button>
    }
}
