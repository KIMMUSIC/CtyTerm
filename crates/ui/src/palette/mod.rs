use eframe::egui;

use crate::theme;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaletteAction {
    ApplyCommand(String),
}

#[derive(Debug, Clone, Default)]
pub struct PaletteState {
    is_open: bool,
    query: String,
    focus_search_next_frame: bool,
}

impl PaletteState {
    pub fn show_window(
        &mut self,
        ctx: &egui::Context,
        history_matches: &[String],
    ) -> Vec<PaletteAction> {
        let mut actions = Vec::new();

        if !self.is_open {
            return actions;
        }

        egui::Window::new("Command Palette")
            .collapsible(false)
            .resizable(true)
            .default_size([580.0, 360.0])
            .frame(theme::panel_frame())
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("SEARCH")
                            .monospace()
                            .strong()
                            .color(theme::TEXT_MUTED),
                    );
                    let search = ui.add_sized(
                        [ui.available_width() - 80.0, 24.0],
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("search history command"),
                    );
                    if self.focus_search_next_frame {
                        search.request_focus();
                        self.focus_search_next_frame = false;
                    }
                    if search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.close();
                    }
                });

                ui.separator();
                ui.label(
                    egui::RichText::new("HISTORY")
                        .monospace()
                        .strong()
                        .color(theme::TEXT_MUTED),
                );

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for command in history_matches {
                            if ui
                                .selectable_label(
                                    false,
                                    egui::RichText::new(command)
                                        .monospace()
                                        .color(theme::TEXT_PRIMARY),
                                )
                                .clicked()
                            {
                                actions.push(PaletteAction::ApplyCommand(command.clone()));
                            }
                        }
                    });
            });

        actions
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.focus_search_next_frame = true;
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.focus_search_next_frame = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.focus_search_next_frame = false;
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn set_query(&mut self, value: impl Into<String>) {
        self.query = value.into();
    }
}
