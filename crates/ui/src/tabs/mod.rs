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
        }
    }
}

impl TabState {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<TabAction> {
        let mut action = None;
        let tabs_snapshot = self.tabs.clone();
        let can_close_tab = tabs_snapshot.len() > 1;

        ui.horizontal(|ui| {
            for (idx, tab) in tabs_snapshot.iter().enumerate() {
                let selected = idx == self.active_idx;
                let label = if selected {
                    format!("{} *", tab.label)
                } else {
                    tab.label.clone()
                };
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            selected,
                            egui::RichText::new(label).monospace().color(if selected {
                                theme::TEXT_BRIGHT
                            } else {
                                theme::TEXT_MUTED
                            }),
                        )
                        .clicked()
                    {
                        self.active_idx = idx;
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
                        .clicked()
                    {
                        action = Some(TabAction::CloseTab(tab.id));
                    }
                });
            }
            ui.separator();
            if ui
                .button(egui::RichText::new("+ new tab").color(theme::ACCENT_BLUE))
                .clicked()
            {
                action = Some(TabAction::AddTab);
            }
        });

        action
    }

    pub fn add_tab(&mut self) -> u64 {
        let tab_id = self.next_id;
        self.tabs.push(TabEntry {
            id: tab_id,
            label: format!("tab-{tab_id}"),
        });
        self.active_idx = self.tabs.len().saturating_sub(1);
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
    }
}
