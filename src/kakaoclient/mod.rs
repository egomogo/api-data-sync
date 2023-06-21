use core::panic;

#[allow(unused_imports)]
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::utils;

pub struct Kakao {
    client: reqwest::Client,
}

impl Kakao {
    pub fn new() -> Kakao {
        Kakao {
            client: reqwest::Client::new(),
        }
    }
    pub async fn get_by_category(
        &self,
        category: Category,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let data = self
            .client
            .get(Self::url())
            .header("Authorization", format!("KakaoAK {}", Self::api_key()))
            .query(&[
                ("category_group_code", category.code().as_str()),
                ("x", "37.5796"),
                ("y", "126.9227"),
                ("radius", "1000"),
            ])
            .send()
            .await?
            .json::<ResponseBody>()
            .await?;
        match serde_json::to_string(&data) {
            Ok(v) => Ok(v),
            Err(e) => panic!("{e:?}"),
        }
    }
    fn url() -> String {
        utils::Const::KakaoRestApiUrl.value()
    }
    fn api_key() -> String {
        utils::Const::KakaoRestApiKey.value()
    }
}

#[test]
fn test_url() {
    dotenv().ok();
    assert_eq!(
        Kakao::url(),
        "https://dapi.kakao.com/v2/local/search/category.json"
    )
}

#[tokio::test]
async fn test_get_by_category() {
    dotenv().ok();
    let kakao_client = Kakao::new();
    let result = kakao_client.get_by_category(Category::Restaurant).await;
    match &result {
        Ok(v) => println!("{v}"),
        Err(e) => panic!("{e:?}"),
    }
    assert!(result.is_ok());
}

#[allow(dead_code)]
pub enum Category {
    Supermarket,
    ConvinienceStore,
    Kindergarden,
    School,
    Academy,
    ParkingLot,
    GasStation,
    SubwayStation,
    Bank,
    CulturalFactilities,
    Brokerage,
    PublicInstitutions,
    Attractions,
    Lodgment,
    Restaurant,
    Cafe,
    Hospital,
    Pharmacy,
}

impl Category {
    pub fn code(&self) -> String {
        match self {
            Self::Supermarket => "MT1",
            Self::ConvinienceStore => "CS2",
            Self::Kindergarden => "PS3",
            Self::School => "SC4",
            Self::Academy => "AC5",
            Self::ParkingLot => "PK6",
            Self::GasStation => "OL7",
            Self::SubwayStation => "SW8",
            Self::Bank => "BK9",
            Self::CulturalFactilities => "CT1",
            Self::Brokerage => "AG2",
            Self::PublicInstitutions => "PO3",
            Self::Attractions => "AT4",
            Self::Lodgment => "AD5",
            Self::Restaurant => "FD6",
            Self::Cafe => "CE7",
            Self::Hospital => "HP8",
            Self::Pharmacy => "PM9",
        }
        .to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    documents: Vec<Document>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    is_end: Option<bool>,
    pegeable_count: Option<usize>,
    total_count: usize,
}
