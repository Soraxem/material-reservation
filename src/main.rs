
// Static files for App
use rocket::fs::{relative, FileServer};

// Webserver start
use rocket::{ launch, State, get, routes, post };

// Postgres database connection
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use sqlx::{ query, query_as, FromRow};

// Read SQL Files
use std::fs;

// Serialize json
use rocket::serde::json::Json;
use rocket::serde::{ Serialize, Deserialize };

// Handeling uuids
use uuid::Uuid;

// Run a SQL file as a query with no result
async fn query_file(pool: &PgPool, path: &str) -> Result<(), sqlx::Error> {

    // Read SQL File
    let sql_file = fs::read_to_string(path).expect("file not read!");

    // Split the file into seperate statements
    let statements: Vec<&str> = sql_file.split_inclusive(";").collect();
    
    // Run each Query
    for statement in statements.iter() {
        println!("Running Query: {}", statement);
        query(&statement)
            .execute(pool)
            .await
            .expect("quering file did not work");
    }

    // Return successfully
    Ok(())
}

// Initilize the databas connnection
async fn init_database() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect("postgres://user:thePassword@database/mat").await.expect("connection failed");

    // Drop all Data -> Only during development
    query_file(&pool, "sql/drop-everything.sql").await;

    // Create all Schemas
    query_file(&pool, "sql/create-auth.sql").await;
    query_file(&pool, "sql/create-inventory.sql").await;
    query_file(&pool, "sql/create-transactions.sql").await;

    return pool
}


#[derive(Debug, Serialize, FromRow)]
struct Article {
    pk: Uuid,
    name: String,
    description: String,
}

#[get("/list-article")]
async fn list_article(state: &State<AppState>) -> Json<Vec<Article>> {
    let rows = query_as::<_, Article>("SELECT pk, name, description FROM inventory.articles")
        .fetch_all(&state.db)
        .await
        .expect("could not fetch articles!");
    
    Json(rows)
}


#[derive(Deserialize, Debug)]
struct NewArticle {
    name: String,
    description: String,
}

#[post("/create-article", data = "<article>")]
async fn create_article(state: &State<AppState>, article: Json<NewArticle>) -> String {

    query("INSERT INTO inventory.articles (name, description) VALUES ($1, $2)")
        .bind(&article.name)
        .bind(&article.description)
        .execute(&state.db)
        .await
        .expect("Failed");


    "ok".to_string()
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
        .mount("/api", routes![list_article, create_article])
        .manage(state)
}
