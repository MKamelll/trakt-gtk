use gtk4::{
    Align, Box, Button, Frame, Grid, Label, Orientation, ScrolledWindow,
    prelude::{BoxExt, ButtonExt, FrameExt, GridExt, WidgetExt},
};

use crate::traktclient::Show;

#[derive(Clone)]
pub struct ShowInfoWidget {
    root: Box,
    title_label: Label,
    year_label: Label,
    overview_label: Label,
    tagline_label: Label,
    first_aired_label: Label,
    runtime_label: Label,
    network_label: Label,
    country_label: Label,
    rating_label: Label,
    languages_label: Label,
    genres_label: Label,
    original_title_label: Label,
}

impl ShowInfoWidget {
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

        vbox.append(&scrolled_window);
        vbox.add_css_class("show-details-widget");

        Self {
            root: vbox,
            title_label,
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
