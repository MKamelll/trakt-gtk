use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Align, Box, Button, CenterBox, Grid, Label, ListBox, ListBoxRow, Orientation, Overlay,
    ScrolledWindow, SearchBar, SearchEntry, Spinner,
    glib::{self, MainContext},
    prelude::{BoxExt, EditableExt, GridExt, ListBoxRowExt, WidgetExt},
};

use crate::traktclient::{SearchResult, Show, TraktClient};

pub struct SearchResultWidget;

impl SearchResultWidget {
    pub fn new(result: &SearchResult) -> Box {
        let vbox = Box::new(Orientation::Vertical, 4);
        vbox.set_hexpand(true);
        let show = &result.show;

        let title_box = Box::new(Orientation::Horizontal, 12);
        let title_label = Label::new(Some(&show.title));
        title_label.set_halign(Align::Start);
        let year = match show.year {
            Some(y) => y.to_string(),
            None => "N/A".to_string(),
        };

        let year_label = Label::new(Some(&year));
        year_label.set_halign(Align::End);
        let overview = match &show.overview {
            Some(o) => o,
            None => &"N/A".to_string(),
        };

        year_label.add_css_class("dim-label");

        title_box.append(&title_label);
        title_box.append(&year_label);

        let overview_label = Label::new(Some(&format!("Overview: {}", overview)));
        overview_label.set_halign(Align::Start);
        overview_label.set_wrap(true);
        overview_label.set_max_width_chars(20);
        overview_label.set_lines(2);
        overview_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        overview_label.add_css_class("dim-label");

        vbox.append(&title_box);
        vbox.append(&overview_label);

        vbox.add_css_class("search-result-widget");
        vbox
    }
}

pub struct SearchWidget;

impl SearchWidget {
    pub fn new(trakt_client: Rc<RefCell<TraktClient>>) -> Box {
        let hbox = Box::new(Orientation::Horizontal, 4);
        let vbox = Box::new(Orientation::Vertical, 8);
        let search_entry = SearchEntry::new();
        let search_bar = SearchBar::new();
        let search_btn = Button::with_label("Search");
        let spinner = Spinner::new();
        let scrolled_window = ScrolledWindow::new();
        let list_box = ListBox::new();
        let overlay = Overlay::new();
        scrolled_window.set_vexpand(true);
        list_box.add_css_class("search-results-listbox");
        scrolled_window.set_hscrollbar_policy(gtk4::PolicyType::Never);

        overlay.set_child(Some(&scrolled_window));
        overlay.add_overlay(&spinner);
        spinner.set_size_request(48, 48);
        spinner.set_halign(Align::Center);
        spinner.set_valign(Align::Center);

        search_entry.set_hexpand(true);
        search_bar.set_hexpand(true);
        search_bar.connect_entry(&search_entry);
        search_bar.set_search_mode(true);
        search_bar.set_child(Some(&search_entry));

        hbox.append(&search_bar);
        hbox.append(&search_btn);

        vbox.append(&hbox);
        scrolled_window.set_child(Some(&list_box));

        vbox.append(&overlay);

        let t1 = Rc::clone(&trakt_client);
        search_entry.connect_activate(glib::clone!(
            #[strong]
            spinner,
            #[weak]
            list_box,
            move |entry| {
                let t2 = Rc::clone(&t1);
                if !entry.text().is_empty() {
                    let entry = entry.clone();
                    MainContext::default().spawn_local(glib::clone!(
                        #[strong]
                        spinner,
                        #[weak]
                        list_box,
                        async move {
                            spinner.start();
                            spinner.set_visible(true);
                            let res = t2.borrow().search(entry.text().to_string()).await;
                            while let Some(c) = list_box.last_child() {
                                list_box.remove(&c);
                            }
                            spinner.stop();
                            spinner.set_visible(false);
                            for r in res {
                                let row = ListBoxRow::new();
                                row.set_child(Some(&SearchResultWidget::new(&r)));
                                list_box.append(&row);
                            }
                        }
                    ));
                }
            }
        ));

        vbox.add_css_class("search-widget");

        vbox
    }
}
