use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await?;

    let row: (String,) = sqlx::query_as(
        "SELECT COLUMN_TYPE FROM INFORMATION_SCHEMA.COLUMNS
         WHERE TABLE_NAME = 'titles'
         AND COLUMN_NAME = 'dewey_code'
         AND TABLE_SCHEMA = DATABASE()"
    )
    .fetch_one(&pool)
    .await?;

    println!("Current dewey_code type: {}", row.0);

    Ok(())
}
