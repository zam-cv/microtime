use mongodb::{
    bson::{self, doc, document, Bson},
    Collection,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error, time::Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    pub _id: f32,
    pub average: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub id: f32,
    pub group: u32,
    pub average: f64,
}

pub fn get_fragment(unit: &str) -> (u32, u32) {
    match unit {
        "day" => ((60 * 60 * 24) / 6, 6),
        "week" => (60 * 60 * 24, 7),
        "month" => ((60 * 60 * 24 * 30) / 4, 4),
        "year" => ((60 * 60 * 24 * 365) / 4, 4),
        _ => (60 * 60 * 24 / 6, 6),
    }
}

pub fn get_start(fragment: &(u32, u32)) -> i64 {
    chrono::Utc::now().timestamp()
        - Duration::from_secs(fragment.0 as u64 * fragment.1 as u64).as_secs() as i64
}

pub async fn get_range_average<T>(
    document: &Collection<T>,
    property: &str,
    start: &i64,
    fragment: &(u32, u32),
) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut raw = document
        .aggregate(
            [
                doc! {
                    "$match": {
                        "headers.timestamp": {
                            "$gte": start,
                            "$lte": chrono::Utc::now().timestamp()
                        }
                    }
                },
                doc! {
                    "$group": {
                        "_id": {
                            "$floor": {
                                "$divide": [
                                    { "$subtract": [ "$headers.timestamp", start ] },
                                    fragment.0
                                ]
                            }
                        },
                        "average": { "$avg": property }
                    }
                },
                doc! {
                    "$sort": { "_id": 1 }
                },
            ],
            None,
        )
        .await?;

    let mut messages: Vec<Value> = Vec::new();

    while let Ok(has_more) = raw.advance().await {
        if has_more {
            if let Ok(message) = raw.deserialize_current() {
                messages.push(bson::from_bson(Bson::Document(message))?)
            }
        } else {
            break;
        }
    }

    return Ok(messages);
}

pub fn normalize(messages: &mut Vec<Value>, fragment: &(u32, u32), start: i64) -> Vec<Data> {
    let mut values = Vec::new();
    let mut ids = HashSet::new();

    for message in messages {
        values.push(Data {
            id: message._id + 1.0,
            group: (fragment.0 * message._id as u32) + start as u32,
            average: message.average,
        });

        ids.insert((message._id + 1.0) as u32);
    }

    let average = values.iter().map(|v| v.average).sum::<f64>() / values.len() as f64;

    for id in 0..fragment.1 {
        if !ids.contains(&(id + 1)) {
            values.push(Data {
                id: id as f32 + 1.0,
                group: (fragment.0 * id as u32) + start as u32,
                average,
            });
        }
    }

    values.sort_by(|a, b| {
        if a.id < b.id {
            std::cmp::Ordering::Less
        } else if a.id > b.id {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    return values;
}
