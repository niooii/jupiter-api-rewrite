#![allow(dead_code, unused_variables)]

mod scraper;
mod statics;
mod stopwatch;
// a
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "jupiter_api=trace");
    env_logger::init();

    let osis = "".to_string();
    let password = "".to_string();

    scraper::login_jupiter(&osis, &password).await?;

    // fetch_timer.reset();

    scraper::get_all_data(&osis).await;

    Ok(())
}
