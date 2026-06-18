use gtk4::{
    Box, Expander, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    prelude::{BoxExt, ListBoxRowExt, WidgetExt},
};

use crate::traktclient::{Episode, Season};

#[derive(Clone)]
pub struct SeasonWidget {
    root: ScrolledWindow,
    list: ListBox,
}

impl SeasonWidget {
    pub fn new() -> Self {
        let list = ListBox::new();
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(&list));

        Self {
            root: scrolled_window,
            list,
        }
    }

    pub fn update(&self, episodes: &Vec<Episode>) {
        for episode in episodes {
            let expander = Expander::new(Some(&episode.title));
            let content = Box::new(Orientation::Vertical, 4);
            expander.set_child(Some(&content));
            let title_label = Label::new(Some(&episode.title));
            content.append(&title_label);

            let row = ListBoxRow::new();
            row.set_child(Some(&expander));
            self.list.append(&row);
        }
    }

    pub fn root(&self) -> &ScrolledWindow {
        &self.root
    }
}
