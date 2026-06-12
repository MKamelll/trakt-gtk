use gtk4::{
    Align, Box, Button, Label, Orientation,
    prelude::{BoxExt, WidgetExt},
};

pub struct LoginWidget;

impl LoginWidget {
    pub fn new() -> Box {
        let vbox = Box::new(Orientation::Vertical, 8);
        let label = Label::new(Some("You're not logged in"));
        let btn = Button::with_label("login");

        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        vbox.append(&label);
        vbox.append(&btn);

        vbox
    }
}
