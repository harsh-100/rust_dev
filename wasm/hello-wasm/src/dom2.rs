use std::cell::RefCell;
use std::{borrow::BorrowMut, f64, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, KeyboardEvent, MouseEvent};
#[derive(Clone, Copy)]
#[wasm_bindgen]
struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.fill_rect(self.x, self.y, 5.0, 5.0);
    }

    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

#[wasm_bindgen]
pub fn canvas_1() {
    let window = web_sys::window().expect("window should be there");
    let document = window.document().expect("Document should be there");
    // let body = document.body().expect("body should be there");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    context.set_fill_style(&"red".into());
    use web_sys::console;

    let mut p = Point::new(400.0, 400.0);
    let mut point_test = Rc::new(RefCell::new(Point { x: 10.0, y: 10.0 }));
    // let keydown_closure = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
    //     let key = ev.key();
    //     console::log_1(&JsValue::from(&key));
    //     if key == "w" {
    //         console::log_1(&"clicked".into());
    //         console::log_3(&"key".into(), &JsValue::from(p.x), &p.y.into());
    //     }
    //     let mut pt = point_test.borrow();
    //     // *pt.set(10.0, 10.0);
    //     // *pt.x;
    // }) as Box<dyn FnMut(_)>);

    let closure = Closure::wrap(Box::new(move || {
        // x += 1.0;
        // y += 1.0;
        context.clear_rect(0.0, 0.0, 500.0, 500.0);
        console::log_3(&"closure key".into(), &JsValue::from(p.x), &p.y.into());
        p.draw(&context);
    }) as Box<dyn FnMut()>);

    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000 / 60,
        )
        .expect("Should set interval");

    // window
    //     .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
    //     .expect("Should attach the event");
    // closure.forget();
    // keydown_closure.forget();
}
