use gtk4::{
    Align, Box, Expander, Grid, Label, ListBox, ListBoxRow, Orientation, PolicyType,
    ScrolledWindow,
    prelude::{BoxExt, GridExt, ListBoxRowExt, WidgetExt},
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
        scrolled_window.set_hscrollbar_policy(PolicyType::Never);
        scrolled_window.set_child(Some(&list));

        Self {
            root: scrolled_window,
            list,
        }
    }

    pub fn update(&self, episodes: &Vec<Episode>) {
        for episode in episodes {
            let expander = Expander::new(Some(&format!("{} - {}", episode.number, episode.title)));
            expander.add_css_class("season-expander-widget");
            let content = Grid::new();
            content.add_css_class("season-expander-content-widget");
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

            let row = ListBoxRow::new();
            row.set_child(Some(&expander));
            self.list.append(&row);
        }
    }

    pub fn root(&self) -> &ScrolledWindow {
        &self.root
    }
}
