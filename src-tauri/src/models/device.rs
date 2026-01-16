use std::{collections::HashMap, sync::PoisonError};

use crate::adb::command::*;

#[derive(Debug, Clone)]
pub enum Status {
    Disconnected, // có dữ liệu trong db nhưng thực tế đang không kết nối
    Waiting,      // đang ở trạng thái chờ (rảnh)
    Running,      // đang ở trạng thái hoạt động
    Connection,   // chưa có dữ liệu trong db nhưng thực tế đang có kết nối
}


#[derive(sqlx::FromRow, serde::Serialize, Debug, Clone)]
pub struct Device {
    pub id: i32,
    pub serial: String,
    pub name: String,
    pub mode: String,
    pub ip_address: String,
    pub note: String,
    pub count_on: u32,
    pub status: Status
}

pub async fn devices_to_hash(pool: SqlitePool) -> HashMap<String, Status>{
    let result = HashMap::<String, Status>::new();
    let db = db_list_device(pool).await.unwrap();
    let adb = adb_list_devices().await.unwrap();

    // thêm 
    for item in db {
        if result.contains_key(&item.serial) == false {
            result.insert(item.serial, item.status);
        }
    }

    for item in adb {
        if result.contains_key(&item.serial) == false {
        }
    }
}


/// Lấy dữ liệu các thiết bị từ database 
async fn db_list_device(pool: SqlitePool) -> Result<Vec<Device>, String>{
    // Lấy dữ liệu từ database
    let mut devices = sqlx::query_as::<_, Device>("SELECT * FROM devices")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(devices)
}

/// lấy danh sách thiết bị đang kết nối 
async fn adb_list_devices() -> Result<Vec<AdbDevice>, String> {
    Adb::list_devices()
}

