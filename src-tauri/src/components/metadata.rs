use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Versions {
    stable: Option<String>,
    head: Option<String>,
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
    name: Option<String>,
    full_name: Option<String>,
    desc: Option<String>,
    license: Option<String>,
    homepage: Option<String>,
    versions: Option<Versions>,
    head_dependencies: Option<Dependencies>,
    outdated: bool,
    deprecated: bool,
}
