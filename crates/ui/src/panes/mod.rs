#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PaneLayout {
    Single,
    VerticalSplit,
    HorizontalSplit,
}

#[derive(Debug, Clone)]
pub struct PaneGridState {
    layout: PaneLayout,
    active_pane: usize,
}

impl Default for PaneGridState {
    fn default() -> Self {
        Self {
            layout: PaneLayout::Single,
            active_pane: 0,
        }
    }
}

impl PaneGridState {
    pub fn set_single(&mut self) {
        self.layout = PaneLayout::Single;
        self.active_pane = 0;
    }

    pub fn split_vertical(&mut self) {
        self.layout = PaneLayout::VerticalSplit;
    }

    pub fn split_horizontal(&mut self) {
        self.layout = PaneLayout::HorizontalSplit;
    }

    pub fn layout(&self) -> PaneLayout {
        self.layout
    }

    pub fn pane_count(&self) -> usize {
        match self.layout {
            PaneLayout::Single => 1,
            PaneLayout::VerticalSplit | PaneLayout::HorizontalSplit => 2,
        }
    }

    pub fn active_pane(&self) -> usize {
        self.active_pane
    }

    pub fn set_active_pane(&mut self, pane_idx: usize) {
        self.active_pane = pane_idx.min(self.pane_count().saturating_sub(1));
    }

    pub fn label(&self) -> String {
        let layout = match self.layout {
            PaneLayout::Single => "single",
            PaneLayout::VerticalSplit => "vertical",
            PaneLayout::HorizontalSplit => "horizontal",
        };
        format!("panes: {} ({layout})", self.pane_count())
    }
}
