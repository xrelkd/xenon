use reqwest::{Client, Url};
use snafu::ResultExt;

use crate::cli::asset_price::{
    error,
    error::Error,
    provider::{Provider, Quote},
};

/// Yahoo Finance public chart endpoint. It is free of charge, requires no API
/// key, and covers global equities, ETFs, and cryptocurrencies in one place.
const YAHOO_ENDPOINT: &str = "https://query1.finance.yahoo.com/v8/finance/chart";

/// Browser-like `User-Agent`; Yahoo Finance rejects requests that omit one with
/// HTTP 429.
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                          (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36";

/// A market-data provider backed by Yahoo Finance.
pub struct YahooProvider {
    client: Client,
}

impl YahooProvider {
    pub fn new() -> Self { Self::default() }
}

impl Default for YahooProvider {
    fn default() -> Self {
        let client =
            Client::builder().user_agent(USER_AGENT).build().expect("Building HTTP client");
        Self { client }
    }
}

impl Provider for YahooProvider {
    fn client(&self) -> &Client { &self.client }

    fn request_url(asset: &str) -> Result<Url, Error> {
        let symbol = resolve_yahoo_symbol(asset);
        let raw = format!("{YAHOO_ENDPOINT}/{symbol}");
        let mut url = Url::parse(&raw).context(error::ParseUrlSnafu { url: raw })?;
        let _ = url.query_pairs_mut().append_pair("range", "1d").append_pair("interval", "1d");
        Ok(url)
    }

    fn parse(payload: &serde_json::Value) -> Option<Quote> {
        let meta = &payload["chart"]["result"].as_array()?.first()?["meta"];
        Some(Quote {
            price: meta["regularMarketPrice"].as_f64()?,
            currency: meta["currency"].as_str()?.to_string(),
        })
    }
}

/// Map friendly asset names to Yahoo Finance symbols.
///
/// Covers the most popular investment targets in the US, the UK, the EU,
/// Taiwan, Japan, Korea, and the cryptocurrency market.
fn resolve_yahoo_symbol(asset: &str) -> String {
    match asset.to_uppercase().as_str() {
        // Taiwan
        "0050" => "0050.TW".to_string(),
        "0056" => "0056.TW".to_string(),
        "2317" => "2317.TW".to_string(),
        "2330" => "2330.TW".to_string(),
        "2412" => "2412.TW".to_string(),
        // Japan
        "1321" | "1321.JP" => "1321.T".to_string(),
        "1322" => "1322.T".to_string(),
        "7203" => "7203.T".to_string(),
        "6758" => "6758.T".to_string(),
        "7974" => "7974.T".to_string(),
        "8035" => "8035.T".to_string(),
        "9983" => "9983.T".to_string(),
        "9984" => "9984.T".to_string(),
        // UK
        "CSPX" => "CSPX.L".to_string(),
        "EQQQ" => "EQQQ.L".to_string(),
        "VUAG" => "VUAG.L".to_string(),
        "VUSD" => "VUSD.L".to_string(),
        "VWRA" => "VWRA.L".to_string(),
        "VWRP" => "VWRP.L".to_string(),
        // EU
        "EUNL" => "EUNL.DE".to_string(),
        "IWDA" => "IWDA.AS".to_string(),
        "IUSQ" => "IUSQ.DE".to_string(),
        "SXR8" => "SXR8.DE".to_string(),
        "VUAA" => "VUAA.DE".to_string(),
        // Korea
        "SAMSUNG" => "005930.KS".to_string(),
        "SK HYNIX" | "SKHYNIX" => "000660.KS".to_string(),
        // Cryptocurrency
        "BTC" => "BTC-USD".to_string(),
        "ETH" => "ETH-USD".to_string(),
        "SOL" => "SOL-USD".to_string(),
        "ADA" => "ADA-USD".to_string(),
        "BNB" => "BNB-USD".to_string(),
        "DOGE" => "DOGE-USD".to_string(),
        "LINK" => "LINK-USD".to_string(),
        "XRP" => "XRP-USD".to_string(),
        // US
        "AAPL" => "AAPL".to_string(),
        "AMZN" => "AMZN".to_string(),
        "GOOGL" => "GOOGL".to_string(),
        "META" => "META".to_string(),
        "MSFT" => "MSFT".to_string(),
        "TSLA" => "TSLA".to_string(),
        "NVDA" => "NVDA".to_string(),
        "DRAM" => "DRAM".to_string(),
        "BOXX" => "BOXX".to_string(),
        "QQQ" => "QQQ".to_string(),
        "QQQM" => "QQQM".to_string(),
        "SPY" => "SPY".to_string(),
        "VOO" => "VOO".to_string(),
        _ => asset.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_yahoo_symbol;

    #[test]
    fn test_resolve_yahoo_symbol() {
        assert_eq!(resolve_yahoo_symbol("0056"), "0056.TW");
        assert_eq!(resolve_yahoo_symbol("2330"), "2330.TW");
        assert_eq!(resolve_yahoo_symbol("1321.jp"), "1321.T");
        assert_eq!(resolve_yahoo_symbol("IWDA"), "IWDA.AS");
        assert_eq!(resolve_yahoo_symbol("VUAA"), "VUAA.DE");
        assert_eq!(resolve_yahoo_symbol("samsung"), "005930.KS");
        assert_eq!(resolve_yahoo_symbol("sk hynix"), "000660.KS");
        assert_eq!(resolve_yahoo_symbol("DRAM"), "DRAM");
        assert_eq!(resolve_yahoo_symbol("BTC"), "BTC-USD");
        assert_eq!(resolve_yahoo_symbol("unexpected-symbol"), "unexpected-symbol");
    }
}
