use clap::Parser;
use std::env;

pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser, PartialEq)]
#[clap(author, version, about, long_about= None)]
struct Args {
    #[clap(
        short,
        long,
        help = "Reduce printing other than the results and work quietly"
    )]
    quiet: bool,
}

#[tokio::main]
fn main()  -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    if !args.quiet {
        println!("Streaming Test CLI v{}", CLI_VERSION);
    }

    println!("Hello World!");

    Ok(());
}