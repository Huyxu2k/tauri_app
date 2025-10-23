use std::sync::Arc;
use command_app_lib::api::app_state::{add_message, list_messages, AppState};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::sync::{Mutex, oneshot};
use axum::{Router, routing::{get, post}};
use tauri::{Manager, Runtime, State};

pub struct ServerState{
    running: bool,
    shutdown_tx: Option<oneshot::Sender<()>>,
    address: String,
    port: i32,
}

#[derive(serde::Serialize)]
pub struct ServerInfo{
    status : String,
    address: String,
    port: i32
}

impl ServerState {
    pub fn new() -> ServerState{
        ServerState { 
            shutdown_tx: None, 
            running: false ,
            address: "127.0.0.1".to_string(),
            port: 3000
        }
    }
}

pub struct DbState {
    pub pool: SqlitePool,
}

#[tauri::command]
pub async fn start_server<R: Runtime>(state: State<'_, Arc<Mutex<ServerState>>>, db_state: State<'_, DbState>, window: tauri::Window<R>) -> Result<(), String> {
  let mut server = state.lock().await;

  if server.running{
    return Err("Server is already running".into());
  }

  let (tx, rx) = oneshot::channel::<()>();
  server.shutdown_tx = Some(tx);
  server.running = true;

  let _adr=  server.address.clone();
  let _port = server.port.clone();

  // Set up database
    //   let pool = SqlitePoolOptions::new()
    //         .max_connections(5)
    //         .connect("sqlite://./test.db")
    //         .await
    //         .map_err(|e| e.to_string())?;

    let app_state = AppState { pool: db_state.pool.clone()};

  // Spawn server trong tokio task
  tokio::spawn(async move {

    // Router
    let app = Router::new()
    .route("/health", get(|| async { "OK" }))
    .route("/add_message", post(add_message))
    .route("/list_messages", get(list_messages))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{_adr}:{_port}")).await.unwrap();
    println!("🚀 Server running at http://{}", format!("{_adr}:{_port}"));

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
            println!("🛑 Server shutting down");
        })
        .await
        .unwrap();
});
  Ok(())
}


#[tauri::command]
pub async fn stop_server<R: Runtime>(state: State<'_, Arc<Mutex<ServerState>>>, window: tauri::Window<R>) -> Result<(), String> {
    let mut server = state.lock().await;
  
    if !server.running {
        return Err("Server is not running".into());
    }

    if let Some(tx) = server.shutdown_tx.take() {
        // gửi tín hiệu shutdown
        let _ = tx.send(()); 
    }

    server.running = false;
  Ok(())
}

#[tauri::command]
pub async fn get_status<R: Runtime>(state: State<'_, Arc<Mutex<ServerState>>>, window: tauri::Window<R>) -> Result<ServerInfo, String> { //app: tauri::AppHandle<R>
    //let state = app.state::<Arc<Mutex<ServerState>>>();
    let server = state.lock().await;

    Ok(ServerInfo {
        status: if server.running { "Running".into() } else { "Stopped".into() },
        address: server.address.to_string(),
        port: server.port.clone(),
    })
}