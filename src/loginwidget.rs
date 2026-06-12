use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Align, Box, Button, Label, Orientation,
    glib::MainContext,
    prelude::{BoxExt, ButtonExt, WidgetExt},
};

use crate::traktclient::TraktClient;

pub struct LoginWidget;

impl LoginWidget {
    pub fn new(trakt_client: Rc<RefCell<TraktClient>>) -> Box {
        let vbox = Box::new(Orientation::Vertical, 8);
        let label = Label::new(Some("You're not logged in"));
        let btn = Button::with_label("login");

        let t1 = Rc::clone(&trakt_client);
        btn.connect_clicked(move |_| {
            let t2 = Rc::clone(&t1);
            MainContext::default().spawn_local(async move {
                t2.borrow_mut().login().await;
            });
        });

        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        vbox.append(&label);
        vbox.append(&btn);

        vbox.add_css_class("login-widget");
        vbox
    }
}
