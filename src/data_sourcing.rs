/*
Module for handling data requests and unpacking from yahoo finance

To Do:

*/

use std::error::Error;

use yahoo_finance::{history, Interval, Bar};
use tokio::runtime::Runtime;
use chrono::{Utc, Duration};
use time::OffsetDateTime;



// Check if stock exists
pub async fn validate_ticker(ticker: &str) -> bool {
    match history::retrieve_interval(ticker, Interval::_1m).await {
        Ok(_) => true,
        Err(_) => false
    }
}

// Pulls longer term data from yahoo
pub async fn pull_ticker_data_lt(tickers: &[String]) -> Result<Vec<(String, Vec<Bar>)>, Box<dyn Error>> {
    let mut all_data = Vec::new();

    for ticker in tickers {
        match history::retrieve_interval(&ticker, Interval::_5y).await {
            Ok(data) => all_data.push((ticker.clone(), data)),
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

#[tokio::test]
async fn test_data_sourcing() {

    // Test basic Yahoo Finance API functionality
    use yahoo_finance_api::YahooConnector;
    let provider = YahooConnector::new().expect("Failed to create YahooConnector");
    
    // Test getting quote data
    let response = provider.get_latest_quotes("AAPL", "1d").await;
    assert!(response.is_ok());
    let quotes = response.unwrap();
    assert!(!quotes.quotes().unwrap().is_empty());
    
    // Test getting historical data 
    let start = chrono::Utc::now() - chrono::Duration::days(10);
    let end = chrono::Utc::now();
    let response = provider.get_quote_history(
        "AAPL",
        time::OffsetDateTime::from_unix_timestamp(start.timestamp()).unwrap(),
        time::OffsetDateTime::from_unix_timestamp(end.timestamp()).unwrap()
    ).await;
    assert!(response.is_ok());
    let history = response.unwrap();
    assert!(!history.quotes().unwrap().is_empty());

    // Verify quote data contains valid prices
    let quotes = quotes.quotes().unwrap();
    let quote = &quotes[0];
    assert!(quote.close > 0.0);
    assert!(quote.volume > 0);
    
    // Verify historical data contains valid prices
    let hist_quotes = history.quotes().unwrap();
    let hist_quote = &hist_quotes[0]; 
    assert!(hist_quote.close > 0.0);
    assert!(hist_quote.volume > 0);
    
    // Test validate_ticker
    assert!(validate_ticker("AAPL").await); // Valid ticker
    assert!(!validate_ticker("NOTREAL").await); // Invalid ticker

    // Test short term data pulling
    let aapl = String::from("AAPL");
    let msft = String::from("MSFT");
    let tickers: &[String] = &[aapl.clone(), msft.clone()];
    
    let st_data = pull_ticker_data_st(tickers).await;
    assert!(st_data.is_ok());
    
    let st_data = st_data.unwrap();
    assert_eq!(st_data.len(), 2); // Should have data for both tickers

    for (name, data) in &st_data {
        assert!(!data.is_empty()); // Data vector shouldn't be empty
        let recent_day = data.first().unwrap();
        println!("{} ST - close was {} at {}", name, recent_day.close, recent_day.timestamp);
        assert!(recent_day.close > 0.0); // Price should be positive
    }

    // Test long term data pulling
    let lt_data = pull_ticker_data_lt(tickers).await;
    assert!(lt_data.is_ok());
    
    let lt_data = lt_data.unwrap();
    assert_eq!(lt_data.len(), 2);

    for (name, data) in &lt_data {
        assert!(!data.is_empty());
        let recent_day = data.first().unwrap();
        println!("{} LT - close was {} at {}", name, recent_day.close, recent_day.timestamp);
        assert!(recent_day.close > 0.0);
    }

    // Test unpacking bars
    for (name, data) in &st_data {
        let closes = unpack_bars_close(name, data.clone());
        assert!(closes.is_ok());
        let closes = closes.unwrap();
        assert_eq!(closes.len(), data.len()); // Should have same number of data points
        assert!(closes.iter().all(|&x| x > 0.0)); // All prices should be positive
    }

    // Test unpacking empty bars
    let empty_bars = Vec::new();
    assert!(unpack_bars_close("TEST", empty_bars).is_err());
}