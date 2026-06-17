use gtk4::{Box, Label, Orientation, prelude::BoxExt};

#[derive(Clone)]
pub struct SeasonsWidget {
    root: Box,
}

impl SeasonsWidget {
    pub fn new() -> Self {
        let vbox = Box::new(Orientation::Vertical, 4);
        let label = Label::new(Some("Seasons"));

        vbox.append(&label);
        Self { root: vbox }
    }

    pub fn root(&self) -> &Box {
        &self.root
    }
}
