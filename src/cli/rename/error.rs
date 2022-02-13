use std::path::PathBuf;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("File `{}` not exists", file_name.display()))]
    FileNotExist { file_name: PathBuf, source: std::io::Error },

    #[snafu(display("User tries to rename file `{}` to an empty file name", file_name.display()))]
    RenameToEmptyFileName { file_name: PathBuf },

    #[snafu(display("Error occurs while trying to rename `{}` to `{new_file_name}`", old_file_name.display()))]
    RenameFile { old_file_name: PathBuf, new_file_name: String, source: std::io::Error },

    #[snafu(display("Error occurs while reading next line, error: {source}"))]
    Rustyline { source: rustyline::error::ReadlineError },
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(source: rustyline::error::ReadlineError) -> Self { Self::Rustyline { source } }
}
