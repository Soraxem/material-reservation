
// Static files for App
use rocket::fs::{relative, FileServer};

// Webserver start
use rocket::launch;


#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", FileServer::from(relative!("html")))
}
