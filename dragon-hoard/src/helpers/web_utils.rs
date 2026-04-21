use web_sys::Window;

pub fn app_window() -> Window {
    web_sys::window().expect("window should be available")
}

pub fn alert_dialog(message: &str) {
    let _ = app_window().alert_with_message(message);
}

pub fn confirm_dialog(message: &str) -> bool {
    app_window().confirm_with_message(message).unwrap_or(false)
}

pub fn prompt_dialog(message: &str, default: &str) -> Option<String> {
    app_window().prompt_with_message_and_default(message, default).ok().flatten()
}