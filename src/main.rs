mod cli;

use self::cli::Cli;

fn main() {
    if let Err(err) = Cli::default().run() {
        eprintln!("{err}");
        std::process::exit(-1);
    }
}
