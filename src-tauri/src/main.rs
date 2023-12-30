// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use async_process::{Command, ExitStatus};
use reqwest;

mod components;
use components::{
    info::Bottle,
    metadata::{Metadata, MetadataCask, Versions},
    *,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_installed_formulas() -> Result<response::InstalledResponse, String> {
    let cmd = Command::new("brew")
        .arg("list")
        .arg("--versions")
        .arg("--formula")
        .output()
        .await;
    if cmd.is_err() {
        return Err(String::from("cmd error"));
    } else {
        let cmd = cmd.unwrap();
        let output = String::from_utf8(cmd.stdout).unwrap();
        // println!("output: {}", output);
        let paths: Vec<&str> = output.split("\n").collect();
        return Ok(response::InstalledResponse {
            count: paths.len(),
            name: paths.iter().map(|path| format!("{}", path)).collect(),
        });
    }
}

#[tauri::command]
async fn get_installed_casks() -> Result<response::InstalledResponse, String> {
    let cmd = Command::new("brew")
        .arg("list")
        .arg("--versions")
        .arg("--cask")
        .output()
        .await;
    if cmd.is_err() {
        return Err(String::from("cmd error"));
    } else {
        let cmd = cmd.unwrap();
        let output = String::from_utf8(cmd.stdout).unwrap();
        // println!("output: {}", output);
        let paths: Vec<&str> = output.split("\n").collect();
        return Ok(response::InstalledResponse {
            count: paths.len(),
            name: paths.iter().map(|path| format!("{}", path)).collect(),
        });
    }
}

#[tauri::command]
async fn get_all_formulas() -> Result<Vec<metadata::Metadata>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://formulae.brew.sh/api/formula.json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Vec<metadata::Metadata>>().await {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(format!("parse error {}", e)),
        },
        _ => Err(format!("status not ok")),
    }
}

#[tauri::command]
async fn get_all_casks() -> Result<Vec<metadata::Metadata>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://formulae.brew.sh/api/cask.json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Vec<metadata::MetadataCask>>().await {
            Ok(parsed) => {
                let rtn: Vec<Metadata> = parsed
                    .iter()
                    .map(|p| {
                        return Metadata {
                            name: p.token.clone(),
                            full_name: Some(String::from(p.name.clone().unwrap().first().unwrap())),
                            desc: p.desc.clone(),
                            license: None,
                            homepage: p.homepage.clone(),
                            versions: Some(Versions {
                                stable: p.version.clone(),
                                head: None,
                            }),
                            head_dependencies: None,
                            outdated: p.disabled,
                            deprecated: p.deprecated,
                        };
                    })
                    .collect();
                Ok(rtn)
            }
            Err(e) => Err(format!("parse error {}", e)),
        },
        _ => Err(format!("status not ok")),
    }
}

#[tauri::command]
async fn get_info_formula(name: String) -> Result<info::Root, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://formulae.brew.sh/api/formula/{}.json",
            name
        ))
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<info::Root>().await {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(format!("parse err {}", e)),
        },
        _ => Err(format!("status not ok")),
    }
}

#[tauri::command]
async fn get_info_cask(name: String) -> Result<info::Root, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://formulae.brew.sh/api/cask/{}.json", name))
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => match response.json::<info::RootCask>().await {
            Ok(parsed) => {
                let mut files: HashMap<String, Option<info::File>> = HashMap::new();
                for (key, _) in parsed.variations.unwrap() {
                    files.insert(key, None);
                }
                let mut bottle: HashMap<String, Bottle> = HashMap::new();
                bottle.insert(format!("stable"), Bottle { files: Some(files) });
                let rtn = info::Root {
                    name: parsed.token,
                    full_name: Some(String::from(parsed.name.unwrap().first().unwrap())),
                    tap: parsed.tap,
                    aliases: None,
                    desc: parsed.desc,
                    license: None,
                    homepage: None,
                    versions: Some(Versions {
                        stable: parsed.version,
                        head: None,
                    }),
                    bottle: Some(bottle),
                    build_dependencies: None,
                    dependencies: None,
                    analytics: parsed.analytics,
                };
                Ok(rtn)
            }
            Err(e) => Err(format!("parse err {}", e)),
        },
        _ => Err(format!("status not ok")),
    }
}

#[tauri::command]
async fn execute_command(args: Vec<String>) -> Result<String, String> {
    let mut brew = Command::new("brew");
    brew.args(args);
    let output = brew.output().await;

    match output {
        Ok(res) => {
            if res.status.success() {
                return Ok(String::from_utf8(res.stdout).unwrap());
            } else {
                return Err(String::from_utf8(res.stderr).unwrap());
            }
        }
        Err(e) => return Err(format!("cmd err {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_installed_formulas,
            get_installed_casks,
            get_all_formulas,
            get_all_casks,
            get_info_formula,
            get_info_cask,
            execute_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
