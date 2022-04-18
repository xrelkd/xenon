mod error;

use clap::Args;
use reqwest::{Client, Url};
use snafu::ResultExt;

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command;

impl Command {
    #[allow(clippy::unused_self)]
    pub fn run(self) -> Result<(), Error> {
        tokio::runtime::Runtime::new().expect("Initializing tokio runtime").block_on(async {
            let client = Client::new();
            let url = Url::parse("http://ifconfig.me/ip").expect("a valid URL");
            let resp = client
                .get(url)
                .send()
                .await
                .context(error::GetResponseSnafu)?
                .text()
                .await
                .context(error::ParseTextSnafu)?;

            println!("{resp}");

            Ok(())
        })
    }
}
