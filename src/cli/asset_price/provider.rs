use reqwest::{Client, Url};
use snafu::ResultExt;

use super::error::{self, Error};

/// A resolved price together with the currency it is denominated in.
pub struct Quote {
    pub price: f64,
    pub currency: String,
}

/// Supported market-data providers (clap-friendly).
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ProviderKind {
    /// Yahoo Finance: global equities, ETFs, and cryptocurrencies.
    Yahoo,
    /// OKX: cryptocurrencies only.
    Okx,
}

/// A market-data provider that resolves symbols and extracts prices.
pub trait Provider {
    /// HTTP client used for requests.
    fn client(&self) -> &Client;

    /// Translate a user-supplied asset symbol into a provider-specific symbol.
    fn resolve(&self, asset: &str) -> String;

    /// Build the full request URL for a resolved symbol.
    fn request_url(&self, symbol: &str) -> Result<Url, Error>;

    /// Extract a quote from a parsed JSON response, if the response carries
    /// one.
    fn parse(&self, payload: &serde_json::Value) -> Option<Quote>;

    /// Fetch the quote for an asset.
    ///
    /// Returns `Ok(None)` when the provider has no data for the symbol.
    async fn fetch(&self, asset: &str) -> Result<Option<Quote>, Error> {
        let symbol = self.resolve(asset);
        let url = self.request_url(&symbol)?;
        let payload: serde_json::Value = self
            .client()
            .get(url)
            .send()
            .await
            .context(error::GetResponseSnafu)?
            .json()
            .await
            .context(error::ParseResponseSnafu)?;
        Ok(self.parse(&payload))
    }
}
