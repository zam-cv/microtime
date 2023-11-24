use anyhow::Result;
use esp_idf_svc::mqtt::client::*;
use esp_idf_sys::*;
use log::*;
use std::{sync::Mutex, thread};

pub struct MessageData<'a> {
    pub data: &'a [u8],
}

impl<'b, 'a: 'b> MessageData<'a> {
    pub fn to_str(&self) -> &'b str {
        self.data.as_ascii().unwrap_or_default().as_str()
    }
}

pub struct Client {
    client: Mutex<EspMqttClient<'static, ConnState<MessageImpl, EspError>>>,
}

impl Client {
    pub fn new(
        id: &str,
        host: &str,
        port: &str,
        mut callback: impl FnMut(Option<&str>, MessageData) + Send + 'static,
    ) -> Result<Self> {
        info!("About to start MQTT client");

        let conf = MqttClientConfiguration {
            client_id: Some(id),
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Default::default()
        };

        let (client, mut connection) =
            EspMqttClient::new_with_conn(&format!("mqtt://{}:{}", host, port), &conf)?;

        info!("MQTT client started");

        thread::spawn(move || {
            info!("MQTT Listening for messages");

            while let Some(msg) = connection.next() {
                match msg {
                    Err(e) => info!("MQTT Message ERROR: {}", e),
                    Ok(msg) => match msg as Event<MessageImpl> {
                        Event::Received(msg) => {
                            callback(msg.topic(), MessageData { data: msg.data() });
                        }
                        _ => {}
                    },
                }
            }

            info!("MQTT connection loop exit");
        });

        Ok(Self {
            client: Mutex::new(client),
        })
    }

    pub fn subscribe(&self, topic: &str) -> Result<()> {
        info!("MQTT Subscribing to {}", topic);

        if let Ok(mut client) = self.client.lock() {
            if let Err(e) = client.subscribe(topic, QoS::AtMostOnce) {
                info!("MQTT Subscribe ERROR: {}", e);
            }
        }

        Ok(())
    }

    pub fn publish(&self, topic: &str, message: &str) -> Result<()> {
        if let Ok(mut client) = self.client.lock() {
            if let Err(e) = client.publish(topic, QoS::AtMostOnce, true, message.as_bytes()) {
                info!("MQTT Subscribe ERROR: {}", e);
            }
        }

        Ok(())
    }
}
