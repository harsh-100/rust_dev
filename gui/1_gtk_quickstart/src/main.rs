// use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
// we used the Rc and RefCel of rust standard library, to make the counter state that can be used
// for being shared between outer function and closures
use std::cell::RefCell;
use std::rc::Rc;

// App id is a unique id that represents the application uniquely at OS level
const APP_ID: &str = "org.rust_dev.new_gtk";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let gtk_box = Box::builder().orientation(Orientation::Vertical).build();

    // Create `counter` reference counted with RefCell having 0 as initial value
    let counter = Rc::new(RefCell::new(0));
    let label = Label::builder()
        .label(&format!("Count {}", counter.borrow()))
        .build();

    let button = Button::builder()
        .margin_top(10)
        .margin_end(10)
        .margin_start(10)
        .margin_bottom(10)
        .label("Click Me Hero!")
        .build();

    gtk_box.append(&label);
    gtk_box.append(&button);

    // register click event with the button. We used the move because we wanted the label, and
    // counter to be accessible in the closure
    button.connect_clicked(move |_| {
        let mut c = counter.borrow_mut();
        *c += 1;
        label.set_label(&format!("Count {}", c.to_string()));
    });

    // attach click event to the button
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present window / show the window
    window.present();
}
