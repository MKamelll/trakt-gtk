use gtk4::{
    Align, Box, Label, ListBox, ListBoxRow, Orientation, Paned, ScrolledWindow,
    prelude::{BoxExt, ListBoxRowExt, WidgetExt},
};

use crate::traktclient::{Season, Show};

#[derive(Clone)]
pub struct SeasonsWidget {
    root: Paned,
    sidebar_list: ListBox,
}

impl SeasonsWidget {
    pub fn new() -> Self {
        let vbox_sidebar = Box::new(Orientation::Vertical, 4);
        vbox_sidebar.add_css_class("seasons-widget-sidebar");
        let vbox_main_content = Box::new(Orientation::Vertical, 4);
        vbox_main_content.add_css_class("seasons-widget-maincontent");
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_start_child(Some(&vbox_sidebar));
        paned.set_end_child(Some(&vbox_main_content));
        paned.set_position(250);

        let list_box = ListBox::new();
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(&list_box));

        let label2 = Label::new(Some("Seasons content"));
        let scrolled_window2 = ScrolledWindow::new();
        scrolled_window2.set_vexpand(true);
        scrolled_window2.set_child(Some(&label2));

        vbox_sidebar.append(&scrolled_window);
        vbox_main_content.append(&scrolled_window2);

        Self {
            root: paned,
            sidebar_list: list_box,
        }
    }

    pub fn update(&self, seasons: &Vec<Season>) {
        for season in seasons {
            let row = ListBoxRow::new();
            row.set_child(Some(&Label::new(Some(match &season.title {
                Some(s) => &s,
                None => "N/A",
            }))));
            row.set_halign(Align::Start);
            self.sidebar_list.append(&row)
        }
    }

    pub fn root(&self) -> &Paned {
        &self.root
    }
}
