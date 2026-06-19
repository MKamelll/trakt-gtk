use std::{cell::RefCell, rc::Rc};

use gtk4::{
    Align, Box, Button, Expander, Grid, Label, ListBox, ListBoxRow, Orientation, PolicyType,
    ScrolledWindow, ToggleButton,
    glib::object::ObjectExt,
    prelude::{BoxExt, ButtonExt, GridExt, ListBoxRowExt, ToggleButtonExt, WidgetExt},
};

use crate::traktclient::{Episode, Season};

#[derive(Clone)]
pub struct SeasonWidget {
    root: Box,
    scroll: ScrolledWindow,
    list: ListBox,
}

impl SeasonWidget {
    pub fn new() -> Self {
        let vbox = Box::new(Orientation::Vertical, 4);
        let list = ListBox::new();
        let scroll = ScrolledWindow::new();
        scroll.set_hexpand(true);
        scroll.set_vexpand(true);
        scroll.set_hscrollbar_policy(PolicyType::Never);
        scroll.set_child(Some(&list));
        vbox.append(&scroll);

        Self {
            root: vbox,
            scroll,
            list,
        }
    }

    pub fn update(&self, episodes: &Vec<Episode>) {
        for episode in episodes {
            let header = Box::new(Orientation::Horizontal, 0);
            header.add_css_class("season-widget-expander-header");

            let title = Label::new(Some(&format!("{} - {}", episode.number, episode.title)));
            title.set_hexpand(true);
            title.set_wrap(true);
            title.set_halign(Align::Start);

            let watched_btn = ToggleButton::new();
            watched_btn.set_icon_name("checkmark-symbolic");
            watched_btn.add_css_class("watched-toggle");

            watched_btn.connect_clicked(|btn| {
                btn.stop_signal_emission_by_name("clicked");
            });

            header.append(&title);
            header.append(&watched_btn);

            let expander = Expander::new(None);
            expander.set_label_widget(Some(&header));

            let content = Grid::new();
            content.add_css_class("season-widget-expander-content");
            content.set_hexpand(true);
            content.set_column_spacing(4);
            content.set_row_spacing(4);
            expander.set_child(Some(&content));

            let overview_label = Label::new(Some(match &episode.overview {
                Some(o) => o,
                None => "N/A",
            }));
            overview_label.set_halign(Align::Start);
            overview_label.set_hexpand(true);
            overview_label.set_wrap(true);
            overview_label.set_lines(4);

            content.attach(&overview_label, 0, 0, 3, 3);

            let runtime_label = Label::new(Some(&format!(
                "Runtime {}",
                match &episode.runtime {
                    Some(r) => r.to_string(),
                    None => "N/A".to_string(),
                }
            )));
            runtime_label.set_halign(Align::Start);

            content.attach(&runtime_label, 3, 0, 1, 1);

            let rating_label = Label::new(Some(&format!(
                "Rating {}/10",
                match &episode.rating {
                    Some(r) => (*r as i32).to_string(),
                    None => "N/A".to_string(),
                }
            )));
            rating_label.set_halign(Align::Start);

            content.attach(&rating_label, 3, 1, 1, 1);

            let first_aired = Label::new(Some(&format!(
                "First Aired {}",
                match &episode.first_aired {
                    Some(d) => d.to_string(),
                    None => "N/A".to_string(),
                }
            )));

            content.attach(&first_aired, 3, 2, 1, 1);

            let row = ListBoxRow::new();
            row.set_child(Some(&expander));
            self.list.append(&row);
        }
    }

    pub fn root(&self) -> &Box {
        &self.root
    }
}
