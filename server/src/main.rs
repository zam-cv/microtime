#![allow(unused_variables, unused_imports, dead_code)]
use crate::mqtt::{DS18B20, MAX3010X, MPU6050};
use actix::Actor;
use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use messages::{Ds18b20, Message};
use mongodb::Collection;
use std::sync::{atomic::AtomicUsize, Arc};
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

pub const HOST: &str = "0.0.0.0";
pub const PORT: &str = "9001";
pub const DATABASE: &str = "drivers";

mod database;
mod messages;
mod mqtt;
mod socket;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    pub _type: String,
}

#[post("/temperature")]
async fn temperature(
    data: web::Data<Collection<Message<Ds18b20>>>,
    req_body: String,
) -> impl Responder {
    let req = serde_json::from_str::<Request>(&req_body);

    if let Ok(req) = req {
        println!("{:?}", req);
    }

    if let Ok(list) = data.get_ref().find_one(None, None).await {
        if let Some(list) = list {
            if let Ok(message) = serde_json::to_string(&list) {
                return HttpResponse::Ok().body(message);
            }
        }
    }

    HttpResponse::Ok().body("{}")
}

#[actix::main]
async fn main() -> Result<()> {
    std::env::set_var("TZ", "CST6CDT,M4.1.0,M10.5.0");

    let client = database::init().await?;
    let db = client.database(DATABASE);

    let ds18b20 = db.collection(DS18B20);
    let max3010x = db.collection(MAX3010X);
    let mpu6050 = db.collection(MPU6050);

    let txs = mqtt::init(ds18b20.clone(), max3010x.clone(), mpu6050.clone()).await?;
    let socket = socket::Server::new().start();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(txs.clone()))
            .app_data(web::Data::new(socket.clone()))
            .app_data(web::Data::new(ds18b20.clone()))
            .service(temperature)
            .route("/ws/", web::get().to(socket::route))
    })
    .bind((HOST, PORT.parse::<u16>()?))?
    .run()
    .await?;

    Ok(())
}
