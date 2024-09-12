mod models;
mod services;
use log::error;
use services::io::read_transactions;
use services::io::write_accounts;
use std::env;

fn main() {
    env_logger::init();

    // Process command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Usage: cargo run -- [INPUT_FILE] > [OUTPUT_FILE]");
        std::process::exit(1);
    }

    // Read input CSV and process transactions
    let accounts = read_transactions(&args[1]).unwrap();

    // Write output CSV with final account details
    write_accounts(accounts);
}
