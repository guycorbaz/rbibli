//! Utility script to verify the database schema for the `titles` table.
//!
//! This script connects to the database and retrieves the column details for
//! `dewey_code` from the `INFORMATION_SCHEMA`. It prints the column name, data type,
//! and maximum character length to the console for verification purposes.

use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    println!("Connecting to database...");
    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await?;

    println!("Connected. Querying schema for 'titles' table...");

    let query = r#"
        SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH 
        FROM INFORMATION_SCHEMA.COLUMNS 
        WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'titles' AND COLUMN_NAME = 'dewey_code'
    "#;

    let row: (String, String, Option<u64>) = sqlx::query_as(query)
        .fetch_one(&pool)
        .await?;

    println!("Column: {}", row.0);
    println!("Type: {}", row.1);
    println!("Max Length: {:?}", row.2);

    Ok(())
}
