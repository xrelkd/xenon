use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("No cryptocurrency symbol is provided"))]
    NoCryptocurrencyProvided,

    #[snafu(display("Error occurs while getting HTTP response, error: {source}"))]
    GetResponse { source: reqwest::Error },

    #[snafu(display("Error occurs while concatenating buffers from a body, error: {source}"))]
    ConcatenateBuffers { source: reqwest::Error },

    #[snafu(display("Error occurs while parsing URI {url}, error: {source}"))]
    ParseUrl { url: String, source: url::ParseError },
}
