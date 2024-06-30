use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::postgres::PgRow;
#[cfg(feature = "ssr")]
use sqlx::{Error, FromRow, Row};
use uuid::Uuid;

use crate::models::category::Category;
use crate::models::fuel::Fuel;
use crate::models::media::Media;
use crate::models::transmission::Transmission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub title: String,
    pub images: Vec<Media>,
    pub video: Media,
    pub category: Category,
    pub fuel: Fuel,
    pub transmission: Transmission,
    pub price: i32,
    pub price_monthly: i32,
    pub year: i32,
    pub mileage: i32,
    pub horsepower: i32,
    pub displacement: i32,
    pub extra: Vec<String>,
    pub model: Uuid,
}

#[cfg(feature = "ssr")]
impl<'r> FromRow<'r, PgRow> for Vehicle {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let image_json: serde_json::Value = row.try_get("image").unwrap();
        let video_json: serde_json::Value = row.try_get("video").unwrap();

        let images = image_json.as_array().unwrap().to_vec().iter().map(|value|
            Media {
                src: value["src"].as_str().unwrap_or("").to_string(),
                alt: value["alt"].as_str().unwrap_or("").to_string(),
            })
            .collect::<Vec<_>>();

        let video = Media {
            src: video_json["src"].as_str().unwrap_or("").to_string(),
            alt: video_json["alt"].as_str().unwrap_or("").to_string(),
        };

        Ok(Vehicle {
            id: row.try_get("id").unwrap(),
            title: row.try_get("title").unwrap(),
            images,
            video,
            category: row.try_get("category").unwrap(),
            fuel: row.try_get("fuel").unwrap(),
            transmission: row.try_get("transmission").unwrap(),
            price: row.try_get("price").unwrap(),
            price_monthly: row.try_get("price_monthly").unwrap(),
            year: row.try_get("year").unwrap(),
            mileage: row.try_get("mileage").unwrap(),
            horsepower: row.try_get("horsepower").unwrap(),
            displacement: row.try_get("displacement").unwrap(),
            extra: row.try_get("extra").unwrap(),
            model: row.try_get("model").unwrap(),
        })
    }
}
