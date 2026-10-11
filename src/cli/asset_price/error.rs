use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("No asset symbol is provided"))]
    NoAssetProvided,

    #[snafu(display("Error occurs while getting HTTP response, error: {source}"))]
    GetResponse { source: reqwest::Error },

    #[snafu(display("Error occurs while parsing response body, error: {source}"))]
    ParseResponse { source: reqwest::Error },

    #[snafu(display("Error occurs while parsing URI {url}, error: {source}"))]
    ParseUrl { url: String, source: url::ParseError },
}
