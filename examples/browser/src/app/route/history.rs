//! Own the popstate listener for exactly the lifetime of its subscription.

use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::{StreamExt, stream};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, Window, console};

use crate::Message;

struct Listener {
    window: Window,
    callback: Closure<dyn FnMut(Event)>,
}

impl Drop for Listener {
    fn drop(&mut self) {
        let _ = self.window.remove_event_listener_with_callback(
            "popstate",
            self.callback.as_ref().unchecked_ref(),
        );
    }
}

pub(super) fn location() -> Option<String> {
    web_sys::window()?.location().pathname().ok()
}

pub(super) fn push(path: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let result = window
        .history()
        .and_then(|history| history.push_state_with_url(&JsValue::NULL, "", Some(path)));
    if let Err(error) = result {
        console::error_2(&"Could not update browser history".into(), &error);
    }
}

pub(super) fn subscription() -> Subscription<Message> {
    Subscription::run(|| {
        let (sender, receiver) = mpsc::unbounded();
        let listener = web_sys::window().and_then(|window| {
            let callback = Closure::new(move |_: Event| {
                if let Some(path) = location() {
                    let _ = sender.unbounded_send(Message::History(path));
                }
            });
            window
                .add_event_listener_with_callback("popstate", callback.as_ref().unchecked_ref())
                .ok()?;
            Some(Listener { window, callback })
        });
        stream::unfold(
            (listener, receiver),
            |(listener, mut receiver)| async move {
                receiver
                    .next()
                    .await
                    .map(|message| (message, (listener, receiver)))
            },
        )
    })
}
