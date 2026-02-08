use eframe::egui;

use crate::theme;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TabAction {
    AddTab,
    CloseTab(u64),
}

#[derive(Debug, Clone)]
struct TabEntry {
    id: u64,
    label: String,
}

#[derive(Debug, Clone)]
pub struct TabState {
    tabs: Vec<TabEntry>,
    active_idx: usize,
    next_id: u64,
    dragging_tab_id: Option<u64>,
    scroll_to_active: bool,
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            tabs: vec![TabEntry {
                id: 0,
                label: "main".to_owned(),
            }],
            active_idx: 0,
            next_id: 1,
            dragging_tab_id: None,
            scroll_to_active: true,
        }
    }
}

impl TabState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<TabAction> {
        let mut action = None;
        let tabs_snapshot = self.tabs.clone();
        let can_close_tab = tabs_snapshot.len() > 1;
        let mut move_request: Option<(u64, u64, bool)> = None;
        let mut tab_hit_rects: Vec<(u64, egui::Rect)> = Vec::with_capacity(tabs_snapshot.len());

        ui.horizontal(|ui| {
            let add_tab_reserve = 120.0;
            let tabs_width = (ui.available_width() - add_tab_reserve).max(80.0);

            ui.allocate_ui_with_layout(
                egui::vec2(tabs_width, ui.available_height()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.style_mut().always_scroll_the_only_direction = true;
                    egui::ScrollArea::horizontal()
                        .id_salt("tab-strip-scroll")
                        .auto_shrink([false, true])
                        .drag_to_scroll(false)
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (idx, tab) in tabs_snapshot.iter().enumerate() {
                                    let selected = idx == self.active_idx;
                                    let label = if selected {
                                        format!("{} *", tab.label)
                                    } else {
                                        tab.label.clone()
                                    };

                                    ui.horizontal(|ui| {
                                        let tab_response = ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(label).monospace().color(
                                                        if selected {
                                                            theme::TEXT_BRIGHT
                                                        } else {
                                                            theme::TEXT_MUTED
                                                        },
                                                    ),
                                                )
                                                .sense(egui::Sense::click_and_drag()),
                                            )
                                            .on_hover_cursor(egui::CursorIcon::Default);
                                        tab_hit_rects.push((tab.id, tab_response.rect));

                                        if tab_response.clicked() {
                                            self.active_idx = idx;
                                            self.scroll_to_active = true;
                                        }
                                        if tab_response.drag_started() {
                                            self.dragging_tab_id = Some(tab.id);
                                        }
                                        if selected && self.scroll_to_active {
                                            tab_response.scroll_to_me(Some(egui::Align::Center));
                                        }

                                        if ui
                                            .add_enabled(
                                                can_close_tab,
                                                egui::Button::new(
                                                    egui::RichText::new("x")
                                                        .monospace()
                                                        .color(theme::TEXT_MUTED),
                                                ),
                                            )
                                            .on_hover_text("close tab")
                                            .on_hover_cursor(egui::CursorIcon::Default)
                                            .clicked()
                                        {
                                            action = Some(TabAction::CloseTab(tab.id));
                                        }
                                    });
                                }
                            });
                        });
                },
            );

            ui.separator();
            if ui
                .button(egui::RichText::new("+ new tab").color(theme::ACCENT_BLUE))
                .clicked()
            {
                action = Some(TabAction::AddTab);
            }
        });

        if let Some(dragging_tab_id) = self.dragging_tab_id {
            let pointer_down = ui.input(|i| i.pointer.primary_down());
            if !pointer_down {
                self.dragging_tab_id = None;
            } else if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                if let Some((target_tab_id, target_rect)) = tab_hit_rects
                    .iter()
                    .find(|(id, rect)| *id != dragging_tab_id && rect.contains(pointer_pos))
                {
                    let insert_after = pointer_pos.x > target_rect.center().x;
                    move_request = Some((dragging_tab_id, *target_tab_id, insert_after));
                } else if let Some((first_tab_id, first_rect)) = tab_hit_rects.first() {
                    if pointer_pos.x < first_rect.left() && dragging_tab_id != *first_tab_id {
                        move_request = Some((dragging_tab_id, *first_tab_id, false));
                    } else if let Some((last_tab_id, last_rect)) = tab_hit_rects.last() {
                        if pointer_pos.x > last_rect.right() && dragging_tab_id != *last_tab_id {
                            move_request = Some((dragging_tab_id, *last_tab_id, true));
                        }
                    }
                }
            }
        }

        if let Some((dragging_tab_id, target_tab_id, insert_after)) = move_request {
            let _ = self.reorder_tab_relative(dragging_tab_id, target_tab_id, insert_after);
        }
        self.scroll_to_active = false;

        action
    }

    pub fn add_tab(&mut self) -> u64 {
        let tab_id = self.next_id;
        self.tabs.push(TabEntry {
            id: tab_id,
            label: format!("tab-{tab_id}"),
        });
        self.active_idx = self.tabs.len().saturating_sub(1);
        self.scroll_to_active = true;
        self.next_id += 1;
        tab_id
    }

    pub fn add_tab_with_label(&mut self, label: impl Into<String>) -> u64 {
        let tab_id = self.next_id;
        self.tabs.push(TabEntry {
            id: tab_id,
            label: label.into(),
        });
        self.active_idx = self.tabs.len().saturating_sub(1);
        self.scroll_to_active = true;
        self.next_id += 1;
        tab_id
    }

    pub fn active_id(&self) -> u64 {
        self.tabs
            .get(self.active_idx)
            .map(|tab| tab.id)
            .unwrap_or(0)
    }

    pub fn set_active_by_id(&mut self, tab_id: u64) -> bool {
        if let Some(idx) = self.tabs.iter().position(|tab| tab.id == tab_id) {
            self.active_idx = idx;
            self.scroll_to_active = true;
            return true;
        }
        false
    }

    pub fn set_tab_label(&mut self, tab_id: u64, label: impl Into<String>) -> bool {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) {
            tab.label = label.into();
            return true;
        }
        false
    }

    pub fn active_label(&self) -> &str {
        self.tabs
            .get(self.active_idx)
            .map(|tab| tab.label.as_str())
            .unwrap_or("main")
    }

    pub fn entries(&self) -> Vec<(u64, String)> {
        self.tabs
            .iter()
            .map(|tab| (tab.id, tab.label.clone()))
            .collect()
    }

    pub fn replace_tabs(&mut self, entries: Vec<(u64, String)>, active_id: u64) {
        if entries.is_empty() {
            self.tabs = vec![TabEntry {
                id: 0,
                label: "main".to_owned(),
            }];
            self.active_idx = 0;
            self.next_id = 1;
            self.dragging_tab_id = None;
            self.scroll_to_active = true;
            return;
        }

        self.tabs = entries
            .into_iter()
            .map(|(id, label)| TabEntry { id, label })
            .collect();

        self.active_idx = self
            .tabs
            .iter()
            .position(|tab| tab.id == active_id)
            .unwrap_or(0);

        self.next_id = self
            .tabs
            .iter()
            .map(|tab| tab.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        self.dragging_tab_id = None;
        self.scroll_to_active = true;
    }

    fn reorder_tab_relative(
        &mut self,
        dragging_tab_id: u64,
        target_tab_id: u64,
        insert_after: bool,
    ) -> bool {
        if dragging_tab_id == target_tab_id {
            return false;
        }
        let Some(from_idx) = self.tabs.iter().position(|tab| tab.id == dragging_tab_id) else {
            return false;
        };
        let Some(target_idx) = self.tabs.iter().position(|tab| tab.id == target_tab_id) else {
            return false;
        };

        let mut destination = target_idx;
        if insert_after {
            destination = destination.saturating_add(1);
        }
        if from_idx < destination {
            destination = destination.saturating_sub(1);
        }
        if destination == from_idx {
            return false;
        }

        let active_tab_id = self.active_id();
        let dragged = self.tabs.remove(from_idx);
        let destination = destination.min(self.tabs.len());
        self.tabs.insert(destination, dragged);
        self.active_idx = self
            .tabs
            .iter()
            .position(|tab| tab.id == active_tab_id)
            .unwrap_or(0);
        self.scroll_to_active = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::TabState;

    #[test]
    fn reorder_tab_before_moves_entry_and_preserves_active_tab() {
        let mut tabs = TabState::default();
        let tab1 = tabs.add_tab();
        let tab2 = tabs.add_tab();
        let tab3 = tabs.add_tab();
        let _ = tabs.set_active_by_id(tab2);

        assert!(tabs.reorder_tab_relative(tab3, tab1, false));
        assert_eq!(
            tabs.entries()
                .into_iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            vec![0, tab3, tab1, tab2]
        );
        assert_eq!(tabs.active_id(), tab2);
    }

    #[test]
    fn reorder_tab_before_ignores_invalid_moves() {
        let mut tabs = TabState::default();
        let tab1 = tabs.add_tab();

        assert!(!tabs.reorder_tab_relative(tab1, tab1, false));
        assert!(!tabs.reorder_tab_relative(tab1, 9999, false));
        assert!(!tabs.reorder_tab_relative(9999, tab1, false));
    }

    #[test]
    fn reorder_tab_relative_can_insert_after_target() {
        let mut tabs = TabState::default();
        let tab1 = tabs.add_tab();
        let tab2 = tabs.add_tab();
        let tab3 = tabs.add_tab();

        assert!(tabs.reorder_tab_relative(tab1, tab3, true));
        assert_eq!(
            tabs.entries()
                .into_iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            vec![0, tab2, tab3, tab1]
        );
    }
}
