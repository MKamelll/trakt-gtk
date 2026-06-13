use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Align, Box, Button, CenterBox, Frame, Grid, Label, ListBox, ListBoxRow, Orientation, Overlay,
    ScrolledWindow, SearchBar, SearchEntry, Spinner, Stack,
    glib::{self, MainContext},
    pango::WrapMode,
    prelude::{BoxExt, ButtonExt, EditableExt, FrameExt, GridExt, ListBoxRowExt, WidgetExt},
};

use crate::traktclient::{SearchResult, Show, TraktClient};

#[derive(Clone)]
pub struct ShowDetailsWidget {
    pub root: Box,
    pub title_label: Label,
    pub year_label: Label,
    pub overview_label: Label,
    pub tagline_label: Label,
    pub first_aired_label: Label,
    pub runtime_label: Label,
    pub network_label: Label,
    pub country_label: Label,
    pub rating_label: Label,
    pub languages_label: Label,
    pub genres_label: Label,
    pub original_title_label: Label,
    pub back_btn: Button,
}

impl ShowDetailsWidget {
    pub fn new() -> Self {
        let vbox = Box::new(Orientation::Vertical, 4);
        let grid = Grid::new();
        grid.set_row_spacing(8);
        grid.set_column_spacing(8);

        let title_label = Label::new(None);
        title_label.set_valign(Align::Start);
        let title_frame = Frame::new(None);
        title_frame.set_child(Some(&title_label));

        let year_label = Label::new(None);
        let year_frame = Frame::new(None);
        year_frame.set_child(Some(&year_label));

        let first_aired_label = Label::new(None);
        let first_aired_frame = Frame::new(Some("First Aired"));
        first_aired_frame.set_child(Some(&first_aired_label));

        let runtime_label = Label::new(None);
        let runtime_frame = Frame::new(Some("Runtime"));
        runtime_frame.set_child(Some(&runtime_label));

        let network_label = Label::new(None);
        let network_frame = Frame::new(Some("Network"));
        network_frame.set_child(Some(&network_label));

        let country_label = Label::new(None);
        let country_frame = Frame::new(Some("Country"));
        country_frame.set_child(Some(&country_label));

        let rating_label = Label::new(None);
        let rating_frame = Frame::new(Some("Rating"));
        rating_frame.set_child(Some(&rating_label));

        let languages_label = Label::new(None);
        let languages_frame = Frame::new(Some("Languages"));
        languages_frame.set_child(Some(&languages_label));

        let genres_label = Label::new(None);
        let genres_frame = Frame::new(Some("Genres"));
        genres_frame.set_child(Some(&genres_label));

        let original_title_label = Label::new(None);
        let original_title_frame = Frame::new(Some("Original Title"));
        original_title_frame.set_child(Some(&original_title_label));

        let overview_label = Label::new(None);
        overview_label.set_wrap(true);
        overview_label.set_hexpand(true);
        overview_label.add_css_class("show-details-widget-overview");

        let overview_frame = Frame::new(None);
        overview_frame.set_hexpand(true);
        overview_frame.set_child(Some(&overview_label));

        let tagline_label = Label::new(None);
        let tagline_frame = Frame::new(None);
        tagline_frame.set_child(Some(&tagline_label));

        //                        col row cs rs
        grid.attach(&title_frame, 0, 0, 2, 1);
        grid.attach(&year_frame, 2, 0, 1, 1);
        grid.attach(&tagline_frame, 0, 1, 3, 1);
        grid.attach(&overview_frame, 0, 2, 1, 6);
        grid.attach(&first_aired_frame, 2, 2, 1, 1);
        grid.attach(&runtime_frame, 2, 3, 1, 1);
        grid.attach(&network_frame, 2, 4, 1, 1);
        grid.attach(&country_frame, 2, 5, 1, 1);
        grid.attach(&rating_frame, 2, 6, 1, 1);
        grid.attach(&languages_frame, 2, 7, 1, 1);
        grid.attach(&genres_frame, 2, 8, 1, 1);
        grid.attach(&original_title_frame, 0, 8, 1, 1);

        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(&grid));

        let back_btn = Button::with_label("Back");
        back_btn.set_halign(Align::End);
        back_btn.set_valign(Align::End);

        vbox.append(&scrolled_window);
        vbox.append(&back_btn);
        vbox.add_css_class("show-details-widget");

        Self {
            root: vbox,
            title_label,
            back_btn,
            year_label,
            overview_label,
            tagline_label,
            first_aired_label,
            runtime_label,
            network_label,
            country_label,
            rating_label,
            languages_label,
            genres_label,
            original_title_label,
        }
    }

    pub fn connect_back<F: Fn() + 'static>(&self, f: F) {
        self.back_btn.connect_clicked(move |_| f());
    }

    pub fn update(&self, show: &Show) -> &Self {
        self.title_label.set_text(&show.title);
        self.year_label.set_text(&match show.year {
            Some(y) => y.to_string(),
            None => "N/A".to_string(),
        });
        self.overview_label.set_text(&match &show.overview {
            Some(o) => o.to_string(),
            None => "N/A".to_string(),
        });

        self.tagline_label.set_text(&match &show.tagline {
            Some(t) => t.to_string(),
            None => "N/A".to_string(),
        });

        self.first_aired_label.set_text(&match &show.first_aired {
            Some(d) => d.to_string(),
            None => "N/A".to_string(),
        });
        self.runtime_label.set_text(&match &show.runtime {
            Some(i) => i.to_string(),
            None => "N/A".to_string(),
        });
        self.network_label.set_text(&match &show.network {
            Some(n) => n.to_string(),
            None => "N/A".to_string(),
        });
        self.country_label.set_text(&match &show.country {
            Some(c) => c.to_string(),
            None => "N/A".to_string(),
        });
        self.rating_label.set_text(&match &show.rating {
            Some(r) => r.to_string(),
            None => "N/A".to_string(),
        });
        self.languages_label.set_text(&match &show.languages {
            Some(r) => r.join(",").to_string(),
            None => "N/A".to_string(),
        });
        self.genres_label.set_text(&match &show.genres {
            Some(r) => r.join(",").to_string(),
            None => "N/A".to_string(),
        });
        self.original_title_label
            .set_text(&match &show.original_title {
                Some(r) => r.to_string(),
                None => "N/A".to_string(),
            });

        self
    }

    pub fn root(&self) -> &Box {
        &self.root
    }
}

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
    pub fn new(trakt_client: Rc<RefCell<TraktClient>>) -> Stack {
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
                                row.set_child(Some(&SearchResultWidget::new(&r)));
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

        stack
    }
}
