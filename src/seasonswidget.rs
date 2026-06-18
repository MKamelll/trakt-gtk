use std::collections::HashMap;

use gtk4::{
    Align, Box, Expander, Label, ListBase, ListBox, ListBoxRow, Orientation, Paned, ScrolledWindow,
    Stack,
    prelude::{BoxExt, ListBoxRowExt, WidgetExt},
};

use crate::traktclient::{Episode, Season, Show};

#[derive(Clone)]
pub struct SeasonsWidget {
    root: Paned,
    sidebar_list: ListBox,
    main_content: ListBox,
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

        let list_box2 = ListBox::new();
        let scrolled_window2 = ScrolledWindow::new();
        scrolled_window2.set_vexpand(true);
        scrolled_window2.set_child(Some(&list_box2));

        vbox_sidebar.append(&scrolled_window);
        vbox_main_content.append(&scrolled_window2);

        Self {
            root: paned,
            sidebar_list: list_box,
            main_content: list_box2,
        }
    }

    pub fn update(&self, seasons: &Vec<Season>, episodes: &HashMap<i64, Vec<Episode>>) {
        for season in seasons {
            let row = ListBoxRow::new();
            row.set_child(Some(&Label::new(Some(match &season.title {
                Some(s) => &s,
                None => "N/A",
            }))));
            row.set_halign(Align::Start);
            self.sidebar_list.append(&row);

            if let Some(episodes) = episodes.get(&season.number) {
                for episode in episodes {
                    let expander = Expander::new(Some(&episode.title));
                    let content = Label::new(Some(match &episode.overview {
                        Some(o) => o,
                        None => "N/A",
                    }));
                    expander.set_child(Some(&content));
                    let row = ListBoxRow::new();
                    row.set_child(Some(&expander));
                    row.set_halign(Align::Start);
                    self.main_content.append(&row);
                }
            }
        }
    }

    pub fn root(&self) -> &Paned {
        &self.root
    }
}
