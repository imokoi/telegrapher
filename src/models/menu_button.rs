use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp(MenuButtonWebApp),
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MenuButtonWebApp {
    pub text: String,
    pub web_app: WebAppInfo,
}
