// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_process::Command;
use reqwest;

mod components;
use components::*;

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

// BURANIN OUTPUTU METADATAYA UYMUYOR HATALI
#[tauri::command]
async fn get_all_casks() -> Result<Vec<metadata::Metadata>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://formulae.brew.sh/api/cask.json")
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_installed_formulas,
            get_installed_casks,
            get_all_formulas,
            get_all_casks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
