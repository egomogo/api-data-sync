pub mod dto;

#[allow(unused_imports)]
use dotenv::dotenv;
use dto::*;
use std::collections::HashSet;

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
    pub async fn get(
        &self,
        category: Category,
        swx: f64,
        swy: f64,
        nex: f64,
        ney: f64,
    ) -> Result<HashSet<Document>, Box<dyn std::error::Error>> {
        let data = self
            .client
            .get(Self::url())
            .header("Authorization", format!("KakaoAK {}", Self::api_key()))
            .query(&[
                ("category_group_code", category.code().as_str()),
                ("rect", format!("{swx},{swy},{nex},{ney}").as_str()),
                ("radius", "1000"),
            ])
            .send()
            .await?
            .json::<ResponseBody>()
            .await?;
        Ok(data.documents)
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
    let result = kakao_client
        .get(
            Category::Restaurant,
            127.164581,
            37.604747,
            127.171844,
            37.612175,
        )
        .await;
    match &result {
        Ok(v) => println!("{v:?}"),
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
