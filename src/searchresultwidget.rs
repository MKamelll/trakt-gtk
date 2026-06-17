use gtk4::{
    Align, Box, Label, Orientation,
    prelude::{BoxExt, WidgetExt},
};

use crate::traktclient::SearchResult;

pub struct SearchResultWidget {
    root: Box,
}

impl SearchResultWidget {
    pub fn new(result: &SearchResult) -> Self {
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

        Self { root: vbox }
    }

    pub fn root(&self) -> &Box {
        &self.root
    }
}
