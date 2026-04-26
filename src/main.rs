
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

    /*
    // Drop all Data -> Only during development
    query_file(&pool, "sql/drop-everything.sql").await;

    // Create all Schemas
    query_file(&pool, "sql/create-auth.sql").await;
    query_file(&pool, "sql/create-inventory.sql").await;
    query_file(&pool, "sql/create-transactions.sql").await;
    */

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

#[get("/get-article/<id>")]
async fn get_article(state: &State<AppState>, id: String) -> Json<Article> {
    let row = query_as::<_, Article>("SELECT pk, name, description FROM inventory.articles WHERE pk = CAST($1 AS uuid)")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .expect("Could not fetch the requested Article!");

    Json(row)
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

#[derive(Debug, Serialize, FromRow)]
struct Position {
    pk: Uuid,
    fk_article: Uuid,
    is_consumable: bool,
    is_unique: bool,
    amount: Option<i32>,
    normal_amount: Option<i32>,
    unique_name: Option<String>,
}

#[get("/list-position/<article_id>")]
async fn list_position(state: &State<AppState>, article_id: String) -> Json<Vec<Position>> {

    let row = query_as::<_, Position>("SELECT * FROM inventory.positions WHERE fk_article = CAST($1 AS uuid)")
        .bind(article_id)
        .fetch_all(&state.db)
        .await
        .expect("Could not fetch the requested Article!");

    Json(row)
    
}

#[derive(Deserialize, Debug)]
struct NewPosition {
    fk_article: Uuid,
    is_consumable: bool,
    is_unique: bool,
    amount: i32,
    normal_amount: Option<i32>,
    unique_name: Option<String>,
}

#[post("/create-position", data = "<position>")]
async fn create_position(state: &State<AppState>, position: Json<NewPosition>) -> String {

    query("INSERT INTO inventory.positions (fk_article, is_consumable, is_unique, amount, normal_amount, unique_name) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&position.fk_article)
        .bind(&position.is_consumable)
        .bind(&position.is_unique)
        .bind(&position.amount)
        .bind(&position.normal_amount)
        .bind(&position.unique_name)
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
        .mount("/api", routes![list_article, get_article, create_article, list_position, create_position])
        .manage(state)
}
