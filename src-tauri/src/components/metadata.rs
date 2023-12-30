use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Versions {
    pub stable: Option<String>,
    pub head: Option<String>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Dependencies {
    build_dependencies: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub desc: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub versions: Option<Versions>,
    pub head_dependencies: Option<Dependencies>,
    pub outdated: bool,
    pub deprecated: bool,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataCask {
    pub token: Option<String>,
    pub full_token: Option<String>,
    pub name: Option<Vec<String>>,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    pub version: Option<String>,
    pub deprecated: bool,
    pub disabled: bool,
}
