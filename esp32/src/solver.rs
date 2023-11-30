use crate::{
    client::Client,
    handlers::{ds18b20::Ds18b20, max3010x::Max3010x, mpu6050::Mpu6050, button::Report},
    network::Network,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub const SOCKET: &str = "socket";
pub const DATABASE: &str = "database";
pub const RED_UPDATES: [&str; 2] = [SOCKET, DATABASE];

const LIMIT: usize = 3000;

macro_rules! count_idents {
    ($($idents:ident),*) => {
        {
            #[allow(dead_code, non_camel_case_types)]
            enum Idents { $($idents,)* __CountIdentsLast }
            const COUNT: usize = Idents::__CountIdentsLast as usize;
            COUNT
        }
    }
}

macro_rules! set_payloads {
    ($($payload:ident),*) => {
        #[derive(Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Payload {
            $(
                $payload($payload),
            )*
        }

        $(
            impl Into<Payload> for $payload {
                fn into(self) -> Payload {
                    Payload::$payload(self)
                }
            }
        )*

        impl Payload {
            pub fn get_topic(&self) -> String {
                match self {
                    $(
                        Payload::$payload(_) => stringify!($payload).to_lowercase(),
                    )*
                }
            }
        }

        // $(
        //     pub const $payload: &str = stringify!($payload);
        // )*

        pub const PAYLOADS: [&str; count_idents!($($payload),*)] = [$(stringify!($payload)),*];
    }
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub headers: Headers,
    pub payload: Payload,
}

pub struct Solver {
    pub client: Arc<Mutex<Option<Client>>>,
    pub network: Arc<Network>,
    pub storage: Mutex<Vec<(String, Message)>>,
}

unsafe impl Send for Solver {}
unsafe impl Sync for Solver {}

set_payloads!(Ds18b20, Max3010x, Mpu6050, Report);

impl Message {
    pub fn new<P: Into<Payload>>(payload: P) -> Self {
        Self {
            headers: Headers {
                timestamp: chrono::Local::now().timestamp(),
            },
            payload: payload.into(),
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
