//! COPY FROM bin/main.rs

use dotenv::dotenv;
use clap::Parser;
use sea_orm_cli::*;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let cli = cli::Cli::parse();

}
