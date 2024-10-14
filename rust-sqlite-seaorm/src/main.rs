use sea_orm::{prelude::*, ConnectionTrait, Statement};
use sea_orm::{Database, EntityTrait, Set};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Fetch the database URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let db = Database::connect(&database_url).await.unwrap();

    // Run the migration
    run_migration(&db).await;

    // Create a new user
    let new_user = user::ActiveModel {
        name: Set("John Doe".to_string()),
        ..Default::default()
    };

    // Insert the new user into the database
    let result = user::Entity::insert(new_user).exec(&db).await.unwrap();
    println!("Inserted user with ID: {}", result.last_insert_id);

    let read_result = user::Entity::find_by_id(1).all(&db).await.unwrap();
    println!("found data: {}", read_result.first().unwrap().name);
}

// Migration function
async fn run_migration(db: &DatabaseConnection) {
    let stmt = "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
    ";

    db.execute(Statement::from_string(db.get_database_backend(), stmt.to_owned()))
        .await
        .unwrap();
}

// Define the User entity
mod user {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
