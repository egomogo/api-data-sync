mod api;
mod db;
mod error;
mod types;
mod utils;
use itertools::Itertools;
use types::*;

struct ArgInput {
    sw: Coords,
    ne: Coords,
}

impl ArgInput {
    fn new() -> Result<ArgInput, error::Error> {
        let mut input = std::env::args()
            .skip(1)
            .take(5)
            .flat_map(|s| s.parse::<f64>());
        let (x1, y1, x2, y2) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );
        let (sw, ne) = Self::make_coords_pair(x1, y1, x2, y2)?;
        Ok(ArgInput { sw, ne })
    }

    fn make_coords_pair(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> Result<(Coords, Coords), error::Error> {
        let pair = (Coords::new(x1, y1)?, Coords::new(x2, y2)?);
        if !pair.0.at_south_west_from(&pair.1) {
            return Ok((pair.1, pair.0));
        }
        Ok(pair)
    }
}

pub async fn run() -> Result<(), error::Error> {
    let config = ArgInput::new()?;
    println!(
        "search for data from kakao in range ({}, {}), ({}, {})",
        config.sw.x, config.sw.y, config.ne.x, config.ne.y
    );
    let set = api::get_from_kakao(config.sw, config.ne).await;
    println!("kakao done. data from kakao: {}", set.len());

    let entry: Vec<(db::models::Restaurant, Vec<db::models::Category>)> = set
        .into_iter()
        .map(|d| {
            let rid = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now();
            let r = db::models::Restaurant {
                id: rid.clone(),
                name: d.place_name,
                address: d.address_name,
                x: d.x.parse().unwrap(),
                y: d.y.parse().unwrap(),
                kakao_place_id: d.id,
                created_at: now,
                updated_at: Some(now.clone()),
            };
            let categories = get_categories_from(d.category_name, &rid);
            (r, categories)
        })
        .collect();

    let db = db::DbPool::new().await?;
    println!("inserting into database...");
    db.insert_all(entry).await?;
    println!("inserting success");
    Ok(())
}

fn get_categories_from(c: String, rid: &str) -> Vec<db::models::Category> {
    c.split(">")
        .map(|s| get_category_from(s, rid))
        .unique_by(|c| c.categories.clone())
        .filter(|c| c.categories.ne(CategoryType::UNDEFINED.name()))
        .collect()
}

fn get_category_from(c: &str, rid: &str) -> db::models::Category {
    db::models::Category {
        restaurant_id: rid.to_string(),
        categories: CategoryType::from(c.trim()).name().to_string(),
    }
}

#[test]
fn test_get_categories_from() {
    let result: Vec<String> = get_categories_from(
        "음식점 > 카페 > 커피전문점 > 커피사피엔스".to_string(),
        "some",
    )
    .into_iter()
    .map(|c| c.categories)
    .collect();
    assert_eq!(result, vec!["CAFE_DESSERT".to_string()]);

    let result: Vec<_> = get_categories_from(
        "음식점 > 양식 > 피자 > 피자스쿨".to_string(), 
        "some"
    )
    .into_iter()
    .map(|c| c.categories)
    .collect();
    assert_eq!(result, vec!["WESTERN_FOOD".to_string(), "PIZZA".to_string()]);
}
