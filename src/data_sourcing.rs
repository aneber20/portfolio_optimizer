
/*
Module for handling data requests and unpacking from yahoo finance

To Do:

*/

use std::error::Error;

use yahoo_finance::{history, Interval, Bar};

// Pulls longer term data from yahoo
pub async fn pull_ticker_data_lt(tickers: &[String]) -> Result<Vec<(String, Vec<Bar>)>, Box<dyn Error>> {
    let mut all_data = Vec::new();

    for ticker in tickers {
        match history::retrieve_interval(&ticker, Interval::_5y).await {
            Ok(data) => match all_data.push((ticker.clone(), data.)),
            Err(_) => print!("Failed to retreive ticker: {}", ticker)
        }
    }
    Ok(all_data)
}

// Pulls shorter term data from yahoo
pub async fn pull_ticker_data_st(tickers: &[String]) -> Result<Vec<(String, Vec<Bar>)>, Box<dyn Error>> {
    let mut all_data = Vec::new();

    for ticker in tickers {
        match history::retrieve_interval(&ticker, Interval::_1mo).await {
            Ok(data) => all_data.push((ticker.clone(), data)),
            Err(_) => print!("Failed to retreive ticker: {}", ticker)
        }
    }
    Ok(all_data)    
}

// Unpack bars into the close for each day - for volatility analysis
pub fn unpack_bars_close(ticker: &str, data: Vec<Bar>) -> Result<Vec<f64>, String> {
    let mut float_vec = Vec::new();

    // Check vector isn't empty
    if data.is_empty() {
        Err(String::from(format!("Ticker {} has no data", ticker)))

    } else {
        for bar in data {
            float_vec.push(bar.close);
        }
        Ok(float_vec)

    }
    
}

fn main() -> () {
    let aapl = String::from("AAPL");
    let msft = String::from("MSFT");
    let mut tickers: &[String] = &[aapl, msft];
    let mut st_data = pull_ticker_data_lt(tickers);
}

/* 
#[test]
fn test() {

    // Pull test data
    let aapl = stringify!("AAPL").to_string();
    let msft = stringify!("MSFT").to_string();
    let mut tickers: &[String] = &[aapl, msft];
    let mut st_data = pull_ticker_data_st(tickers).await;

    // Make sure it works
    assert!(st_data.is_ok());
    for (name, data) in st_data.unwrap() {
        let recent_day = match data.first() {
            Some(bar) => bar,
        None => continue,
        };
        println!("{} - close was {} at {}", name, recent_day.close, recent_day.timestamp);
    }
        

}
    */