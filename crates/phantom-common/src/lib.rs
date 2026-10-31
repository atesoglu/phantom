use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "phantom.sock";
pub const CONFIG_DIR: &str = "tinymacro";
pub const SHARE_DIR: &str = "tinymacro";

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
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    ListMacros,
    ToggleMacro { id: String, enabled: bool },
    AddMacro { macro_def: Macro },
    RemoveMacro { id: String },
    DaemonStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Response {
    Macros { items: Vec<Macro> },
    Ok,
    Status { running: bool, macros: usize },
    Error { message: String },
}

pub fn socket_path() -> std::path::PathBuf {
    let base = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    std::path::PathBuf::from(base).join(SOCKET_NAME)
}

pub fn config_dir() -> std::path::PathBuf {
    let base = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    std::path::PathBuf::from(base).join(".config").join(CONFIG_DIR)
}