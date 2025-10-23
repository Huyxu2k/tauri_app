use serde::Serialize;
use tauri::State;

use crate::{message::ToastPayload, server::DbState};



#[derive(sqlx::FromRow, serde::Serialize, Debug, Clone)]
pub struct Device {
    pub id: i32,
    pub serial: String,
    pub name: String,
    pub mode: String,
    pub ip_address: String,
    pub note: String,
    pub count_on: u32,
    pub status: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct AdbDevice{
    pub serial: String,
}

///
/// Chạy lệnh ADB để tìm kiếm các thiết bị đang kết nối với máu tính
#[tauri::command]
pub async fn adb_list_device() -> Result<Vec<AdbDevice>, String> {
    use tokio::process::Command;

    let output = Command::new("adb")
        .arg("devices")
        .output()
        .await
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();

    let mut index = 0;
    for line in stdout.lines().skip(1) { 
        // bỏ dòng "List of devices attached"
        if line.trim().is_empty() {
            continue;
        }
        index += 1;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            devices.push(AdbDevice {
                serial: parts[0].to_string()
            });
        }
    }

    Ok(devices)
}


/// Lấy dữ liệu các thiết bị từ database 
pub async fn db_list_device(db_state: State<'_, DbState>) -> Result<Vec<Device>, String>{
    // Lấy dữ liệu từ database
    let devices = sqlx::query_as::<_, Device>("SELECT * FROM devices")
        .fetch_all(&db_state.pool)
        .await
        .map_err(|e| e.to_string())?;

    // Lấy dữ liệu bằng lệnh adb
    // Nếu thiết bị lấy từ database mà có trong danh sách adb trả về thì update lại status 


    Ok(devices)
}



/// Lưu dữ liệu các thiết bị vào database
pub async fn db_save_devices(data: Vec<Device>){
    todo!()
}

/// Xóa dữ liệu của cac thiết bị đã lưu trong database
pub async fn db_delete_devives(data: Vec<i32>){
    todo!()
}


//TEST
#[tauri::command]
pub async fn test_success() -> Result<ToastPayload, String>{
    Ok(ToastPayload::success("Thành công!"))
}

//TEST
#[tauri::command]
pub async fn test_error() -> Result<ToastPayload, String>{
    Ok(ToastPayload::error("Thất bại1"))
}

