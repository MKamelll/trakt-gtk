use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Application, ApplicationWindow, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::ExitCode,
    prelude::GtkWindowExt,
    style_context_add_provider_for_display,
};
use loginwidget::LoginWidget;
use searchwidget::SearchWidget;
use traktclient::TraktClient;

mod loginwidget;
mod searchwidget;
mod traktclient;

#[tokio::main]
async fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("com.mkamelll")
        .build();

    app.connect_activate(|app| {
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));

        style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("couldn't get the default display"),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(app);
    });

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("trakt")
        .default_height(460)
        .default_width(640)
        .build();

    let trakt_client = Rc::new(RefCell::new(TraktClient::new()));

    let t1 = Rc::clone(&trakt_client);
    let login = LoginWidget::new(t1);

    let t2 = Rc::clone(&trakt_client);
    let search = SearchWidget::new(t2);

    if trakt_client.borrow().is_logged_in() {
        window.set_child(Some(&search))
    } else {
        window.set_child(Some(&login));
    }

    window.present();
}
