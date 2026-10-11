mod error;
mod okx_provider;
mod provider;
mod yahoo_provider;

use clap::{Args, ValueEnum};

use self::{okx_provider::OkxProvider, provider::Provider, yahoo_provider::YahooProvider};

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command {
    #[arg(
        name = "assets",
        value_delimiter = ',',
        help = "Asset symbol list, separated with ',' (e.g. \"CSPX,BTC,1321.jp,0050,2330\")"
    )]
    assets: Vec<String>,

    #[arg(
        name = "provider",
        long = "provider",
        short = 'p',
        default_value = "yahoo",
        value_enum,
        help = "Market-data provider used to look up prices"
    )]
    provider: ProviderKind,

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
        if self.assets.is_empty() {
            return Err(Error::NoAssetProvided);
        }

        match self.provider {
            ProviderKind::Yahoo => self.run_provider(&YahooProvider::new()),
            ProviderKind::Okx => self.run_provider(&OkxProvider::new()),
        }
    }

    fn run_provider<P: Provider>(self, provider: &P) -> Result<(), Error> {
        let ret =
            tokio::runtime::Runtime::new().expect("Initializing tokio runtime").block_on(async {
                let mut prices = Vec::with_capacity(self.assets.len());
                for asset in &self.assets {
                    prices.push(match provider.fetch(asset).await {
                        Ok(Some(quote)) => {
                            format!("{asset}: {}{}", currency_symbol(&quote.currency), quote.price)
                        }
                        Ok(None) | Err(_) => format!("{asset}: n/a"),
                    });
                }

                println!("{}", prices.join(" "));

                Ok(())
            });

        if ret.is_err() && self.show_fallback_message {
            let fallback_message = self
                .assets
                .iter()
                .map(|asset| format!("{asset}: n/a"))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{fallback_message}");
        }

        ret
    }
}

/// Supported market-data providers (clap-friendly).
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProviderKind {
    /// Yahoo Finance: global equities, ETFs, and cryptocurrencies.
    Yahoo,
    /// OKX: cryptocurrencies only.
    Okx,
}

/// Map ISO currency codes to their symbol for compact, single-token display.
///
/// Covers the markets commonly traded by investors in Japan, Korea, Taiwan,
/// Hong Kong, Singapore, China, the UK, the EU, and the US.
///
/// Unknown codes are returned unchanged so the currency stays identifiable.
fn currency_symbol(currency: &str) -> &str {
    match currency {
        "EUR" => "€",
        "GBP" => "£",
        "HKD" => "HK$",
        "JPY" | "CNY" => "¥",
        "KRW" => "₩",
        "SGD" => "S$",
        "TWD" => "NT$",
        "USD" | "USDT" => "$",
        other => other,
    }
}
