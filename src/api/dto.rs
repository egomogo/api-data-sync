use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub documents: HashSet<Document>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub address_name: String,
    pub category_name: String,
    pub id: String,
    pub phone: String,
    pub place_name: String,
    pub place_url: String,
    pub road_address_name: String,
    pub x: String,
    pub y: String,
}

impl Eq for Document {}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Hash for Document {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub is_end: bool,
    pub pageable_count: usize,
    pub total_count: usize,
}
