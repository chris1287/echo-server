mod machinery;

use clap::{Parser, ValueEnum};
use machinery::{server, client};

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

fn main() {
    let args = Args::parse();
    match args.mode {
        Mode::Server => server::serve(args.address.as_str()),
        Mode::Client => client::connect(args.address.as_str())
    };
}
