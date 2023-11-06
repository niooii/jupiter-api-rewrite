use std::collections::HashMap;

use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::scraper::UserCache;
use once_cell::sync::Lazy;
use reqwest::Client;

lazy_static! {
    pub static ref CLIENT_CACHE_MAP: Mutex<HashMap<String, (UserCache, Client)>> = {
        let m = HashMap::new();

        Mutex::new(m)
    };
}
