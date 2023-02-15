use wasm_bindgen::{prelude::*, JsCast};

use crate::alert;

#[wasm_bindgen]
pub fn add_dom_elements_1() {
    use web_sys::console;
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let paragraph = document.create_element("p");

    match paragraph {
        Ok(paragraph) => {
            paragraph.set_inner_html("Hello, from wasm!");
            body.append_child(&paragraph)
                .expect("should append a p to the body");
        }
        Err(_) => {
            console::log_1(&"Error creating paragraph".into());
        }
    }

    for i in 1..=3 {
        let test_paragraph = document
            .create_element("span")
            .expect("Should create a span");
        let index_to_string = i.to_string();
        test_paragraph.set_text_content(Some(&index_to_string));
        body.append_child(&test_paragraph)
            .expect("Should append child element");
    }
}

#[wasm_bindgen]
pub fn show_alert() {
    alert("Hello Alert");
}

#[wasm_bindgen]
pub fn register_button_click() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let button = document
        .create_element("button")
        .expect("Should create Button");

    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
        show_alert();
    });

    // use the js_sys::Function::bind method to bind the show_alert function to the button click event

    button.set_inner_html("Click Me To Show Alert");
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Should show alert");
    closure.forget();
    body.append_child(&button)
        .expect("should add button to the body");
    // declare a js-sys function
}
