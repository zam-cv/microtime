#![allow(unused_variables, unused_imports, dead_code)]
use crate::mqtt::{DS18B20, MAX3010X, MPU6050, REPORT};
use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use anyhow::Result;

pub const HOST: &str = "0.0.0.0";
pub const PORT: &str = "9001";
pub const DATABASE: &str = "drivers";

mod database;
mod messages;
mod mqtt;
mod services;
mod socket;
mod utils;

#[actix::main]
async fn main() -> Result<()> {
    std::env::set_var("TZ", "CST6CDT,M4.1.0,M10.5.0");

    let client = database::init().await?;
    let db = client.database(DATABASE);

    let ds18b20 = db.collection(DS18B20);
    let max3010x = db.collection(MAX3010X);
    let mpu6050 = db.collection(MPU6050);
    let report = db.collection(REPORT);

    let txs = mqtt::init(
        ds18b20.clone(),
        max3010x.clone(),
        mpu6050.clone(),
        report.clone(),
    )
    .await?;
    let socket = socket::Server::new().start();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(txs.clone()))
            .app_data(web::Data::new(socket.clone()))
            .app_data(web::Data::new(max3010x.clone()))
            .app_data(web::Data::new(mpu6050.clone()))
            .app_data(web::Data::new(ds18b20.clone()))
            .app_data(web::Data::new(report.clone()))
            .service(services::temperature::get_values)
            .service(services::report::get_values)
            .service(services::steps::get_values)
            .service(services::heart_rate::get_values)
            .route("/ws/", web::get().to(socket::route))
    })
    .bind((HOST, PORT.parse::<u16>()?))?
    .run()
    .await?;

    Ok(())
}
