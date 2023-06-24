pub mod dto;
pub mod error;

#[allow(unused_imports)]
use dotenv::dotenv;
use dto::*;
use std::collections::HashSet;

use crate::utils;

macro_rules! unwrap_result_or {
    ($e: expr, $or: expr) => {
        match $e {
            Ok(v) => v,
            Err(..) => $or,
        }
    };
}

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
        category: &Category,
        swx: f64,
        swy: f64,
        nex: f64,
        ney: f64,
    ) -> HashSet<Document> {
        let mut stack = vec![(swx, swy, nex, ney)];
        let mut result = HashSet::new();

        while let Some((swx, swy, nex, ney)) = stack.pop() {
            let mut page = 1;
            let size = 15;
            let body = unwrap_result_or!(
                self.get_body(&category, swx, swy, nex, ney, page, size)
                    .await,
                continue
            );
            let (documents, meta) = (body.documents, body.meta);
            let (pageable_count, total_count, ..) =
                (meta.pageable_count, meta.total_count, meta.is_end);
            println!("depth: {}, {meta:?}", stack.len());
            if pageable_count < total_count {
                let w = (nex - swy).abs();
                let h = (ney - swy).abs();
                match w > h {
                    true => {
                        stack.push((swx, swy, (nex + swx) / 2.0, ney));
                        stack.push(((nex + swx) / 2.0, swy, nex, ney));
                    }
                    false => {
                        stack.push((swx, (ney + swy) / 2.0, nex, ney));
                        stack.push((swx, swy, nex, (ney + swy) / 2.0));
                    }
                }
                continue;
            }
            let mut remain: isize = (pageable_count - documents.len()) as isize;
            // println!("{remain} remains. {page}th page will be req");
            result.extend(documents);
            page += 1;
            while remain > 0 {
                let body = unwrap_result_or!(
                    self.get_body(&category, swx, swy, nex, ney, page, size)
                        .await,
                    break
                );
                let documents = body.documents;
                result.extend(documents);
                remain -= size as isize;
                // println!("{remain} remains. {page}th page will be req");
                if body.meta.is_end {
                    break;
                }
                page += 1;
            }
            println!("{}", result.len());
        }

        result
    }

    async fn get_body(
        &self,
        category: &Category,
        swx: f64,
        swy: f64,
        nex: f64,
        ney: f64,
        page: usize,
        size: usize,
    ) -> Result<ResponseBody, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get(Self::url())
            .header("Authorization", format!("KakaoAK {}", Self::api_key()))
            .query(&[
                ("category_group_code", category.code().as_str()),
                ("rect", format!("{swx},{swy},{nex},{ney}").as_str()),
                ("page", page.to_string().as_str()),
                ("size", size.to_string().as_str()),
            ])
            .send()
            .await?
            .json::<ResponseBody>()
            .await?)
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
async fn test_get_body() {
    dotenv().ok();
    let kakao_client = Kakao::new();
    let meta = kakao_client
        .get_body(
            &Category::Restaurant,
            126.916080,
            37.574244,
            126.928096,
            37.584311,
            1,
            15,
        )
        .await
        .unwrap()
        .meta;
    println!("{meta:?}");
    let meta = kakao_client
        .get_body(
            &Category::Restaurant,
            126.916080,
            37.574244,
            126.920000,
            37.58,
            1,
            15,
        )
        .await
        .unwrap()
        .meta;
    println!("{meta:?}");
}

#[tokio::test]
async fn test_get_by_category() {
    dotenv().ok();
    let kakao_client = Kakao::new();
    let result = kakao_client
        .get(
            &Category::Restaurant,
            126.898128,
            37.500534,
            127.077195,
            37.632722,
        )
        .await;
    println!("{}", result.len());
    // println!("{result:?}");
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
