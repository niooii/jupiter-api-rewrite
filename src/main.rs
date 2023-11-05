#![allow(dead_code, unused_variables)]

mod jupiter;
mod scraper;
mod statics;
mod jupiter_endpoints;
mod stopwatch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut fetch_timer = stopwatch::Stopwatch::new();

    let osis = "".to_string();
    let password = "".to_string();

    scraper::login_jupiter(&osis, &password).await?;

    // fetch_timer.reset();

    scraper::get_all_data(&osis).await;

    println!("Fetch finished in {:?} seconds.", fetch_timer.elapsed_seconds());

    //let response = client.get(JUPITER_LOGIN).send().await.unwrap();

    Ok(())
}
