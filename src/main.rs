use clap::Subcommand;
use log::error;
use login::LoginCredentials;
use reqwest::blocking::ClientBuilder;
use std::sync::Arc;
mod config;
mod cookies;
mod download;
mod login;
mod run;
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
    /// Run the test cases
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    executable: String,
}

#[derive(Args)]
struct DownloadArgs {
    url: String,
}

#[derive(Args, Serialize, Deserialize)]
struct LoginArgs {
    username: String,
    password: String,
    /// save credentials to file
    #[arg(short, long)]
    save: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let jar_path = config::get_jar();
    match cli.command {
        Commands::Download(args) => {
            let cookie_store = cookies::load_jar(jar_path);
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
                cookies::save_jar(cookie_store, jar_path);
                if args.save {
                    let file = config::get_credentials_file();
                    let credentials = LoginCredentials::new(user, pass);
                    login::save_credentials(credentials, file);
                }
            }
        }
        Commands::Run(exe) => {
            // check if test cases exist
            run::run(exe.executable);
        }
    }
}
