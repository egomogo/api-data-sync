mod kakaoclient;
mod utils;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    Ok(())
}

#[test]
fn dotenv_test() {
    dotenv().ok();
    assert_eq!(std::env::var("TEST_KEY"), Ok("TEST_VALUE".to_owned()));
}

#[test]
fn utils_const() {
    dotenv().ok();
    assert_eq!(utils::Const::TestKey.value(), "TEST_VALUE".to_owned());
}
