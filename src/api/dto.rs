use std::{collections::HashSet, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub documents: HashSet<Document>,
    meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    address_name: Option<String>,
    category_name: Option<String>,
    id: Option<String>,
    phone: Option<String>,
    place_name: Option<String>,
    place_url: Option<String>,
    road_address_name: Option<String>,
    x: Option<String>,
    y: Option<String>,
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
    is_end: Option<bool>,
    pegeable_count: Option<usize>,
    total_count: usize,
}
