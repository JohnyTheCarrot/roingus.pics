use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{get, App, Error, HttpServer};
use dotenv::dotenv;
use rand::seq::IteratorRandom;
use std::env;

#[get("/{tail:.*}")]
async fn random_roing() -> Result<fs::NamedFile, Error> {
    let mut rng = rand::thread_rng();
    let files = std::fs::read_dir("./roingus-pics").unwrap();
    let dir_entry = files.choose(&mut rng).unwrap().unwrap();

    let file = fs::NamedFile::open(dir_entry.path())?;

    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("Failed to parse port");

    let ip = env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());

    HttpServer::new(|| App::new().wrap(Logger::default()).service(random_roing))
        .bind((ip, port))?
        .run()
        .await
}
