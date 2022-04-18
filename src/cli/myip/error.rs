use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error occurs while getting HTTP response, error: {source}"))]
    GetResponse { source: reqwest::Error },

    #[snafu(display("Error occurs while parsing text from HTTP response, error: {source}"))]
    ParseText { source: reqwest::Error },
}
