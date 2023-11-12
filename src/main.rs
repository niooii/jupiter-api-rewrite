#![allow(dead_code, unused_variables)]

use actix_web::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod scraper;
mod statics;
mod stopwatch;

const APPLICATION_JSON: &str = "application/json";

#[actix::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug,jupiter_api=trace");
    env_logger::init();
    
    HttpServer::new(
        || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(get_jupiter)
        }
    ).bind("127.0.0.1:9090")?
    .run()
    .await
}

#[derive(Deserialize)]
struct LoginInfo {
    osis: String,
    password: String,
}

#[get("jupiter")]
async fn get_jupiter(login: web::Query<LoginInfo>) -> HttpResponse {
    // fetch_timer.reset();

    let jd = scraper::get_all_data(&login.osis, &login.password ).await;

    if let Err(e) = jd {
        return HttpResponse::Unauthorized()
        .content_type(APPLICATION_JSON)
        .json(e);
    }

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(jd)
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
//     let osis = "".to_string();
//     let password = "".to_string();

//     scraper::login_jupiter(&osis, &password).await?;

//     // fetch_timer.reset();

//     scraper::get_all_data(&osis).await;

//     Ok(())
// }
