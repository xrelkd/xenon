mod error;

use std::path::PathBuf;

use clap::{Args, ValueHint};
use snafu::{IntoError, ResultExt};

pub use self::error::Error;

#[derive(Args, Debug)]
pub struct Command {
    #[arg(name = "file-name", value_hint(ValueHint::FilePath))]
    old_file_name: PathBuf,
}

impl Command {
    pub fn run(self) -> Result<(), Error> {
        let mut editor = rustyline::Editor::<(), _>::new()?;

        if let Err(err) = std::fs::metadata(&self.old_file_name) {
            eprintln!("`{}` does not exist", self.old_file_name.display());

            return Err(
                error::FileNotExistSnafu { file_name: self.old_file_name.clone() }.into_error(err)
            );
        }

        let readline = editor.readline_with_initial(
            "rename to: ",
            (self.old_file_name.to_str().expect("`self.old_file_name` is a valid path; qed`"), ""),
        );
        let new_file_name = readline?.trim_end().to_string();

        if new_file_name.is_empty() {
            eprintln!(
                "Try to rename file `{}` to an empty file name",
                self.old_file_name.display()
            );

            return error::RenameToEmptyFileNameSnafu { file_name: self.old_file_name.clone() }
                .fail()?;
        }

        std::fs::rename(&self.old_file_name, &new_file_name).context(error::RenameFileSnafu {
            old_file_name: self.old_file_name.clone(),
            new_file_name,
        })?;

        Ok(())
    }
}
