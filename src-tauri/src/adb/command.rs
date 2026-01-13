use tokio::process::Command;

use crate::adb::args::{ADB, DEVICE, DEVICES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdbDevice {
    serial: String
}

#[derive(Debug, Clone)]
pub struct Adb;

impl Adb {
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

    pub async fn list_devices() -> Result<Vec<AdbDevice>, String> {
        let stdout = Self::execute(&[DEVICES]).await?;
        let mut devices = Vec::new();

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >=2 && parts[1] == DEVICE {
                devices.push(AdbDevice {
                    serial: parts[0].to_string()
                });
            } 
        }

        Ok(devices)
    }
}