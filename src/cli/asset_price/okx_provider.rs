use reqwest::{Client, Url};
use snafu::ResultExt;

use crate::cli::asset_price::{
    error,
    error::Error,
    provider::{Provider, Quote},
};

/// OKX public market-data endpoint. Unlike Binance's public API, it is not
/// geo-restricted in this environment and requires no API key.
const OKX_ENDPOINT: &str = "https://www.okx.com/api/v5/market/ticker";

/// Quote currency each cryptocurrency is priced against on OKX.
const OKX_QUOTE_CURRENCY: &str = "USDT";

/// A market-data provider backed by OKX.
pub struct OkxProvider {
    client: Client,
}

impl OkxProvider {
    pub fn new() -> Self { Self::default() }
}

impl Default for OkxProvider {
    fn default() -> Self { Self { client: Client::new() } }
}

impl Provider for OkxProvider {
    fn client(&self) -> &Client { &self.client }

    fn request_url(asset: &str) -> Result<Url, Error> {
        let instrument = format!("{}-{}", asset.to_uppercase(), OKX_QUOTE_CURRENCY);
        let mut url = Url::parse(OKX_ENDPOINT)
            .context(error::ParseUrlSnafu { url: OKX_ENDPOINT.to_string() })?;
        let _ = url.query_pairs_mut().append_pair("instId", &instrument);
        Ok(url)
    }

    fn parse(payload: &serde_json::Value) -> Option<Quote> {
        let ticker = payload["data"].as_array()?.first()?;
        Some(Quote {
            price: ticker["last"].as_str()?.parse::<f64>().ok()?,
            currency: OKX_QUOTE_CURRENCY.to_string(),
        })
    }
}
