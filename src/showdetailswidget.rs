use gtk4::{
    Align, Box, Button, Frame, Grid, Label, Notebook, Orientation, ScrolledWindow,
    prelude::{BoxExt, ButtonExt, FrameExt, GridExt, WidgetExt},
};

use crate::{
    seasonswidget::SeasonsWidget,
    showinfoWidget::ShowInfoWidget,
    traktclient::{Season, Show},
};

#[derive(Clone)]
pub struct ShowDetailsWidget {
    root: Notebook,
    info_widget: ShowInfoWidget,
    seasons_widget: SeasonsWidget,
    back_btn: Button,
}

impl ShowDetailsWidget {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        let info_widget = ShowInfoWidget::new();
        let seasons_widget = SeasonsWidget::new();
        let back_btn = Button::with_label("Back");
        back_btn.set_halign(Align::End);
        back_btn.set_valign(Align::End);

        notebook.append_page(info_widget.root(), Some(&Label::new(Some("Info"))));
        notebook.append_page(seasons_widget.root(), Some(&Label::new(Some("Seasons"))));

        Self {
            root: notebook,
            info_widget,
            seasons_widget,
            back_btn,
        }
    }

    pub fn connect_back<F: Fn() + 'static>(&self, f: F) {
        self.back_btn.connect_clicked(move |_| f());
    }

    pub fn update(&self, show: &Show, seasons: &Vec<Season>) {
        self.info_widget.update(show);
        self.seasons_widget.update(seasons);
    }

    pub fn root(&self) -> &Notebook {
        &self.root
    }
}
