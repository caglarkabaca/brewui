use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::metadata::Versions;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    name: Option<String>,
    full_name: Option<String>,
    tap: Option<String>,
    aliases: Option<Vec<String>>,
    desc: Option<String>,
    license: Option<String>,
    homepage: Option<String>,
    versions: Option<Versions>,
    bottle: Option<HashMap<String, Bottle>>,
    build_dependencies: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    analytics: Option<HashMap<String, Analytic>>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Bottle {
    files: Option<HashMap<String, File>>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    cellar: Option<String>,
    url: Option<String>,
    sha256: Option<String>,
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
