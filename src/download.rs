use reqwest::blocking::Client;
use std::io::Result;
use scraper::Selector;
use std::{fs::File, io::Write, path::Path};

pub fn download(client: Client, url: String) {
    let tests = get_test_links(&client, &url);
    for test in tests {
        download_test(&client, &test).unwrap();
    }
}

fn get_test_links(client: &Client, url: &str) -> Vec<String> {
    let body = client.get(url).send().unwrap().text().unwrap();
    let doc = scraper::Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();
    let mut tests: Vec<String> = Vec::new();
    for element in doc.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.contains("%40tests/") {
                let test_link = format!("https://themis.housing.rug.nl{}", href);
                println!("{}", test_link);
                tests.push(test_link);
            }
        }
    }
    tests
}

fn download_test(client: &Client, url: &str) -> Result<usize> {
    let filename = url.split('/').last().unwrap();
    let path_str = {
        if filename.ends_with("out") {
            "./tests/out/".to_string() + filename
        } else {
            "./tests/in/".to_string() + filename
        }
    };
    std::fs::create_dir_all("./tests/in/").expect("can't create in dir");
    std::fs::create_dir_all("./tests/out/").expect("can't create out dir");
    let path = Path::new(&path_str);
    let mut dest = File::create(Path::new(path)).unwrap();
    let data = client.get(url).send().unwrap().bytes().unwrap();
    dest.write(&data)
}
