mod cryptocurrency_price;
mod myip;
mod rename;

use std::io::Write;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Show current version")]
    Version,

    #[command(about = "Show shell completions")]
    Completions { shell: Shell },

    #[command(name = "myip", about = "Show IP address of current host")]
    MyIp(myip::Command),

    #[command(name = "rename-file", about = "Rename a file")]
    RenameFile(rename::Command),

    #[command(name = "cryptocurrency-price", about = "Show cryptocurrency price")]
    CryptocurrencyPrice(cryptocurrency_price::Command),
}

impl Cli {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.commands {
            Commands::Version => {
                let mut stdout = std::io::stdout();
                stdout.write_all(Self::command().render_long_version().as_bytes())?;
                Ok(())
            }
            Commands::Completions { shell } => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                Ok(())
            }
            Commands::MyIp(cmd) => Ok(cmd.run()?),
            Commands::RenameFile(cmd) => Ok(cmd.run()?),
            Commands::CryptocurrencyPrice(cmd) => Ok(cmd.run()?),
        }
    }
}

impl Default for Cli {
    #[inline]
    fn default() -> Self { Self::parse() }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{Cli, Commands};

    #[test]
    fn test_cli_simple() {
        matches!(Cli::parse_from(["program_name", "version"]).commands, Commands::Version);
    }
}
