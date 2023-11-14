// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod process;

use std::sync::Mutex;
use sysinfo::{System, SystemExt};

pub struct GlobalSystem(Mutex<System>);

impl GlobalSystem {
    fn new_all() -> Self {
        Self(Mutex::new(System::new_all()))
    }
}

fn main() {

    tauri::Builder::default()
        .manage(GlobalSystem::new_all())
        .invoke_handler(tauri::generate_handler![process::sys_info, process::find_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
