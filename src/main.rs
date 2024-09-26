mod data_sourcing;
mod analytics;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Structs for ticker requests and responses
struct TickerRequest {
    tickers: Vec<String>
}

struct TickerResponse {
    ticker: String,
    data: Vec<f64>
}

pub async fn stock_data_handler(req: web::Json<TickerRequest>) -> impl Responder {
    match data_sourcing::pull_ticker_data_st(&req.tickers).await {
        Ok(data) => {
            let response: Vec<TickerResponse> = data.into_iter()
            .map(|(ticker, data)| TickerResponse { ticker, data: data_sourcing::unpack_bars_close(ticker.as_str(), data)  })
            .collect();
        HttpResponse::Ok().json(response)
        }

    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}