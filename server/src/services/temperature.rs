use crate::messages::{Ds18b20, Message};
use crate::utils;
use actix_web::{post, web, Either, HttpResponse, Responder, Result};
use lazy_static::lazy_static;
use mongodb::{
    bson::{self, doc, Bson},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error, time::Duration};

#[post("/temperature")]
pub async fn get_values(
    data: web::Data<Collection<Message<Ds18b20>>>,
    req_body: String,
) -> Result<impl Responder, Box<dyn Error>> {
    let req = serde_json::from_str::<utils::Request>(&req_body)?;

    let fragment = utils::get_fragment(&req.unit);
    let start = utils::get_start(&fragment);
    let mut messages =
        utils::get_range_average(&data, "$payload.temperature", &start, &fragment).await?;
    let values = utils::normalize(&mut messages, &fragment, start);

    Ok(HttpResponse::Ok().body(serde_json::to_string(&values)?))
}