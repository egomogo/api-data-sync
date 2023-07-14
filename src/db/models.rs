use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Restaurant {
    pub id: String,
    pub name: String,
    pub address: String,
    pub x: f64,
    pub y: f64,
    pub kakao_place_id: String,
    pub api_called_at: chrono::DateTime<chrono::Utc>,
    pub scraped_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Category {
    pub restaurant_id: String,
    pub categories: String,
}
