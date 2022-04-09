
use dotenv::dotenv;
use clap::Parser;
use sea_orm_cli::*;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let matches = cli::Cli::parse();

    // match matches.subcommand() {
        // ("generate", Some(matches)) => run_generate_command(matches)
            // .await
            // .unwrap_or_else(handle_error),
        // ("migrate", Some(matches)) => run_migrate_command(matches).unwrap_or_else(handle_error),
        // _ => unreachable!("You should never see this message"),
    // }
}
