use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub documents: HashSet<Document>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    address_name: String,
    category_name: String,
    id: String,
    phone: String,
    place_name: String,
    place_url: String,
    road_address_name: String,
    x: String,
    y: String,
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
