mod error;

use clap::Args;
use reqwest::Client;
use snafu::ResultExt;
use url::Url;

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command {
    /// IP provider service to use.
    #[arg(long = "provider", short = 'p', default_value = "amazon", value_enum)]
    kind: ProviderKind,
}

impl Command {
    pub fn run(self) -> Result<(), Error> {
        tokio::runtime::Runtime::new().expect("Initializing tokio runtime").block_on(async {
            let provider = SimpleProvider::new(self.kind);
            let ip = provider.fetch().await?;
            println!("{ip}");
            Ok(())
        })
    }
}

/// A provider that can return the public IP address of the current host.
trait Provider {
    /// URL to fetch the IP from.
    fn url(&self) -> Url;

    /// HTTP client used for requests.
    fn client(&self) -> &Client;

    /// Parse the IP address from the raw response text.
    ///
    /// The default implementation trims surrounding whitespace.
    /// Override this if the provider wraps the IP in extra content.
    fn parse(&self, text: &str) -> Result<String, Error> {
        let ip = text.trim();
        if ip.is_empty() {
            return Err(Error::EmptyResponse);
        }
        Ok(ip.to_string())
    }

    /// Fetch the IP address from the remote service.
    async fn fetch(&self) -> Result<String, Error> {
        let text = self
            .client()
            .get(self.url())
            .send()
            .await
            .context(error::GetResponseSnafu)?
            .text()
            .await
            .context(error::ParseTextSnafu)?;
        self.parse(&text)
    }
}

/// Kinds of supported IP provider services (clap-friendly).
#[derive(Debug, Clone, clap::ValueEnum)]
enum ProviderKind {
    Amazon,
    Icanhazip,
    IfconfigIo,
    Ipinfo,
    IdentMe,
}

/// A concrete provider that owns an HTTP client.
struct SimpleProvider {
    kind: ProviderKind,
    client: Client,
}

impl SimpleProvider {
    fn new(kind: ProviderKind) -> Self { Self { kind, client: Client::new() } }
}

impl Provider for SimpleProvider {
    fn url(&self) -> Url {
        match self.kind {
            ProviderKind::Amazon => Url::parse("http://checkip.amazonaws.com").expect("valid URL"),
            ProviderKind::Icanhazip => Url::parse("http://icanhazip.com").expect("valid URL"),
            ProviderKind::IfconfigIo => Url::parse("http://ifconfig.io/ip").expect("valid URL"),
            ProviderKind::Ipinfo => Url::parse("http://ipinfo.io/ip").expect("valid URL"),
            ProviderKind::IdentMe => Url::parse("http://ident.me").expect("valid URL"),
        }
    }

    fn client(&self) -> &Client { &self.client }
}
