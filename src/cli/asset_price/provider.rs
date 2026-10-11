use reqwest::{Client, Url};
use snafu::ResultExt;

use crate::cli::asset_price::{error, error::Error};

/// A resolved price together with the currency it is denominated in.
pub struct Quote {
    pub price: f64,
    pub currency: String,
}

/// A market-data provider that resolves symbols, builds request URLs, and
/// extracts prices from responses.
///
/// Stateless operations are associated functions so that implementations stay
/// free of an enum selector; only the shared HTTP client is stateful.
pub trait Provider {
    /// HTTP client used for requests.
    fn client(&self) -> &Client;

    /// Build the full request URL for an asset symbol.
    fn request_url(asset: &str) -> Result<Url, Error>;

    /// Extract a quote from a parsed JSON response, if the response carries
    /// one.
    fn parse(payload: &serde_json::Value) -> Option<Quote>;

    /// Fetch the quote for an asset.
    ///
    /// Returns `Ok(None)` when the provider has no data for the symbol.
    async fn fetch(&self, asset: &str) -> Result<Option<Quote>, Error> {
        let url = Self::request_url(asset)?;
        let payload: serde_json::Value = self
            .client()
            .get(url)
            .send()
            .await
            .context(error::GetResponseSnafu)?
            .json()
            .await
            .context(error::ParseResponseSnafu)?;
        Ok(Self::parse(&payload))
    }
}
