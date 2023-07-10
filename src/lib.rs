mod error;
mod utils;
mod api;
mod db;
mod types;
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
        Ok(ArgInput {sw, ne})
    }

    fn make_coords_pair(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<(Coords, Coords), error::Error> {
        let pair = (Coords::new(x1, y1)?, Coords::new(x2, y2)?);
        if !pair.0.at_south_west_from(&pair.1) {
            return Ok((pair.1, pair.0));
        }
        Ok(pair)
    }
}


pub async fn run() -> Result<(), error::Error> {
    let config = ArgInput::new()?;
    let set = api::get_from_kakao(config.sw, config.ne).await;
    println!("{set:?} {}", set.len());
    Ok(())
}
