#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::cmd::{extract, get_mod_list, rename, write_file};

mod cmd;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_mod_list,
            rename,
            write_file,
            extract
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
