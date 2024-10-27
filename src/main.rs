use clap::Subcommand;
use log::error;
use reqwest::blocking::ClientBuilder;
use std::sync::Arc;
mod cookies;
mod login;
mod download;
use clap::{Args, Parser};
use serde::{Deserialize, Serialize};

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
    Download(DownloadArgs),
}

#[derive(Args)]
struct DownloadArgs {
    url: String,
}

#[derive(Args, Serialize, Deserialize)]
struct LoginArgs {
    username: String,
    password: String,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Download(args) => {
            let cookie_store = cookies::load_jar("cookies.json".to_owned());
            let client = ClientBuilder::new()
                .cookie_provider(Arc::clone(&cookie_store))
                .build()
                .expect("unable to build reqwest client");
            let url = args.url;
            download::download(client, url);
        }
        Commands::Login(args) => {
            let user = args.username;
            let pass = args.password;
            let cookie_store = cookies::new_jar();
            let client = ClientBuilder::new()
                .cookie_provider(Arc::clone(&cookie_store))
                .build()
                .expect("unable to build reqwest client");
            if login::login(client, &user, &pass).is_none() {
                error!("cannot login");
            } else {
                cookies::save_jar(cookie_store, "cookies.json".to_owned());
            }
        }
    }
}
