pub mod models;

use sqlx::MySqlPool;

use crate::{error, utils::*};

use self::models::{Category, Restaurant};

pub struct DbPool {
    pool: MySqlPool,
}

impl DbPool {
    pub async fn new() -> Result<DbPool, error::Error> {
        match MySqlPool::connect(&Const::DbUrl.value()).await {
            Ok(pool) => Ok(DbPool { pool }),
            Err(e) => Err(error::Error::DbConnectionFailed(e)),
        }
    }

    pub async fn insert_all(
        &self,
        data: Vec<(Restaurant, Vec<Category>)>,
    ) -> Result<(), error::Error> {
        for d in data.into_iter() {
            let rid = d.0.id.clone();
            Self::insert_restaurant(d.0, &self.pool).await?;
            for c in d.1.into_iter() {
                Self::insert_category(c, &rid, &self.pool).await?;
            }
        }
        Ok(())
    }

    async fn insert_restaurant(r: Restaurant, pool: &MySqlPool) -> Result<(), error::Error> {
        let sql = "insert into restaurant (id, name, address, x, y, kakao_place_id, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(r.id)
            .bind(r.name)
            .bind(r.address)
            .bind(r.x)
            .bind(r.y)
            .bind(r.kakao_place_id)
            .bind(r.created_at)
            .bind(r.updated_at)
            .execute(pool)
            .await;
        match result {
            Ok(..) => Ok(()),
            Err(e) => Err(error::Error::SqlExecutionFailed(e)),
        }
    }

    async fn insert_category(c: Category, rid: &str, pool: &MySqlPool) -> Result<(), error::Error> {
        let result = sqlx::query(
            "insert into restaurant_categories (restaurant_id, categories) values (?, ?)",
        )
        .bind(rid)
        .bind(c.categories)
        .execute(pool)
        .await;
        match result {
            Ok(..) => Ok(()),
            Err(e) => Err(error::Error::SqlExecutionFailed(e)),
        }
    }
}

#[tokio::main]
#[test]
pub async fn test_connection() -> Result<(), sqlx::Error> {
    use dotenv;
    use models::*;
    dotenv::dotenv().ok();

    let url = Const::DbUrl.value();

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

// #[tokio::test]
// pub async fn delete_all() -> Result<(), sqlx::Error> {
//     use dotenv;
//     dotenv::dotenv().ok();

//     let url = Const::DB_URL.value();

//     let pool = MySqlPool::connect(&url).await?;
//     sqlx::query("delete from restaurant where 1=1")
//         .execute(&pool)
//         .await?;

//     sqlx::query("delete from restaurant_categories where 1=1")
//         .execute(&pool)
//         .await?;
//     Ok(())
// }

#[tokio::test]
pub async fn select_all() -> Result<(), sqlx::Error> {
    use dotenv;
    dotenv::dotenv().ok();

    let url = Const::DbUrl.value();

    let pool = MySqlPool::connect(&url).await?;

    let restaurants: Vec<Restaurant> = sqlx::query_as("select * from restaurant")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", restaurants.iter().take(100).collect::<Vec<_>>());

    let categories: Vec<Category> = sqlx::query_as("select * from restaurant_categories")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", categories.iter().take(100).collect::<Vec<_>>());

    println!(
        "restaurant: {}, category: {}",
        restaurants.len(),
        categories.len()
    );
    Ok(())
}
