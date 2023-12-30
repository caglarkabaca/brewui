use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::metadata::Versions;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub tap: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub desc: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub versions: Option<Versions>,
    pub bottle: Option<HashMap<String, Bottle>>,
    pub build_dependencies: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub analytics: Option<HashMap<String, Analytic>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootCask {
    pub token: Option<String>,
    pub full_token: Option<String>,
    pub tap: Option<String>,
    pub name: Option<Vec<String>>,
    pub desc: Option<String>,
    pub version: Option<String>,
    pub variations: Option<HashMap<String, serde_json::Value>>,
    pub analytics: Option<HashMap<String, Analytic>>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bottle {
    pub files: Option<HashMap<String, Option<File>>>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub cellar: Option<String>,
    pub url: Option<String>,
    pub sha256: Option<String>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Analytic {
    #[serde(rename = "30d")]
    n30d: Option<HashMap<String, i64>>,
    #[serde(rename = "90d")]
    n90d: Option<HashMap<String, i64>>,
    #[serde(rename = "365d")]
    n365d: Option<HashMap<String, i64>>,
}
