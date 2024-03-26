// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;
mod calc;
mod files;

#[tauri::command]
fn cook(stats_vector : Vec<Vec<u32>>) -> Vec<Vec<String>> {
    let shannon = calc::shannon(stats_vector.clone());
    let simpson = calc::simpson(stats_vector.clone());
    let dice = calc::dice(stats_vector.clone());
    let jaccard = calc::jaccard(stats_vector);
    return vec![shannon,simpson,dice,jaccard];
}

#[tauri::command]
fn getlocale() -> String {
    return format!("{}",sys_locale::get_locale().unwrap_or_else(|| String::from("en-US")));
}

#[tauri::command]
fn load_data_from_csv(selected : String) -> Vec<String>{
    let values = files::loadcsv(&selected);
    return values;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cook,getlocale,load_data_from_csv])
        .run(tauri::generate_context!())
        .expect("Unknown error: can't run tauri application.");
}