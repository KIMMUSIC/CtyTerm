use anyhow::Result;
use serde::{Deserialize, Serialize};
use ux_model::ai::AiTool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub ai: AiConfig,
    pub session: SessionConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiConfig {
    pub timeout_sec: u64,
    pub claude_continue: bool,
    pub codex: AiCommandTemplate,
    pub claude: AiCommandTemplate,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            timeout_sec: 300,
            claude_continue: true,
            codex: AiCommandTemplate {
                program: "codex".to_owned(),
                args: vec!["exec".to_owned(), "{prompt}".to_owned()],
            },
            claude: AiCommandTemplate {
                program: "claude".to_owned(),
                args: vec!["--print".to_owned(), "{prompt}".to_owned()],
            },
        }
    }
}

impl AiConfig {
    pub fn resolve(&self, tool: AiTool, prompt: &str) -> ResolvedAiCommand {
        match tool {
            AiTool::CodexCli => self.codex.resolve(prompt),
            AiTool::ClaudeCode => {
                let mut resolved = self.claude.resolve(prompt);
                if self.claude_continue {
                    resolved.args = ensure_claude_continue_args(&resolved.args);
                }
                resolved
            }
        }
    }
}

fn ensure_claude_continue_args(args: &[String]) -> Vec<String> {
    let has_session_control_flag = args.iter().any(|arg| {
        arg == "--continue"
            || arg == "-c"
            || arg == "--resume"
            || arg == "-r"
            || arg.starts_with("--resume=")
            || arg == "--session-id"
            || arg.starts_with("--session-id=")
            || arg == "--no-session-persistence"
    });

    if has_session_control_flag {
        return args.to_vec();
    }

    let mut updated = Vec::with_capacity(args.len() + 1);
    updated.push("--continue".to_owned());
    updated.extend(args.iter().cloned());
    updated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionConfig {
    pub autosave_interval_sec: u64,
    pub session_file: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            autosave_interval_sec: 3,
            session_file: "state/session.toml".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiCommandTemplate {
    pub program: String,
    pub args: Vec<String>,
}

impl Default for AiCommandTemplate {
    fn default() -> Self {
        Self {
            program: String::new(),
            args: Vec::new(),
        }
    }
}

impl AiCommandTemplate {
    pub fn resolve(&self, prompt: &str) -> ResolvedAiCommand {
        let args = self
            .args
            .iter()
            .map(|arg| arg.replace("{prompt}", prompt))
            .collect();

        ResolvedAiCommand {
            program: self.program.clone(),
            args,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedAiCommand {
    pub program: String,
    pub args: Vec<String>,
}

pub fn serialize_pretty(config: &AppConfig) -> Result<String> {
    Ok(toml::to_string_pretty(config)?)
}

pub fn deserialize(content: &str) -> Result<AppConfig> {
    Ok(toml::from_str(content)?)
}

#[cfg(test)]
mod tests {
    use super::{AiCommandTemplate, AiConfig, AppConfig, deserialize, serialize_pretty};
    use ux_model::ai::AiTool;

    #[test]
    fn template_replaces_prompt_placeholder() {
        let template = AiCommandTemplate {
            program: "tool".to_owned(),
            args: vec!["run".to_owned(), "{prompt}".to_owned()],
        };

        let resolved = template.resolve("hello world");
        assert_eq!(resolved.program, "tool");
        assert_eq!(resolved.args, vec!["run", "hello world"]);
    }

    #[test]
    fn config_roundtrip_works() {
        let config = AppConfig::default();
        let text = serialize_pretty(&config).expect("serialize should succeed");
        let parsed = deserialize(&text).expect("deserialize should succeed");
        assert_eq!(parsed.session.autosave_interval_sec, 3);
        assert_eq!(parsed.session.session_file, "state/session.toml");
        assert_eq!(parsed.ai.timeout_sec, 300);
        assert!(parsed.ai.claude_continue);
        assert_eq!(parsed.ai.codex.program, "codex");
    }

    #[test]
    fn claude_resolve_injects_continue_by_default() {
        let config = AiConfig::default();
        let resolved = config.resolve(AiTool::ClaudeCode, "hello");
        assert_eq!(
            resolved.args,
            vec![
                "--continue".to_owned(),
                "--print".to_owned(),
                "hello".to_owned()
            ]
        );
    }

    #[test]
    fn claude_resolve_keeps_explicit_session_flags() {
        let config = AiConfig {
            claude: AiCommandTemplate {
                program: "claude".to_owned(),
                args: vec!["--resume".to_owned(), "{prompt}".to_owned()],
            },
            ..AiConfig::default()
        };
        let resolved = config.resolve(AiTool::ClaudeCode, "hello");
        assert_eq!(
            resolved.args,
            vec!["--resume".to_owned(), "hello".to_owned()]
        );
    }

    #[test]
    fn claude_resolve_respects_no_session_persistence() {
        let config = AiConfig {
            claude: AiCommandTemplate {
                program: "claude".to_owned(),
                args: vec!["--no-session-persistence".to_owned(), "{prompt}".to_owned()],
            },
            ..AiConfig::default()
        };
        let resolved = config.resolve(AiTool::ClaudeCode, "hello");
        assert_eq!(
            resolved.args,
            vec!["--no-session-persistence".to_owned(), "hello".to_owned()]
        );
    }
}
