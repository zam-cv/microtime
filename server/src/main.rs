#![allow(unused_variables, unused_imports, dead_code)]
use actix::Actor;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use std::sync::{atomic::AtomicUsize, Arc};
use tokio::sync::broadcast;

pub const HOST: &str = "0.0.0.0";
pub const PORT: &str = "9001";
pub const DATABASE: &str = "drivers";

mod database;
mod messages;
mod mqtt;
mod socket;

#[actix::main]
async fn main() -> Result<()> {
    std::env::set_var("TZ", "CST6CDT,M4.1.0,M10.5.0");

    let client = database::init().await?;
    let db = client.database(DATABASE);
    let txs = mqtt::init(db).await?;
    let server = socket::Server::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(txs.clone()))
            .app_data(web::Data::new(server.clone()))
            .route("/ws/", web::get().to(socket::route))
    })
    .bind((HOST, PORT.parse::<u16>()?))?
    .run()
    .await?;

    Ok(())
}
