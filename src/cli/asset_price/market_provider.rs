use reqwest::{Client, Url};
use snafu::ResultExt;

use super::{
    error::{self, Error},
    provider::{Provider, ProviderKind, Quote},
};

/// Yahoo Finance public chart endpoint. It is free of charge, requires no API
/// key, and covers global equities, ETFs, and cryptocurrencies in one place.
const YAHOO_ENDPOINT: &str = "https://query1.finance.yahoo.com/v8/finance/chart";

/// OKX public market-data endpoint. Unlike Binance's public API, it is not
/// geo-restricted in this environment and requires no API key.
const OKX_ENDPOINT: &str = "https://www.okx.com/api/v5/market/ticker";

/// Quote currency each cryptocurrency is priced against on OKX.
const OKX_QUOTE_CURRENCY: &str = "USDT";

/// Browser-like `User-Agent`; Yahoo Finance rejects requests that omit one with
/// HTTP 429.
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 \
                          (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36";

/// A concrete provider that owns an HTTP client.
pub struct MarketProvider {
    kind: ProviderKind,
    client: Client,
}

impl MarketProvider {
    pub fn new(kind: ProviderKind) -> Self {
        let client =
            Client::builder().user_agent(USER_AGENT).build().expect("Building HTTP client");
        Self { kind, client }
    }
}

impl Provider for MarketProvider {
    fn client(&self) -> &Client { &self.client }

    fn resolve(&self, asset: &str) -> String {
        match self.kind {
            ProviderKind::Yahoo => resolve_yahoo_symbol(asset),
            ProviderKind::Okx => format!("{}-{}", asset.to_uppercase(), OKX_QUOTE_CURRENCY),
        }
    }

    fn request_url(&self, symbol: &str) -> Result<Url, Error> {
        match self.kind {
            ProviderKind::Yahoo => {
                let raw = format!("{YAHOO_ENDPOINT}/{symbol}");
                let mut url = Url::parse(&raw).context(error::ParseUrlSnafu { url: raw })?;
                let _ =
                    url.query_pairs_mut().append_pair("range", "1d").append_pair("interval", "1d");
                Ok(url)
            }
            ProviderKind::Okx => {
                let mut url = Url::parse(OKX_ENDPOINT)
                    .context(error::ParseUrlSnafu { url: OKX_ENDPOINT.to_string() })?;
                let _ = url.query_pairs_mut().append_pair("instId", symbol);
                Ok(url)
            }
        }
    }

    fn parse(&self, payload: &serde_json::Value) -> Option<Quote> {
        match self.kind {
            ProviderKind::Yahoo => {
                let meta = &payload["chart"]["result"].as_array()?.first()?["meta"];
                Some(Quote {
                    price: meta["regularMarketPrice"].as_f64()?,
                    currency: meta["currency"].as_str()?.to_string(),
                })
            }
            ProviderKind::Okx => {
                let ticker = payload["data"].as_array()?.first()?;
                Some(Quote {
                    price: ticker["last"].as_str()?.parse::<f64>().ok()?,
                    currency: OKX_QUOTE_CURRENCY.to_string(),
                })
            }
        }
    }
}

/// Map friendly asset names to Yahoo Finance symbols.
///
/// Unknown names are passed through unchanged so that raw Yahoo symbols (e.g.
/// `AAPL`, `CSPX.L`) keep working.
fn resolve_yahoo_symbol(asset: &str) -> String {
    match asset.to_uppercase().as_str() {
        "CSPX" => "CSPX.L".to_string(),
        "VWRA" => "VWRA.L".to_string(),
        "VWRP" => "VWRP.L".to_string(),
        "1321" | "1321.JP" => "1321.T".to_string(),
        "0050" => "0050.TW".to_string(),
        "2330" => "2330.TW".to_string(),
        "BTC" => "BTC-USD".to_string(),
        "ETH" => "ETH-USD".to_string(),
        "SOL" => "SOL-USD".to_string(),
        _ => asset.to_string(),
    }
}
