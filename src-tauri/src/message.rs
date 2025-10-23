
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum ToastType {
    Success,
    Error
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ToastPayload {
    pub message: String,
    pub state: ToastType,
}

impl ToastPayload {
    pub fn success(msg: &str) -> Self{
        ToastPayload { 
            message: msg.to_string(), 
            state: ToastType::Success
        }
    }

    pub fn error(msg: &str) -> Self{
        ToastPayload { 
            message: msg.to_string(), 
            state: ToastType::Error
        }
    }
}