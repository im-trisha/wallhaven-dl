use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ProfileWithPath {
    pub profile: Profile,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Firefox {
    pub profiles: Vec<ProfileWithPath>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub windows: Vec<Window>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Window {
    pub tabs: Vec<Tab>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tab {
    pub entries: Vec<Entry>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub url: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
