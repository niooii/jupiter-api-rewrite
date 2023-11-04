use fantoccini::{ClientBuilder, Locator, cookies::Cookie};
use std::{time::Duration, sync::Arc, future, collections::HashMap};
use tokio::{time::sleep, join};

const JUPITER_LOGIN: &str = "https://login.jupitered.com/login/index.php";

const CONTENTTYPE_FORM: &str = "application/x-www-form-urlencoded";

static mut CACHE: HashMap<reqwest::Client, UserCache> = HashMap::new();

#[derive(Debug, Default)]
pub struct UserCache {
    mini: String,
    session: String,
    server: String,
    district: String,
    school: String,
    year: String,
    stud: String,
    contact: String,
    datemenu: String,
    //class1: String, FIELD IS
    gterm: String,
    ass: String,

    class_ids: Vec<String>,
    raw_cookies: Vec<String>,
}

impl UserCache {
    pub fn empty() -> UserCache {
        UserCache {
            ..Default::default()
        }
    }
}

/// Send keys to textfield with identifier "id=blahblah"
async fn send_keys(client: &fantoccini::Client, id: &str, string: &str) {
    client
        .find(Locator::Id(id))
        .await
        .unwrap_or_else(|_| panic!("Failed to search for element with id {id}"))
        .send_keys(string)
        .await
        .unwrap_or_else(|_| panic!("Failed to write {string} into element with id {id}."));
}

/// Modifies an instance of UserCache, can be used to update the request payload.
/// Uses Chromedriver on port 4444.
pub async fn login_jupiter(
    cache: &mut UserCache,
    osis: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // Go to jupiter website
    client.goto(JUPITER_LOGIN).await?;

    // Wait for first box to show up in form (osis box)
    let osis_area = client
        .wait()
        .for_element(Locator::Id("text_studid1"))
        .await?;

    osis_area.send_keys(osis).await?;

    send_keys(&client, "text_password1", password).await;
    send_keys(&client, "text_school1", "Bronx High School Of Science").await;
    send_keys(&client, "text_city1", "Bronx").await;

    // Click on states menu
    client
        .find(Locator::Id("region1_label"))
        .await?
        .click()
        .await?;

    // Go to new york
    client.active_element().await?.send_keys("New York").await?;

    let ny_statebutton = client
        .wait()
        .for_element(Locator::XPath("//div[contains(@val, 'us_ny')]"))
        .await?;

    // Wait for element to be ready to be clicked
    sleep(Duration::from_millis(100)).await;

    ny_statebutton.click().await?;

    //click login
    client.find(Locator::Id("loginbtn")).await?.click().await?;

    let html: String = client.source().await?;

    // TODO: handle invalid login using response.

    //grab fields from html and update cache struct
    // TODO: FIX DIABOLICAL CODE BLOCK
    cache.mini = client.wait().for_element(Locator::XPath("//input[contains(@name, 'from')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.session = client.find(Locator::XPath("//input[contains(@name, 'session')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.server = client.find(Locator::XPath("//input[contains(@name, 'server')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.district = client.find(Locator::XPath("//input[contains(@name, 'district')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.school = client.find(Locator::XPath("//input[contains(@name, 'school')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.year = client.find(Locator::XPath("//input[contains(@name, 'school')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.stud = client.find(Locator::XPath("//input[contains(@name, 'stud')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.contact = client.find(Locator::XPath("//input[contains(@name, 'contact')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.datemenu = client.find(Locator::XPath("//input[contains(@name, 'datemenu')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.gterm = client.find(Locator::XPath("//input[contains(@name, 'gterm')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.ass = client.find(Locator::XPath("//input[contains(@name, 'ass')]")).await?.attr("value").await?.unwrap_or(String::new());


    // TODO: FIND ALL THE "CLASS IDS".
    // TODO: click="postval('class1',5640488); go('grades');"
    cache.class_ids.clear();

    let class_id_containers = client.find_all(Locator::XPath("//div[@class='navrow' and @click]")).await.expect("could not perform search");

    for element in class_id_containers {
        let str_option = element.attr("click").await?;
        if str_option.is_none() {
            continue;
        }

        let str = str_option.unwrap();
        let begin_idx = str.find("',").unwrap() + 2;
        let end_idx = str.find(')').unwrap();
        
        cache.class_ids.push(str[begin_idx..end_idx].to_string());
    }

    println!("{:?}", cache.class_ids);
    
    cache.raw_cookies = client
        .get_all_cookies()
        .await?
        .iter()
        .map(|c| c.to_string())
        .collect();


    do_shit_with_html(cache).await;

    client.close().await?;

    Ok(())
}

fn todo_endpoint(cache: &UserCache) -> String {
    format!("https://login.jupitered.com/0/student.php?w={},{},0&from=grades&to=todo&todo=&mini=0&session={}&server=1&district={}&school={}&year={}&stud={}&contact={}&gterm={}&ass={}&pagecomplete=1&busymsg=Loading"
    , cache.school,
    cache.stud, 
    cache.session,//session
    cache.school, //district
    cache.school, //school
    cache.year,
    cache.stud,
    cache.contact,
    cache.gterm,
    cache.ass
    )
}

fn class_endpoint(cache: &UserCache, class_id: &String) -> String {
    format!("https://login.jupitered.com/0/student.php?w={},{},0&from=todo&to=grades&todo=&mini=0&session={}&server=1&district={}&school={}&year={}&stud={}&contact={}&class1={}&gterm={}&ass={}&pagecomplete=1&busymsg=Loading"
    , cache.school,
    cache.stud, 
    cache.session,//session
    cache.school, //district
    cache.school, //school
    cache.year,
    cache.stud,
    cache.contact,
    class_id,
    cache.gterm,
    cache.ass
    )
}

// TODO: make a global reqwest client, the "requestor" client.
// TODO: function that accepts &mut UserCache and uses global requestor to fetch a page with usercache stuff, then scrape some elements.

fn build_client(cookies: &Vec<String>) -> reqwest::Client {
    let cookie_jar = reqwest::cookie::Jar::default();

    let mut cookie_str = String::new();

    for cookie in cookies {
        // cookie_jar.add_cookie_str(cookie, &endpoint.parse::<reqwest::Url>().unwrap());
        cookie_str += cookie.as_str();
    }

    let client_builder = reqwest::ClientBuilder::new();

    // print!("{:?}", cookie_jar);

    client_builder
    .https_only(true)
    .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
    .referer(false)
    .cookie_store(true)
    .cookie_provider(Arc::new(cookie_jar))
    .https_only(true)
    .build()
    .expect("ouch")
}

async fn get_site_html(endpoint: &str, request_client: reqwest::Client) -> scraper::Html {    
    

    let res = request_client
        .get(endpoint)
        .send()
        .await
        .expect("failed to get html response.");
    
    let html_string = res.text()
    .await
    .expect("failed to get text from html document.");

    println!("{html_string}");

    scraper::Html::parse_document(
        &html_string
    )

}

async fn do_shit_with_html(cache: &UserCache) {
    //make a request client for now, i will optimize later.
    
    

    let url = class_endpoint(cache, &cache.class_ids[0]);

    println!("{url}");

    let html = get_site_html(&url, &cache.raw_cookies).await;
}
