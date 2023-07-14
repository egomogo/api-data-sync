pub mod dto;

#[allow(unused_imports)]
use dotenv::dotenv;
use dto::*;
use std::collections::HashSet;

use crate::{types::*, utils};

macro_rules! unwrap_result_or {
    ($e: expr, $or: expr) => {
        match $e {
            Ok(v) => v,
            Err(..) => $or,
        }
    };
}

pub async fn get_from_kakao(sw: Coords, ne: Coords) -> HashSet<Document> {
    let kakao = Kakao::new();
    let mut restaurants = kakao
        .get(&Category::Restaurant, sw.x, sw.y, ne.x, ne.y)
        .await;
    println!("{:?}: {}", Category::Restaurant, restaurants.len());
    let cafe = kakao.get(&Category::Cafe, sw.x, sw.y, ne.x, ne.y).await;
    println!("{:?}: {}", Category::Cafe, cafe.len());
    restaurants.extend(cafe);
    restaurants
}

struct Kakao {
    client: reqwest::Client,
}

impl Kakao {
    pub fn new() -> Kakao {
        Kakao {
            client: reqwest::Client::new(),
        }
    }

    async fn get(
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
                self.get_body(category, swx, swy, nex, ney, page, size)
                    .await,
                continue
            );
            let (documents, meta) = (body.documents, body.meta);
            let (pageable_count, total_count, ..) =
                (meta.pageable_count, meta.total_count, meta.is_end);
            // println!("depth: {}, {meta:?}", stack.len());
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
                    self.get_body(category, swx, swy, nex, ney, page, size)
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
            // println!("{}", result.len());
        }

        result
    }

    #[allow(clippy::too_many_arguments)]
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

#[tokio::main]
#[test]
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

#[tokio::main]
#[test]
async fn test_get_by_category() {
    use std::collections::HashMap;
    dotenv().ok();
    let kakao_client = Kakao::new();
    let result = kakao_client
        .get(
            &Category::Cafe,
            126.907418,
            37.569670,
            126.938746,
            37.585196,
        )
        .await;
    let mut group = HashMap::new();
    result.iter().for_each(|d| {
        let parsed = match d.category_name.split(" > ").skip(1).next() {
            Some(v) => v,
            None => d.category_name.as_str(),
        };
        group.entry(parsed).or_insert(vec![]).push(d);
    });
    for (k, v) in group.iter() {
        println!("{k}: {}", v.len());
        for d in v.iter() {
            println!("{} {}", d.place_name, d.category_name);
        }
        println!();
    }
    // println!("{:?}", group);
    // println!("{}", result.len());
    // println!("{result:?}");
}

#[allow(dead_code)]
#[derive(Debug)]
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
