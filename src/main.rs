#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use weirdly_long_language_i_guess::Result;
#[cfg(feature = "cli")]
mod cli {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser)]
    #[command(version, about, long_about = None, propagate_version = true)]
    struct Cli {
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(short = 'L', long)]
        long_license: bool,
        #[arg(short = 'l', long)]
        short_license: bool,
    }
    pub(crate) fn run() -> Result<()> {
        let cli = Cli::parse();
        if cli.long_license {
            println!(
                "{}",
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE"))
            );
        } else if cli.short_license {
            println!(
                "This project is license under the MIT license, a permissive software license."
            );
        }
        if let Some(file) = cli.file {
            std::fs::read_to_string(file)?;
        }
        Ok(())
    }
}
fn main() -> Result<()> {
    #[cfg(not(feature = "cli"))]
    compile_error!("Can't build binary without cli feature");
    #[cfg(feature = "cli")]
    cli::run()
}
