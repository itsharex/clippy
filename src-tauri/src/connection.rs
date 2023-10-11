use crate::{service::window::get_data_path, types::types::Config};
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = if !cfg!(debug_assertions) {
        String::from("sqlite://../clippy.sqlite?mode=rwc")
    } else {
        get_prod_database_url()
    };

    println!("database_url: {}", database_url);

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&db, None).await.ok();

    Ok(db)
}

fn get_prod_database_url() -> String {
    let data_path = get_data_path();

    let json = std::fs::read_to_string(data_path.config_file_path).unwrap();

    let config: Config = serde_json::from_str(&json).unwrap();

    let db = format!("sqlite://{}?mode=rwc", config.db);

    db
}
