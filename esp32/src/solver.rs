use crate::{client::Client, CLIENT_ID, HOST, PORT};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SOCKET: &str = "socket";
pub const DATABASE: &str = "database";

pub const DS18B20: &str = "ds18b20";
pub const MAX3010X: &str = "max3010x";
pub const MPU6050: &str = "mpu6050";

pub const RED_UPDATES: [&str; 2] = [SOCKET, DATABASE];
pub const DRIVERS: [&str; 3] = [DS18B20, MAX3010X, MPU6050];

#[derive(Serialize, Deserialize)]
pub struct Headers {
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Message<P> {
    pub headers: Headers,
    pub payload: P,
}

pub enum Driver {
    Ds18b20,
    Max3010x,
    Mpu6050,
}

pub enum Update<P> {
    Database(Message<P>),
    Socket(Message<P>),
    Storage(Message<P>),
    Auto(Message<P>),
    None,
}

pub struct Solver {
    pub client: Client,
}

unsafe impl Send for Solver {}
unsafe impl Sync for Solver {}

impl Solver {
    pub fn new() -> Result<Self> {
        let client = Client::new(CLIENT_ID, HOST, PORT, |_, _| {})?;

        for update in RED_UPDATES {
            for driver in DRIVERS {
                client.subscribe(&format!("{}/{}", update, driver))?
            }
        }

        Ok(Self { client })
    }

    pub fn send<P: Serialize>(&self, driver: Driver, update: Update<P>) -> Result<()> {
        let topic = match driver {
            Driver::Ds18b20 => DS18B20,
            Driver::Max3010x => MAX3010X,
            Driver::Mpu6050 => MPU6050,
        };

        match update {
            Update::Database(message) => {
                let message = serde_json::to_string(&message)?;
                self.client
                    .publish(&format!("{}/{}", DATABASE, topic), message.as_str())
            }
            Update::Socket(message) => {
                let message = serde_json::to_string(&message)?;
                self.client
                    .publish(&format!("{}/{}", SOCKET, topic), message.as_str())
            }
            Update::Storage(_) => Ok(()),
            Update::Auto(_) => Ok(()),
            Update::None => Ok(()),
        }
    }
}
