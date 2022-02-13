use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error occurs while getting HTTP response, error: {}", source))]
    GetResponse { source: hyper::Error },

    #[snafu(display("Error occurs while concatenating buffers from a body, error: {}", source))]
    ConcatenateBuffers { source: hyper::Error },
}
