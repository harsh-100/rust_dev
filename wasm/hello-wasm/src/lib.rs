#[allow(dead_code)]
mod dom1;
use crate::dom1::register_button_click;
mod dom2;

use crate::dom2::canvas_1;

mod utils;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[allow(unused_variables)]
#[allow(dead_code)]
#[wasm_bindgen]
pub fn greet() {
    use web_sys::console;
    let number_value: JsValue = JsValue::from(1);
    console::log_1(&"Hello, wasm-game-of-life!".into());
    console::log_2(&"Here is a number".into(), &number_value);

    // add_dom_elements_1()
    register_button_click();
}

#[wasm_bindgen]
pub fn render() {
    register_button_click();
}
