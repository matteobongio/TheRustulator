use clap::Subcommand;
use reqwest::blocking::ClientBuilder;
use std::process::exit;
mod login;
use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set Login information
    Login(LoginArgs),
    /// Download Test Cases
    Download,
}

#[derive(Args)]
struct LoginArgs {
    username: String,
    password: String,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let client = ClientBuilder::new()
        .cookie_store(true)
        .build()
        .expect("unable to build reqwest client");
    match cli.command {
        Commands::Download => {}
        Commands::Login(args) => {
            let user = args.username;
            let pass = args.password;
            if login::login(client, &user, &pass).is_none() {
                exit(1);
            }
        }
    }
}
