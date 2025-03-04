use anyhow::Result;
use clap::Parser;

use buffett::get_report;

#[derive(Parser)]
/// Checks the recent price history for a given stock symbol and reports whether it's a good buy.
struct Args {
    /// Stock or fund symbol
    stock: String
}

fn main() -> Result<()> {
    let args = Args::parse();
    let report = get_report(&args.stock)?;
    println!("{report}");
    Ok(())
}
