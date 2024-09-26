mod data_sourcing;
mod analytics;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;


struct AppState {
    strings: Mutex<Vec<String>>
}

// Structs for ticker requests and responses
#[derive(Deserialize)]
struct TickerRequest {
    tickers: Vec<String>
}

#[derive(Serialize)]
struct StringResponse {
    ticker: String,
}

// Add a string to the list
async fn add_string(data: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let mut strings = app_state.strings.lock().unwrap();
    strings.push(data.into_inner());
    HttpResponse::Created().finish()
}

// Print list
async fn get_strings(app_state: web::Data<AppState>) -> impl Responder {
    let strings = app_state.strings.lock().unwrap();
    HttpResponse::Ok().body(strings.join(", "))
}

// Delete string from list
async fn remove_string(data: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let mut strings = app_state.strings.lock().unwrap();
    let target = data.into_inner();
    if let Some(pos) = strings.iter().position(|x| *x == target) {
        strings.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the stock analysis API!")
}

#[actix_web::main]
// #[tokio::main]
async fn main()  -> std::io::Result<()>  {
    let app_state = web::Data::new(AppState { 
        strings: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
            .service(hello)
            .route("/add/{value}", web::get().to(add_string))
            .route("/strings", web::get().to(get_strings))
            .route("/remove/{value}", web::get().to(remove_string))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}