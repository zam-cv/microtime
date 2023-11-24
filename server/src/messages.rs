use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Headers {
    pub timestamp: u64,
}

impl Headers {
    pub fn get_timestamp(&self) -> i64 {
        (self.timestamp as i64) + (chrono::Local::now().offset().local_minus_utc() as i64)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<P> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub headers: Headers,
    pub payload: P
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ds18b20 {
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Max3010x {
    pub red: u32,
    pub ir: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Accel {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mpu6050 {
    pub accel: Accel,
    pub rotation: Rotation,
}