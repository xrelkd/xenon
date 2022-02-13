mod error;

use clap::Args;
use hyper::{Client, Uri};
use snafu::ResultExt;

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command;

impl Command {
    #[allow(clippy::unused_self)]
    pub fn run(self) -> Result<(), Error> {
        tokio::runtime::Runtime::new().expect("Initailizing tokio runtime").block_on(async {
            let client = Client::new();
            let uri = Uri::from_static("http://ifconfig.me/ip");
            let resp = client.get(uri).await.context(error::GetResponseSnafu)?;

            let body = hyper::body::to_bytes(resp.into_body())
                .await
                .context(error::ConcatenateBuffersSnafu)?;
            println!("{}", String::from_utf8_lossy(&body[..]));

            Ok(())
        })
    }
}
