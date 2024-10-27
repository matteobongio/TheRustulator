use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

pub fn load_jar(path: String) -> Arc<CookieStoreMutex> {
    let cookie_store = {
        let file = std::fs::File::open(path)
            .map(std::io::BufReader::new)
            .unwrap();
        // use re-exported version of `CookieStore` for crate compatibility
        reqwest_cookie_store::CookieStore::load_json(file).expect("unable to parse cookie JSON")
    };
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    std::sync::Arc::new(cookie_store)
}

pub fn new_jar() -> Arc<CookieStoreMutex> {
    let cookie_store = reqwest_cookie_store::CookieStore::new(None);
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    std::sync::Arc::new(cookie_store)
}

pub fn save_jar(cookie_store: Arc<CookieStoreMutex>, path: String) {
    {
        // Write store back to disk
        let mut writer = std::fs::File::create(path)
            .map(std::io::BufWriter::new)
            .unwrap();
        let store = cookie_store.lock().unwrap();
        store.save_json(&mut writer).unwrap();
    }
}
