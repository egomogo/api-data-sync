use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

use crate::{error, utils::geo::*};

#[derive(Serialize, Debug, Clone)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

impl Coords {
    pub fn new(x: f64, y: f64) -> Result<Coords, error::Error> {
        assert_long_range(x)?;
        assert_lat_range(y)?;
        Ok(Coords { x, y })
    }

    pub fn at_south_west_from(&self, other: &Coords) -> bool {
        self.x < other.x && self.y < other.y
    }
}

macro_rules! named_enum {
    (
        pub enum $name:ident {
            $($variant:ident),*,
        }
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Eq, PartialEq, Hash, Clone, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

/**
 * db enum
 */
named_enum! {
    pub enum CategoryType {
        OTHERS,
        UNDEFINED,
        KOREAN, SEA_FOOD, MEAT, NODDLE, RAW_FISH, PORRIDGE, KOREAN_STEW,
        JAPANESE, TUNA_SASHIMI, SUSHI, PORK_CUTLET_UDON, RAMEN, SHABU_SHABU,
        ALCOHOL, INDOOR_STALLS, HOF_PUB, WINE_BAR, IZAKAYA, COCKTAIL_BAR,
        DRIVERS,
        ASIAN, SOUTH_EAST_ASIAN, INDIAN,
        CHINESE, LAMB_SKEWERS,
        LUNCH_BOX, FAST_FOOD, SANDWICH,
        CAFE_DESSERT, BAKERY, RICE_CAKE, ICE_CREAM, DONUT, TOAST,
        CHICKEN,
        SCHOOL_FOOD,
        WESTERN_FOOD, ITALY, PIZZA, BURGER, STEAK_RIB, MEXICAN,
        SALAD,
    }
}

static CATEGORY_MAP: &[(&'static str, CategoryType)] = &[
    ("한식", CategoryType::KOREAN),
    ("해물,생선", CategoryType::SEA_FOOD),
    ("육류,고기", CategoryType::MEAT),
    ("국수", CategoryType::NODDLE),
    ("회", CategoryType::RAW_FISH),
    ("죽", CategoryType::PORRIDGE),
    ("찌개, 전골", CategoryType::KOREAN_STEW),
    ("일식", CategoryType::JAPANESE),
    ("초밥,롤", CategoryType::SUSHI),
    ("참치회", CategoryType::TUNA_SASHIMI),
    ("돈까스,우동", CategoryType::PORK_CUTLET_UDON),
    ("일본식라면", CategoryType::RAMEN),
    ("샤브샤브", CategoryType::SHABU_SHABU),
    ("술집", CategoryType::ALCOHOL),
    ("실내포장마차", CategoryType::INDOOR_STALLS),
    ("호프,요리주점", CategoryType::HOF_PUB),
    ("와인바", CategoryType::WINE_BAR),
    ("일본식주점", CategoryType::IZAKAYA),
    ("칵테일바", CategoryType::COCKTAIL_BAR),
    ("기사식당", CategoryType::DRIVERS),
    ("아시아음식", CategoryType::ASIAN),
    ("동남아음식", CategoryType::SOUTH_EAST_ASIAN),
    ("인도음식", CategoryType::INDIAN),
    ("중식", CategoryType::CHINESE),
    ("양꼬치", CategoryType::LAMB_SKEWERS),
    ("도시락", CategoryType::LUNCH_BOX),
    ("패스트푸드", CategoryType::FAST_FOOD),
    ("샌드위치", CategoryType::SANDWICH),
    ("카페", CategoryType::CAFE_DESSERT),
    ("간식", CategoryType::CAFE_DESSERT),
    ("제과,베이커리", CategoryType::BAKERY),
    ("떡,한과", CategoryType::RICE_CAKE),
    ("아이스크림", CategoryType::ICE_CREAM),
    ("도넛", CategoryType::DONUT),
    ("토스트", CategoryType::TOAST),
    ("치킨", CategoryType::CHICKEN),
    ("분식", CategoryType::SCHOOL_FOOD),
    ("양식", CategoryType::WESTERN_FOOD),
    ("이탈리안", CategoryType::ITALY),
    ("피자", CategoryType::PIZZA),
    ("햄버거", CategoryType::BURGER),
    ("스테이크,립", CategoryType::STEAK_RIB),
    ("멕시칸,브라질", CategoryType::MEXICAN),
    ("샐러드", CategoryType::SALAD),
    // 특이
    ("퓨전요리", CategoryType::OTHERS),
];

impl From<&str> for CategoryType {
    fn from(value: &str) -> Self {
        let idx = CATEGORY_MAP.iter().filter(|e| e.0 == value).next();
        match CATEGORY_MAP.iter().find(|e| e.0 == value) {
            Some((.., t)) => t.clone(),
            None => CategoryType::UNDEFINED,
        }
    }
}

#[test]
pub fn test_static_map() {
    let c = CategoryType::from("떡,한과");
    assert_eq!(c, CategoryType::RICE_CAKE);
}

#[test]
pub fn test_named_enum() {
    let name = CategoryType::ALCOHOL.name();
    assert_eq!(name, "ALCOHOL");
}
