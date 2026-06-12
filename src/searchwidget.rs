use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Box, Button, Orientation, SearchBar, SearchEntry,
    prelude::{BoxExt, WidgetExt},
};

use crate::traktclient::TraktClient;

pub struct SearchWidget;

impl SearchWidget {
    pub fn new(trakt_client: Rc<RefCell<TraktClient>>) -> Box {
        let hbox = Box::new(Orientation::Horizontal, 4);
        let vbox = Box::new(Orientation::Vertical, 8);
        let search_entry = SearchEntry::new();
        let search_bar = SearchBar::new();
        let search_btn = Button::with_label("Search");

        search_entry.set_hexpand(true);
        search_bar.set_hexpand(true);
        search_bar.connect_entry(&search_entry);
        search_bar.set_search_mode(true);
        search_bar.set_child(Some(&search_entry));

        hbox.append(&search_bar);
        hbox.append(&search_btn);

        vbox.append(&hbox);
        vbox.add_css_class("search-widget");

        vbox
    }
}
