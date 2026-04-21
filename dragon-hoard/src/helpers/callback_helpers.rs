use yew::prelude::*;

pub fn make_toast_callback(action: impl Fn() + 'static, toast: UseStateHandle<String>, message: &'static str) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        action();
        toast.set(message.to_string());
    })
}

pub fn make_optional_toast_callback(action: impl Fn() -> Option<String> + 'static, toast: UseStateHandle<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if let Some(message) = action() {
            toast.set(message);
        }
    })
}

pub fn make_event_toast_callback<E: 'static>(action: impl Fn(E) -> String + 'static, toast: UseStateHandle<String>) -> Callback<E> {
    Callback::from(move |event| {
        toast.set(action(event));
    })
}