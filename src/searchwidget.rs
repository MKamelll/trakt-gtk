use gtk4::{Box, Button, Orientation, SearchBar, SearchEntry, prelude::BoxExt};

pub struct SearchWidget;

impl SearchWidget {
    pub fn new() -> Box {
        let hbox = Box::new(Orientation::Horizontal, 4);
        let vbox = Box::new(Orientation::Vertical, 8);
        let search_entry = SearchEntry::new();
        let search_bar = SearchBar::new();
        let search_btn = Button::with_label("Search");

        search_bar.connect_entry(&search_entry);
        search_bar.set_search_mode(true);
        search_bar.set_child(Some(&search_entry));

        hbox.append(&search_bar);
        hbox.append(&search_btn);

        vbox.append(&hbox);

        vbox
    }
}
