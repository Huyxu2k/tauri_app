// Cấu hình số luồng chạy 

use serde::Serialize;


#[derive(Serialize)]
pub struct Setting{
    pub id: i32,
    pub threads: i32,
    pub domain: String,
}


pub async fn save_settings(){

}

pub async fn load_settings(){

}