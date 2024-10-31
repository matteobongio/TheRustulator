use std::{
    fs::File,
    io::{Read, Write},
    path,
};

use clap::builder::Str;
use log::{error, info};
use reqwest::blocking::Client;
use scraper::Selector;

pub fn login(client: Client, user: &str, pass: &str) -> Option<()> {
    let login_url = "https://themis.housing.rug.nl/log/in";
    let login_page = client.get(login_url).send().unwrap().text().unwrap();
    let csrf = get_csrf_token(&login_page).unwrap();
    info!("CRSF: {}", csrf);
    let form = [("user", user), ("password", pass), ("_csrf", &csrf)];
    let res = client.post(login_url).form(&form).send().unwrap();
    if !res.status().is_success() {
        error!("Error logging in");
        return None;
    }
    Some(())
}

fn get_csrf_token(login_page: &str) -> Option<String> {
    let document = scraper::Html::parse_document(login_page);
    let selector = Selector::parse("input[name='_csrf']").unwrap();
    if let Some(element) = document.select(&selector).next() {
        if let Some(csrf_token) = element.value().attr("value") {
            return Some(csrf_token.to_string());
        }
    }
    None
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
}
impl LoginCredentials {
    pub fn new(username: String, password: String) -> LoginCredentials {
        LoginCredentials { username, password }
    }
}

pub fn read_credentials(path: String) -> LoginCredentials {
    let mut file = File::open(&path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let str = String::from_utf8(buf).unwrap();
    toml::from_str(&str).unwrap()
}

pub fn save_credentials(credentials: LoginCredentials, path: String) {
    let str = toml::to_string_pretty(&credentials).unwrap();
    let mut file = File::create(&path).unwrap();
    file.write_all(str.as_bytes()).unwrap();
}
