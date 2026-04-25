
// Static files for App
use rocket::fs::{relative, FileServer};

// Webserver start
use rocket::{ launch, State, get };

// Postgres database connection
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::{ query, query_as, FromRow};

// Read SQL Files
use std::fs;

// Serialize json
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use uuid::Uuid;

// Run a SQL file as a query with no result
async fn query_file(pool: &PgPool, path: &str) -> Result<(), sqlx::Error> {
    let sql = fs::read_to_string(path)?;
    let rows = query(&sql)
        .fetch_all(pool)
        .await?;
    Ok(())
}

// Initilize the databas connnection
async fn init_database() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect("postgres://user:thePassword@database/mat").await.expect("connection failed");

    // Drop all Data -> Only during development
    query_file(&pool, "sql/drop-everything.sql");

    // Create all Schemas
    query_file(&pool, "sql/create-auth.sql");
    query_file(&pool, "sql/create-inventory.sql");
    query_file(&pool, "sql/create-transactions.sql");

    return pool
}

#[derive(Debug, Serialize, FromRow)]
//#[serde(crate = "rocket::serde")]
struct Article {
    pk: Uuid,
    name: String,
    description: String,
}

#[get("/article-list")]
async fn article_list(state: State<AppState>) -> Json<Vec<Article>> {
    let rows = query_as::<_, Article>("SELECT * FROM inventory.articles")
        .fetch_all(&state.db)
        .await
        .expect("could not fetch articles!");
    
    Json(rows)
}

struct AppState {
    db: PgPool
}

#[launch]
async fn rocket() -> _ {

    let state = AppState {
        db: init_database().await
    };

    rocket::build()
        .mount("/", FileServer::from(relative!("html")))
        .manage(state)
}
