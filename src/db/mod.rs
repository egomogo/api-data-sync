pub mod models;

use sqlx::MySqlPool;

use crate::utils::*;

#[tokio::main]
#[test]
pub async fn test_connection() -> Result<(), sqlx::Error> {
    use dotenv;
    use models::*;
    dotenv::dotenv().ok();

    let url = Const::DB_URL.value();

    let pool = MySqlPool::connect(&url).await?;

    sqlx::query("delete from restaurant where id=?")
        .bind("some_id")
        .execute(&pool)
        .await?;

    sqlx::query("insert into restaurant values (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind("some_id")
        .bind("some_name")
        .bind("some_address")
        .bind(36)
        .bind(127)
        .bind("some_kakao_id")
        .bind(chrono::Utc::now())
        .bind(chrono::Utc::now())
        .execute(&pool)
        .await?;

    let restaurant: Restaurant = sqlx::query_as("select * from restaurant where id=?")
        .bind("some_id")
        .fetch_one(&pool)
        .await?;
    println!("{:?}", restaurant);

    sqlx::query("delete from restaurant where id=?")
        .bind("some_id")
        .execute(&pool)
        .await?;

    assert!(
        sqlx::query_as::<sqlx::MySql, Restaurant>("select * from restaurant where id=?")
            .bind("some_id")
            .fetch_one(&pool)
            .await
            .is_err()
    );

    Ok(())
}
