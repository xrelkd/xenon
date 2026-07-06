mod error;

use clap::Args;
use reqwest::{Client, Url};
use snafu::ResultExt;

pub use self::error::Error;

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
                let url = {
                    let symbols: Vec<String> =
                        self.cryptocurrencies.iter().map(|c| format!("\"{c}USDT\"")).collect();
                    let symbols_json = format!("[{}]", symbols.join(","));
                    let mut url = Url::parse("https://api.binance.com/api/v3/ticker/price")
                        .context(error::ParseUrlSnafu {
                            url: "https://api.binance.com/api/v3/ticker/price".to_string(),
                        })?;
                    let _ = url.query_pairs_mut().append_pair("symbols", &symbols_json);
                    url
                };

                let resp = client.get(url).send().await.context(error::GetResponseSnafu)?;

                let payload: serde_json::Value =
                    resp.json().await.context(error::ConcatenateBuffersSnafu)?;

                if let Some(prices) = payload.as_array() {
                    let result = self
                        .cryptocurrencies
                        .iter()
                        .map(|currency| {
                            let symbol = format!("{currency}USDT");
                            let price = prices
                                .iter()
                                .find(|item| item["symbol"].as_str() == Some(&symbol))
                                .and_then(|item| item["price"].as_str())
                                .and_then(|p| p.parse::<f64>().ok())
                                .expect("price must be a float");
                            format!("{currency}: ${price}")
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    println!("{result}");
                }

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
