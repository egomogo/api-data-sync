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
        #[derive(Eq, PartialEq, Hash)]
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
        ROOT,
        KOREAN, SEA_FOOD, MEAT, NODDLE, RAW_FISH, PORRIDGE, KOREAN_STEW, JAPANESE, TUNA_SASHIMI, SUSHI, PORK_CUTLET_UDON, RAMEN, ALCOHOL, INDOOR_STALLS, HOF_PUB, WINE_BAR, IZAKAYA, COCKTAIL_BAR, SHABU_SHABU, DRIVERS, ASIAN, SOUTH_EAST_ASIAN, INDIAN, CHINESE, LAMB_SKEWERS, LUNCH_BOX, FAST_FOOD, SANDWICH, CAFE_DESSERT, BAKERY, RICE_CAKE, ICE_CREAM, DONUT, TOAST, CHICKEN, SCHOOL_FOOD, WESTERN_FOOD, ITALY, PIZZA, BURGER, STEAK_RIB, MEXICAN, SALAD,
    }
}

#[test]
pub fn test_named_enum() {
    let name = CategoryType::ALCOHOL.name();
    assert_eq!(name, "ALCOHOL");
}
