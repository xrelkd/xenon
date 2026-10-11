mod error;

use clap::Args;
use reqwest::{Client, Url};
use snafu::ResultExt;

pub use self::error::Error;

/// OKX public market-data endpoint. Unlike Binance's public API, it is not
/// geo-restricted in this environment and requires no API key.
const API_ENDPOINT: &str = "https://www.okx.com/api/v5/market/ticker";

/// Quote currency each requested cryptocurrency is priced against.
const QUOTE_CURRENCY: &str = "USDT";

#[derive(Args, Debug)]
pub struct Command {
    #[arg(
        name = "cryptocurrencies",
        value_delimiter = ',',
        help = "Cryptocurrency symbol list, separated with ','"
    )]
    cryptocurrencies: Vec<String>,

    #[arg(
        name = "show-fallback-message",
        long = "show-fallback-message",
        help = "Show fallback message (ex: \"BTC: n/a, ETH: n/a\") while failed to fetch price \
                information"
    )]
    show_fallback_message: bool,
}

impl Command {
    pub fn run(self) -> Result<(), Error> {
        if self.cryptocurrencies.is_empty() {
            return Err(Error::NoCryptocurrencyProvided);
        }

        let ret =
            tokio::runtime::Runtime::new().expect("Initializing tokio runtime").block_on(async {
                let client = Client::new();

                let mut prices = Vec::with_capacity(self.cryptocurrencies.len());
                for currency in &self.cryptocurrencies {
                    let instrument = format!("{}-{}", currency.to_uppercase(), QUOTE_CURRENCY);
                    let mut url = Url::parse(API_ENDPOINT)
                        .context(error::ParseUrlSnafu { url: API_ENDPOINT.to_string() })?;
                    let _ = url.query_pairs_mut().append_pair("instId", &instrument);

                    let payload: serde_json::Value = client
                        .get(url)
                        .send()
                        .await
                        .context(error::GetResponseSnafu)?
                        .json()
                        .await
                        .context(error::ParseResponseSnafu)?;

                    let price = payload["data"]
                        .as_array()
                        .and_then(|data| data.first())
                        .and_then(|ticker| ticker["last"].as_str())
                        .and_then(|last| last.parse::<f64>().ok());

                    prices.push(price.map_or_else(
                        || format!("{currency}: n/a"),
                        |price| format!("{currency}: ${price}"),
                    ));
                }

                println!("{}", prices.join(" "));

                Ok(())
            });

        if ret.is_err() && self.show_fallback_message {
            let fallback_message = self
                .cryptocurrencies
                .iter()
                .map(|currency| format!("{currency}: n/a"))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{fallback_message}");
        }

        ret
    }
}
