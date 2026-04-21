use gloo_timers::callback::Timeout;
use web_sys::OscillatorType;
use yew::prelude::*;

pub const ICON_GOLD: &str = "🪙";
pub const ICON_FOOD: &str = "🍖";
pub const ICON_MANA: &str = "✨";

pub fn format_cost(value: f64, style: NumberFormat) -> String {
    format_number(value, style, 1)
}

pub fn labeled_cost(icon: &str, value: f64, style: NumberFormat) -> String {
    format!("{} {}", icon, format_cost(value, style))
}

pub fn paired_cost(gold: f64, mana: f64, style: NumberFormat) -> String {
    format!("{} {} / {} {}", ICON_GOLD, format_cost(gold, style), ICON_MANA, format_cost(mana, style))
}

pub fn cost_label(icon: &str, value: f64, style: NumberFormat) -> String {
    labeled_cost(icon, value, style)
}

pub fn cost_pair(gold: f64, mana: f64, style: NumberFormat) -> String {
    paired_cost(gold, mana, style)
}

fn play_click_sound() {
    if let Ok(audio_ctx) = web_sys::AudioContext::new() {
        if let (Ok(osc), Ok(gain)) = (audio_ctx.create_oscillator(), audio_ctx.create_gain()) {
            osc.set_type(OscillatorType::Triangle);
            osc.frequency().set_value(420.0);
            gain.gain().set_value(0.12);
            let _ = osc.connect_with_audio_node(&gain);
            let _ = gain.connect_with_audio_node(&audio_ctx.destination());
            let _ = osc.start();
            Timeout::new(90, move || {
                osc.stop().ok();
            })
            .forget();
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberFormat {
    Standard,
    Compact,
    Scientific,
}

pub fn format_number(value: f64, style: NumberFormat, precision: usize) -> String {
    if value.is_nan() || value.is_infinite() {
        return format!("{:.*}", precision, value);
    }

    match style {
        NumberFormat::Standard => format_number_standard(value, precision),
        NumberFormat::Compact => format_number_compact(value, precision),
        NumberFormat::Scientific => format_number_scientific(value, precision),
    }
}

fn format_number_standard(value: f64, precision: usize) -> String {
    let sign = if value < 0.0 { "-" } else { "" };
    let abs = value.abs();
    let formatted = format!("{:.*}", precision, abs);
    let mut parts = formatted.splitn(2, '.');
    let integer_part = parts.next().unwrap_or("0");
    let fractional_part = parts.next();

    let mut reversed = integer_part.chars().rev().peekable();
    let mut with_commas = String::new();
    for idx in 0.. {
        match reversed.next() {
            Some(c) => {
                if idx > 0 && idx % 3 == 0 {
                    with_commas.push(',');
                }
                with_commas.push(c);
            }
            None => break,
        }
    }

    let integer_with_commas: String = with_commas.chars().rev().collect();
    if let Some(frac) = fractional_part {
        if precision > 0 {
            format!("{}{}.{}", sign, integer_with_commas, frac)
        } else {
            format!("{}{}", sign, integer_with_commas)
        }
    } else {
        format!("{}{}", sign, integer_with_commas)
    }
}

fn format_number_compact(value: f64, precision: usize) -> String {
    let sign = if value < 0.0 { "-" } else { "" };
    let abs = value.abs();

    let formatted = if abs >= 1_000_000_000_000.0 {
        format!("{:.*}T", precision, abs / 1_000_000_000_000.0)
    } else if abs >= 1_000_000_000.0 {
        format!("{:.*}B", precision, abs / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{:.*}M", precision, abs / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.*}K", precision, abs / 1_000.0)
    } else {
        format!("{:.*}", precision, abs)
    };

    format!("{}{}", sign, formatted)
}

fn format_number_scientific(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return format!("{:.*}", precision, value);
    }

    let sign = if value < 0.0 { "-" } else { "" };
    let abs = value.abs();
    let exponent = abs.log10().floor() as i32;
    let mantissa = abs / 10f64.powi(exponent);
    let formatted = format!("{:.*}", precision, mantissa);

    format!("{}{}x10^{}", sign, formatted, exponent)
}

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
    #[prop_or_default]
    pub title: String,
}

#[function_component(ActionButton)]
pub fn action_button(props: &ActionButtonProps) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        let disabled = props.disabled;
        Callback::from(move |event| {
            if !disabled {
                play_click_sound();
            }
            onclick.emit(event);
        })
    };

    html! {
        <button
            {onclick}
            disabled={props.disabled}
            class={classes!("button", props.class.clone())}
            title={props.title.clone()}
        >
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
    pub icon: Option<String>,
    #[prop_or_default]
    pub hint: String,
    #[prop_or_default]
    pub hint_class: String,
}

#[function_component(StatRow)]
pub fn stat_row(props: &StatRowProps) -> Html {
    html! {
        <div class={classes!("stat-row", if !props.hint_class.is_empty() { Some(props.hint_class.clone()) } else { None })}>
            <div title={format!("{}: {}", props.label, props.value)}>
                <div class="stat-label-top">
                    if let Some(icon) = &props.icon {
                        <span class="stat-icon">{ icon }</span>
                    }
                    <span class="stat-label">{ &props.label }</span>
                </div>
                if !props.hint.is_empty() {
                    <div class={classes!("stat-hint", props.hint_class.clone())}>{ &props.hint }</div>
                }
            </div>
            <strong>{ &props.value }</strong>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ResourceStatRowProps {
    pub label: String,
    pub icon: String,
    pub value: String,
    #[prop_or_default]
    pub hint: String,
    #[prop_or_default]
    pub hint_class: String,
}

#[function_component(ResourceStatRow)]
pub fn resource_stat_row(props: &ResourceStatRowProps) -> Html {
    html! {
        <StatRow
            label={props.label.clone()}
            value={props.value.clone()}
            icon={Some(props.icon.clone())}
            hint={props.hint.clone()}
            hint_class={props.hint_class.clone()}
        />
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
