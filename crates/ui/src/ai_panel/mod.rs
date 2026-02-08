use eframe::egui;
use ux_model::ai::AiTool;

use crate::theme;

#[derive(Debug, Clone)]
pub enum AiPanelAction {
    RunPrompt { tool: AiTool, prompt: String },
}

#[derive(Debug, Clone)]
pub struct AiPanelState {
    selected_tool: AiTool,
    prompt_input: String,
    clear_on_run: bool,
}

impl Default for AiPanelState {
    fn default() -> Self {
        Self {
            selected_tool: AiTool::CodexCli,
            prompt_input: String::new(),
            clear_on_run: true,
        }
    }
}

impl AiPanelState {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        selected_context_count: usize,
        running_jobs: usize,
        status_line: &str,
    ) -> Option<AiPanelAction> {
        let mut action = None;

        theme::panel_frame().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new("AI PANEL")
                        .monospace()
                        .strong()
                        .color(theme::TEXT_MUTED),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("context: {selected_context_count}"))
                        .monospace()
                        .color(theme::TEXT_MUTED),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("running: {running_jobs}"))
                        .monospace()
                        .color(if running_jobs > 0 {
                            theme::ACCENT_BLUE
                        } else {
                            theme::TEXT_MUTED
                        }),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(status_line)
                        .monospace()
                        .color(theme::TEXT_PRIMARY),
                );
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("tool")
                        .monospace()
                        .color(theme::TEXT_MUTED),
                );
                egui::ComboBox::from_id_salt("ai-tool-combo")
                    .selected_text(self.selected_tool.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_tool,
                            AiTool::CodexCli,
                            AiTool::CodexCli.label(),
                        );
                        ui.selectable_value(
                            &mut self.selected_tool,
                            AiTool::ClaudeCode,
                            AiTool::ClaudeCode.label(),
                        );
                    });

                ui.checkbox(&mut self.clear_on_run, "clear prompt on run");
            });

            ui.add_space(4.0);
            ui.add_sized(
                [ui.available_width(), 84.0],
                egui::TextEdit::multiline(&mut self.prompt_input).hint_text(
                    "enter AI prompt (selected context blocks are attached automatically)",
                ),
            );

            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("run ai").color(theme::ACCENT_BLUE))
                    .clicked()
                {
                    let prompt = self.prompt_input.trim().to_owned();
                    if !prompt.is_empty() {
                        action = Some(AiPanelAction::RunPrompt {
                            tool: self.selected_tool,
                            prompt,
                        });
                        if self.clear_on_run {
                            self.prompt_input.clear();
                        }
                    }
                }
                if ui
                    .button(egui::RichText::new("clear").color(theme::TEXT_MUTED))
                    .clicked()
                {
                    self.prompt_input.clear();
                }
            });
        });

        action
    }
}
