mod error;

use clap::Args;
use reqwest::{Client, Url};
use snafu::ResultExt;

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command {
    #[clap(
        name = "cryptocurrencies",
        value_delimiter = ',',
        help = "Cryptocurrency symbol list, separated with ','"
    )]
    cryptocurrencies: Vec<String>,

    #[clap(
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
                    let url = format!(
                        "https://min-api.cryptocompare.com/data/pricemulti?fsyms={fsyms}&tsyms=USD",
                        fsyms = self.cryptocurrencies.join(",")
                    );
                    Url::parse(&url).context(error::ParseUrlSnafu { url })?
                };

                let resp = client.get(url).send().await.context(error::GetResponseSnafu)?;

                let payload: serde_json::Value =
                    resp.json().await.context(error::ConcatenateBuffersSnafu)?;

                if let Some(prices) = payload.as_object() {
                    let result = self
                        .cryptocurrencies
                        .iter()
                        .map(|currency| {
                            let price =
                                prices[currency]["USD"].as_f64().expect("price must be a float");
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
