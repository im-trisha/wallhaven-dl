use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ProfileWithPath {
    pub profile: Profile,
    pub path: PathBuf,
}

pub struct Firefox {
    pub profiles: Vec<ProfileWithPath>,
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub windows: Vec<Window>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Window {
    pub tabs: Vec<Tab>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Tab {
    pub entries: Vec<Entry>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub url: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
