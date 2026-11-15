mod core {
    pub use iced_core::{Rectangle, input_method};
}
#[path = "web_input.rs"]
mod web_input;
use core::input_method::Purpose;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Array;

thread_local! {
    static INPUT: RefCell<Option<web_input::Input>> = const { RefCell::new(None) };
    static EVENTS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}
#[wasm_bindgen]
pub fn setup(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let input = web_input::Input::new(canvas, |event| {
        EVENTS.with_borrow_mut(|events| events.push(format!("{event:?}")))
    })?;
    INPUT.with_borrow_mut(|stored| *stored = Some(input));
    Ok(())
}
#[wasm_bindgen]
pub fn enable(gesture: bool, secure: bool) {
    let update = || {
        INPUT.with_borrow_mut(|input| {
            input.as_mut().unwrap().enable(
                core::Rectangle {
                    x: 20.,
                    y: 40.,
                    width: 2.,
                    height: 20.,
                },
                if secure {
                    Purpose::Secure
                } else {
                    Purpose::Normal
                },
            )
        })
    };
    if gesture {
        web_input::gesture(update);
    } else {
        update();
    }
}
#[wasm_bindgen]
pub fn disable() {
    INPUT.with_borrow_mut(|input| input.as_mut().unwrap().disable());
}
#[wasm_bindgen]
pub fn dispose() {
    INPUT.with_borrow_mut(|input| *input = None);
}
#[wasm_bindgen]
pub fn events() -> Array {
    EVENTS.with_borrow_mut(|events| events.drain(..).map(JsValue::from).collect())
}
