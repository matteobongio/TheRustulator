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
