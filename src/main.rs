#![allow(dead_code, unused_variables)]

mod jupiter;
mod scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut uc = scraper::UserCache::empty();
    let a = scraper::login_jupiter(&mut uc, "", "").await?;

    println!("{:?}", a);

    //let response = client.get(JUPITER_LOGIN).send().await.unwrap();
    // println!("{:#?}", response.text().await.unwrap_or(String::from("failed to unwrap...")));

    // println!("{:?}", res);

    Ok(())
}
