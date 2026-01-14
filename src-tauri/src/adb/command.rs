use tokio::process::Command;

use crate::adb::args::{ADB, DEVICE, DEVICES, PULL, PUSH, VERSION};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdbDevice {
    serial: String,
}

#[derive(Debug, Clone)]
pub struct AdbVersion {
    bridge_version: String,
    version: String,
    installed_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Adb;

impl Adb {

    /// hàm chạy các lệnh adb
    /// ## return
    pub async fn execute(args: &[&str]) -> Result<String, String> {
        let output = Command::new(ADB)
            .args(args)
            .output()
            .await
            .map_err(|e| format!("Không thể thực thi câu lệnh: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// kiểm tra phiên bản của Adb
    /// ## return
    /// [`AdbVersion`]
    pub async fn get_version() -> Result<AdbVersion, String> {
        let stdout = Self::execute(&[VERSION]).await?;

        let mut bridge_version: Option<String> = None;
        let mut version: Option<String> = None;
        let mut installed_path: Option<String> = None;

        for line in stdout.lines() {
            let line = line.trim();

            if line.starts_with("Android Debug Bridge version") {
                bridge_version = line.split_whitespace().last().map(|s| s.to_string());
            } else if line.starts_with("Version") {
                version = line.split_whitespace().nth(1).map(|s| s.to_string());
            } else if line.starts_with("Installed as") {
                installed_path = Some(line.strip_prefix("Installed as ").unwrap_or("").to_string());
            }
        }

        match (bridge_version, version) {
            (Some(bridge_version), Some(version)) => Ok(AdbVersion {
                bridge_version,
                version,
                installed_path,
            }),
            _ => Err("Không tìm thấy phiên bản của Adb.".to_string()),
        }
    }

    /// lấy các thiết bị đang kết nối
    /// ## return
    /// Vec<[`AdbDevice`]>
    pub async fn list_devices() -> Result<Vec<AdbDevice>, String> {
        let stdout = Self::execute(&[DEVICES]).await?;
        let mut devices = Vec::new();

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == DEVICE {
                devices.push(AdbDevice {
                    serial: parts[0].to_string(),
                });
            }
        }

        Ok(devices)
    }

    /// copy từ máy tính sang điện thoại
    /// ## arguments
    /// * `source`: đường dẫn file trên máy tính
    /// * `directory`: đường dẫn như mục trên điện thoại
    /// ## return
    /// `true` nếu thành công
    pub async fn push(source: &str, directory: &str) -> Result<bool, String> {
        let result = Self::execute(&[PUSH, source, directory])
                                            .await;

        match result {
            Ok(out) => Ok(!out.is_empty()),
            Err(_) => Err(format!("Không thể copy file {} từ máy tính sang điện thoại.", source)),
        }
    }

    // copy từ điện thoại sang máy tính
    /// ## arguments
    /// * `source`: đường dẫn file trên điện thoại
    /// * `directory`: đường dẫn như mục trên máy tính
    /// ## return
    /// `true` nếu thành công
    pub async fn pull(source: &str, directory: &str) -> Result<bool, String> {
        let result = Self::execute(&[PULL, source, directory])
                                            .await;
        match result {
            Ok(out) => Ok(!out.is_empty()),
            Err(_) => Err(format!("Không thể copy file {} từ điện thoại sang máy tính.", source)),
        }
    }

    pub async fn install() -> Result<bool, String>{

    }
}
