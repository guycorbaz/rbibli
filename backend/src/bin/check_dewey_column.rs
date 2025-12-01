//! Utility script to check the column type of `dewey_code` in the `titles` table.
//!
//! This script connects to the database and queries the `INFORMATION_SCHEMA` to verify
//! the data type of the `dewey_code` column. It is used to ensure that the column
//! has been correctly migrated to a `VARCHAR` type.

use sqlx::mysql::MySqlPoolOptions;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = backend::configuration::get_configuration(None).expect("Failed to read configuration.");
    let database_url = configuration.database.connection_string();

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
