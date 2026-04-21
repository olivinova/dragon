use crate::ui::{format_number, NumberFormat};

pub fn rate_hint(rate: f64, style: NumberFormat) -> String {
    let text = format_number(rate, style, 1);
    if rate >= 0.0 {
        format!("+{}/s", text)
    } else {
        format!("{}/s", text)
    }
}

pub fn rate_class(rate: f64) -> String {
    if rate >= 0.0 {
        "positive".to_string()
    } else {
        "negative".to_string()
    }
}