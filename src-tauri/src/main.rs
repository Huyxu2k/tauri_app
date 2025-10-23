// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::Mutex;

mod server;
mod device;
mod setting;
mod message;

use server::{ServerState, start_server, stop_server, get_status};
use device::{adb_list_device, test_error, test_success};
use setting::*;

use crate::server::DbState;

#[tokio::main]
async fn main() {
    //command_app_lib::run()

    // Set up database
    let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite://./test.db")
            .await
            .expect("Failed to create database pool.");

    tauri::Builder::default()
    .manage(Arc::new(Mutex::new(ServerState::new())))
    .manage(DbState { pool })
    .invoke_handler(tauri::generate_handler![
        // Server
        start_server, stop_server, get_status,
        // Device
        adb_list_device,
        test_success,
        test_error
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



