mod machinery;

use clap::{Parser, ValueEnum};
use machinery::{server, client};
use anyhow::{Result, Context};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short, long, value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Client,
    Server,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.mode {
        Mode::Server => server::serve(args.address.as_str()).context("server execution failed")?,
        Mode::Client => client::connect(args.address.as_str()).context("client execution failed")?
    };

    Ok(())
}
