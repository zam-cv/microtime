use crate::messages::{Message, Report};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::doc, Collection};
use std::time::Duration;

#[get("/report")]
pub async fn get_values(data: web::Data<Collection<Message<Report>>>, _: String) -> impl Responder {
    let document = data.get_ref();
    let filter = doc! {
        "headers.timestamp": {
            "$gte": chrono::Utc::now().timestamp() - Duration::from_secs(60 * 60 * 24 * 7).as_secs() as i64
        }
    };

    if let Ok(mut cursor) = document.find(filter, None).await {
        let mut reports = Vec::new();

        while let Ok(has_more) = cursor.advance().await {
            if has_more {
                if let Ok(report) = cursor.deserialize_current() {
                    reports.push(report);
                }
            } else {
                break;
            }
        }

        if let Ok(message) = serde_json::to_string(&reports) {
            return HttpResponse::Ok().body(message);
        }
    }

    HttpResponse::Ok().body("{}")
}
