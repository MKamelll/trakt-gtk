use std::collections::HashMap;

use gtk4::{
    Align, Box, Expander, Label, ListBase, ListBox, ListBoxRow, Orientation, Paned, ScrolledWindow,
    Stack, glib,
    prelude::{BoxExt, ListBoxRowExt, WidgetExt},
};

use crate::{
    seasonwidget::SeasonWidget,
    traktclient::{Episode, Season, Show},
};

#[derive(Clone)]
pub struct SeasonsWidget {
    root: Paned,
    sidebar_list: ListBox,
    stack: Stack,
}

impl SeasonsWidget {
    pub fn new() -> Self {
        let stack = Stack::new();

        let list_box = ListBox::new();
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_vexpand(true);
        scrolled_window.set_child(Some(&list_box));

        let paned = Paned::new(Orientation::Horizontal);
        paned.set_start_child(Some(&scrolled_window));
        paned.set_end_child(Some(&stack));
        paned.set_position(150);

        list_box.connect_row_activated(glib::clone!(
            #[weak]
            stack,
            move |_, row| {
                stack.set_visible_child_name(&row.index().to_string());
            }
        ));

        Self {
            root: paned,
            sidebar_list: list_box,
            stack,
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
                let s_widget = SeasonWidget::new();
                s_widget.update(episodes);
                self.stack
                    .add_named(s_widget.root(), Some(&season.number.to_string()));
            } else {
                eprintln!(
                    "couldn't get episodes of season number {}",
                    season.number.to_string()
                )
            }
        }
    }

    pub fn root(&self) -> &Paned {
        &self.root
    }
}
