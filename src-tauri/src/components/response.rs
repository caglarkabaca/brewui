#[derive(serde::Serialize)]
pub struct InstalledResponse {
    pub name: Vec<String>,
    pub count: usize,
}
