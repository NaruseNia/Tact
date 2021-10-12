mod models;
mod handlers;

use actix_web::{ web, App, HttpServer }; 
use std::io::Result;
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {

    //get env
    dotenv().ok();

    //get host and port from .env
    let server_host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".into());
    let server_port = env::var("SERVER_PORT").unwrap_or("8080".into());

    println!("Server started on {}:{}", &server_host, &server_port);

    //Router
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::status))
    })
    .bind(format!("{}:{}", &server_host, &server_port))?
    .run()
    .await
}



