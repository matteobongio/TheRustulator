use std::path::Path;

pub fn get_jar() -> String {
    "cookies.json".to_string()
}

pub fn get_credentials_file() -> String {
    let mut conf_dir = dirs::config_dir().unwrap();
    conf_dir.push("therustulator/");
    conf_dir.push("credentials.toml");
    conf_dir.to_str().unwrap().to_owned()
}
