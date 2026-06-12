use gtk4::{
    Application, ApplicationWindow,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::ExitCode,
    prelude::GtkWindowExt,
};
use loginwidget::LoginWidget;
use searchwidget::SearchWidget;

mod loginwidget;
mod searchwidget;
mod traktclient;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("com.mkamelll")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("trakt")
        .default_height(460)
        .default_width(640)
        .build();

    let login = LoginWidget::new();
    let search = SearchWidget::new();

    window.set_child(Some(&login));
    window.present();
}
