use std::collections::HashMap;

use tokio::sync::{Mutex};
use lazy_static::lazy_static;

use crate::scraper::UserCache;
use reqwest::Client;
use once_cell::sync::Lazy;

lazy_static!{
    pub static ref CLIENT_CACHE_MAP: Mutex<HashMap<String,(UserCache, Client)>> = {
        let m = HashMap::new();

        Mutex::new(m)
    };
}