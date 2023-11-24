use crate::messages::{Ds18b20, Max3010x, Message, Mpu6050};
use anyhow::Result;
use mongodb::{Collection, Database};
use rumqttc::v5::{
    mqttbytes::{v5::Publish, QoS},
    AsyncClient, Event, Incoming, MqttOptions,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    sync::broadcast::{self, Sender},
    task,
};

pub const CLIENT_ID: &str = "server";
pub const HOST: &str = "localhost";
pub const PORT: &str = "1884";

pub const SOCKET: &str = "socket";
pub const DATABASE: &str = "database";

pub const DS18B20: &str = "ds18b20";
pub const MAX3010X: &str = "max3010x";
pub const MPU6050: &str = "mpu6050";

pub const RED_UPDATES: [&str; 2] = [SOCKET, DATABASE];
pub const DRIVERS: [&str; 3] = [DS18B20, MAX3010X, MPU6050];

pub async fn handle(
    publish: &Publish,
    txs: Arc<HashMap<String, Sender<String>>>,
    ds18b20: &Collection<Message<Ds18b20>>,
    max3010x: &Collection<Message<Max3010x>>,
    mpu6050: &Collection<Message<Mpu6050>>,
) -> Result<()> {
    let topic = std::str::from_utf8(&publish.topic)?;
    let payload = std::str::from_utf8(&publish.payload)?.to_string();
    let routes = topic.split('/').collect::<Vec<&str>>();

    if routes.len() == 2 {
        let update = routes[0];
        let driver = routes[1];

        if let Some(tx) = txs.get(&driver.to_string()) {
            println!("{}", &payload);

            match update {
                DATABASE => match driver {
                    DS18B20 => {
                        let message = serde_json::from_str::<Message<Ds18b20>>(&payload)?;
                        ds18b20.insert_one(message, None).await?;
                    }
                    MAX3010X => {
                        let message = serde_json::from_str::<Message<Max3010x>>(&payload)?;
                        max3010x.insert_one(message, None).await?;
                    }
                    MPU6050 => {
                        let message = serde_json::from_str::<Message<Mpu6050>>(&payload)?;
                        mpu6050.insert_one(message, None).await?;
                    }
                    _ => {}
                },
                SOCKET => {
                    let _ = tx.send(payload);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub async fn init(db: Database) -> Result<Arc<HashMap<String, Sender<String>>>> {
    let mut mqttoptions = MqttOptions::new(CLIENT_ID, HOST, PORT.parse::<u16>()?);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let mut txs = HashMap::new();


    for &update in RED_UPDATES.iter() {
        for &driver in DRIVERS.iter() {
            client.subscribe(&format!("{}/{}", update, driver), QoS::AtMostOnce).await?;
        }
    }

    for &driver in DRIVERS.iter() {
        let (tx, _) = broadcast::channel(10);
        txs.insert(driver.to_string(), tx);
    }

    let txs = Arc::new(txs);
    let txs_clone = Arc::clone(&txs);

    let ds18b20 = db.collection(DS18B20);
    let max3010x = db.collection(MAX3010X);
    let mpu6050 = db.collection(MPU6050);

    task::spawn(async move {
        while let Ok(event) = eventloop.poll().await {
            if let Event::Incoming(Incoming::Publish(publish)) = event {
                if let Err(e) = handle(
                    &publish,
                    Arc::clone(&txs_clone),
                    &ds18b20,
                    &max3010x,
                    &mpu6050,
                )
                .await
                {
                    println!("Error: {}", e);
                }

                let c = client.clone();
                tokio::spawn(async move {
                    if let Err(e) = c.ack(&publish).await {
                        println!("Error: {}", e);
                    }
                });
            }
        }
    });

    Ok(txs)
}
