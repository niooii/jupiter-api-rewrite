#![allow(dead_code, unused_variables)]

use actix_web::*;
use serde::{Serialize, Deserialize};
use serde_json::json;

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
                .service(login_jupiter)
                .service(get_jupiter)
        }
    ).bind("0.0.0.0:9090")?
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

    let jd = scraper::get_all_data(&login.osis, &login.password).await;

    if let Err(e) = jd {
        return HttpResponse::Unauthorized()
        .content_type(APPLICATION_JSON)
        .json(e);
    }

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(jd.unwrap())
}

#[get("login_jupiter")]
async fn login_jupiter(login: web::Query<LoginInfo>) -> HttpResponse {

    let jd = scraper::get_all_data(&login.osis, &login.password).await;

    if let Err(e) = jd {
        return HttpResponse::Unauthorized()
        .content_type(APPLICATION_JSON)
        .json(
            json!(
                {
                "success": false,
                "why": e
                }
            )
        );
    }

    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(
        json!(
            {
            "success": true
            }
        )
    )
}
