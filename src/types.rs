use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) email: String,

    #[serde(default)]
    pub(crate) ssh_path: Option<String>,
    #[serde(default)]
    pub(crate) alias: Option<String>,
    pub(crate) token: Option<Vec<u8>>,
    
    #[serde(default)]
    pub(crate) default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct ActiveAccount {
    pub(crate) username: String,
    pub(crate) email: String,

    #[serde(default)]
    pub(crate) alias: Option<String>,
    
    #[serde(default)]
    pub(crate) token: Option<Vec<u8>>,
}

impl Default for ActiveAccount {
    fn default() -> Self {
        ActiveAccount {
            username: String::new(),
            email: String::new(),
            alias: None,
            token: None,
        }
    }
}