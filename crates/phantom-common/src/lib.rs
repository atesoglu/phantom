use serde::{Deserialize, Serialize};

pub const CONFIG_DIR: &str = "tinymacro";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Trigger {
    Hotkey { keys: String },
    Schedule { cron: String },
    ActiveWindow { title_contains: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Keystroke { text: String },
    Keys { combo: String },
    TextExpand { shortcut: String, expansion: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayStep {
    pub dx: i32,
    pub dy: i32,
    pub click: bool,
}

pub fn config_dir() -> std::path::PathBuf {
    let base = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    std::path::PathBuf::from(base).join(".config").join(CONFIG_DIR)
}