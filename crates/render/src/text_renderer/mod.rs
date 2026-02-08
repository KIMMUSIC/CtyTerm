use egui::{RichText, ScrollArea, Ui};

#[derive(Debug, Clone, Default)]
pub struct TerminalRenderData {
    pub lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BasicGridRenderer {
    pub row_height: f32,
}

impl Default for BasicGridRenderer {
    fn default() -> Self {
        Self { row_height: 18.0 }
    }
}

impl BasicGridRenderer {
    pub fn render(&self, ui: &mut Ui, data: &TerminalRenderData) {
        ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for line in &data.lines {
                    ui.label(RichText::new(line).monospace().size(self.row_height - 4.0));
                }
            });
    }
}
