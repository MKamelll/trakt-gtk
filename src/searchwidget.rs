use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Align, Box, Button, CenterBox, Frame, Grid, Label, ListBox, ListBoxRow, Orientation, Overlay,
    ScrolledWindow, SearchBar, SearchEntry, Spinner, Stack,
    glib::{self, MainContext},
    pango::WrapMode,
    prelude::{BoxExt, ButtonExt, EditableExt, FrameExt, GridExt, ListBoxRowExt, WidgetExt},
};

use crate::{
    searchresultwidget::SearchResultWidget,
    showdetailswidget::ShowDetailsWidget,
    showinfoWidget::ShowInfoWidget,
    traktclient::{SearchResult, Show, TraktClient},
};

pub struct SearchWidget {
    root: Stack,
}

impl SearchWidget {
    pub fn new(trakt_client: Rc<RefCell<TraktClient>>) -> Self {
        let hbox = Box::new(Orientation::Horizontal, 4);
        let vbox = Box::new(Orientation::Vertical, 8);
        let search_entry = SearchEntry::new();
        let search_bar = SearchBar::new();
        let search_btn = Button::with_label("Search");
        let spinner = Spinner::new();
        let scrolled_window = ScrolledWindow::new();
        let list_box = ListBox::new();
        let overlay = Overlay::new();
        let stack = Stack::new();
        let show_widget = ShowDetailsWidget::new();
        stack.add_named(&vbox, Some("search"));
        stack.add_named(show_widget.root(), Some("show"));
        stack.set_visible_child_name("search");

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

        let results = Rc::new(RefCell::new(Vec::<SearchResult>::new()));

        let t1 = Rc::clone(&trakt_client);
        let r1 = Rc::clone(&results);
        search_entry.connect_activate(glib::clone!(
            #[strong]
            spinner,
            #[weak]
            list_box,
            move |entry| {
                let t2 = Rc::clone(&t1);
                if !entry.text().is_empty() {
                    let entry = entry.clone();
                    let r2 = Rc::clone(&r1);
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
                                row.set_child(Some(SearchResultWidget::new(&r).root()));
                                list_box.append(&row);
                                r2.borrow_mut().push(r.clone());
                            }
                        }
                    ));
                }
            }
        ));

        let r2 = Rc::clone(&results);
        list_box.connect_row_activated(glib::clone!(
            #[weak]
            stack,
            #[strong]
            show_widget,
            move |_, row| {
                let show = &r2.borrow()[row.index() as usize].show;
                show_widget.update(show);
                stack.set_visible_child_name("show");
            }
        ));

        show_widget.connect_back(glib::clone!(
            #[weak]
            stack,
            move || {
                stack.set_visible_child_name("search");
            }
        ));

        vbox.add_css_class("search-widget");

        Self { root: stack }
    }

    pub fn root(&self) -> &Stack {
        &self.root
    }
}
