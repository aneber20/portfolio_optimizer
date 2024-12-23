mod data_sourcing;
mod analytics;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{ops::{Deref, DerefMut}, sync::Mutex};


struct AppState {
    strings: Mutex<Vec<String>>,
    amounts_held: Mutex<Vec<f64>>
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
async fn add_ticker(data: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let mut strings = app_state.strings.lock().unwrap();

    // Check if ticker can be found 
    if data_sourcing::validate_ticker(&data).await {
        strings.push(data.into_inner());
        HttpResponse::Created().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}

// Delete string from list
async fn remove_ticker(data: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let mut strings = app_state.strings.lock().unwrap();
    let target = data.into_inner();
    if let Some(pos) = strings.iter().position(|x| *x == target) {
        strings.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Print list
async fn get_tickers(app_state: web::Data<AppState>) -> impl Responder {
    let strings = app_state.strings.lock().unwrap();
    HttpResponse::Ok().body(strings.join(", "))
}
/* 
// Gather stock info
async fn pull_data(app_state: web::Data<AppState>) -> impl Responder {
    let strings = app_state.strings.lock().unwrap();
}
*/  

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the stock analysis API!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_crud_operations() {
        // Set up test app with state
        let app_state = web::Data::new(AppState {
            strings: Mutex::new(Vec::new()),
            amounts_held: Mutex::new(Vec::new())
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/add/{value}", web::get().to(add_ticker))
                .route("/strings", web::get().to(get_tickers)) 
                .route("/remove/{value}", web::get().to(remove_ticker))
        ).await;

        // Test adding valid ticker
        let req = test::TestRequest::get().uri("/add/AAPL").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201); // Created

        // Test getting tickers
        let req = test::TestRequest::get().uri("/strings").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "AAPL");

        // Test removing ticker
        let req = test::TestRequest::get().uri("/remove/AAPL").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // Verify ticker was removed
        let req = test::TestRequest::get().uri("/strings").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "");

        // Test adding invalid ticker
        let req = test::TestRequest::get().uri("/add/INVALID").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400); // Bad Request

        // Test removing non-existent ticker
        let req = test::TestRequest::get().uri("/remove/NONEXISTENT").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404); // Not Found
    }

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new().service(hello)
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Welcome to the stock analysis API!");
    }
}


#[actix_web::main]
// #[tokio::main]
async fn main()  -> std::io::Result<()>  {
    let app_state = web::Data::new(AppState { 
        strings: Mutex::new(Vec::new()),
        amounts_held: Mutex::new(Vec::new())
    });

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
            .service(hello)
            .route("/add/{value}", web::get().to(add_ticker))
            .route("/strings", web::get().to(get_tickers))
            .route("/remove/{value}", web::get().to(remove_ticker))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    

    // Add some test stock symbols
}