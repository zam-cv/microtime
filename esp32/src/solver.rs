use crate::{
    client::Client,
    handlers::{ds18b20::Ds18b20, max3010x::Max3010x, mpu6050::Mpu6050},
    network::Network,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub const SOCKET: &str = "socket";
pub const DATABASE: &str = "database";

pub const DS18B20: &str = "ds18b20";
pub const MAX3010X: &str = "max3010x";
pub const MPU6050: &str = "mpu6050";

pub const RED_UPDATES: [&str; 2] = [SOCKET, DATABASE];
pub const DRIVERS: [&str; 3] = [DS18B20, MAX3010X, MPU6050];

const LIMIT: usize = 3000;

#[derive(Serialize, Deserialize)]
pub struct Headers {
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub headers: Headers,
    pub payload: Driver,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Driver {
    Ds18b20(Ds18b20),
    Max3010x(Max3010x),
    Mpu6050(Mpu6050),
}

pub struct Solver {
    pub client: Arc<Mutex<Option<Client>>>,
    pub network: Arc<Network>,
    pub storage: Mutex<Vec<(String, Message)>>,
}

unsafe impl Send for Solver {}
unsafe impl Sync for Solver {}

impl Into<Driver> for Mpu6050 {
    fn into(self) -> Driver {
        Driver::Mpu6050(self)
    }
}

impl Into<Driver> for Max3010x {
    fn into(self) -> Driver {
        Driver::Max3010x(self)
    }
}

impl Into<Driver> for Ds18b20 {
    fn into(self) -> Driver {
        Driver::Ds18b20(self)
    }
}

impl Message {
    pub fn new<P: Into<Driver>>(payload: P) -> Self {
        Self {
            headers: Headers {
                timestamp: chrono::Local::now().timestamp(),
            },
            payload: payload.into(),
        }
    }
}

impl Driver {
    pub fn get_topic(&self) -> &'static str {
        match self {
            Driver::Ds18b20(_) => DS18B20,
            Driver::Max3010x(_) => MAX3010X,
            Driver::Mpu6050(_) => MPU6050,
        }
    }
}

impl Solver {
    pub fn new(client: Arc<Mutex<Option<Client>>>, network: Arc<Network>) -> Result<Self> {
        Ok(Self {
            client,
            storage: Mutex::new(Vec::new()),
            network,
        })
    }

    pub fn send_to_database(&self, message: Message) -> Result<()> {
        let route = format!("{}/{}", DATABASE, message.payload.get_topic());
        if let Ok(mut client_opt) = self.client.lock() {
            if let Some(client) = client_opt.as_mut() {
                // if self.network.is_connected()? {
                let message = serde_json::to_string(&message)?;

                if let Ok(mut storage) = self.storage.lock() {
                    for (_route, mut _message) in storage.drain(..) {
                        let now = chrono::Local::now().timestamp() as i64;
                        _message.headers.timestamp = (now - _message.headers.timestamp) + now;

                        let _message = serde_json::to_string(&_message)?;
                        client.publish(&_route, &_message)?;
                    }
                }

                client.publish(&route, message.as_str())?;
                return Ok(());
                // } else {
                //     *client_opt = None;
                // }
            }

            if let Ok(mut storage) = self.storage.lock() {
                if storage.len() < LIMIT {
                    storage.insert(0, (route, message));
                } else {
                    storage.remove(0);
                    storage.insert(0, (route, message));
                }
            }
        }

        Ok(())
    }

    pub fn send_to_socket(&self, message: Message) -> Result<()> {
        let route = format!("{}/{}", SOCKET, message.payload.get_topic());
        let message = serde_json::to_string(&message)?;

        if let Ok(mut client) = self.client.lock() {
            if let Some(client) = client.as_mut() {
                client.publish(&route, message.as_str())?;
            }
        }

        Ok(())
    }
}
