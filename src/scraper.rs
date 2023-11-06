use fantoccini::{ClientBuilder, Locator};
use scraper::Selector;
use std::collections::HashMap;
use std::{time::Duration, sync::Arc};
use tokio::{time::sleep};
use futures::future::join_all;

use log::debug;
use log::error;
use log::info;
use log::warn;


use crate::{statics::*};
use crate::stopwatch::Stopwatch;

const JUPITER_LOGIN: &str = "https://login.jupitered.com/login/index.php";

const CONTENTTYPE_FORM: &str = "application/x-www-form-urlencoded";

#[derive(Debug, Hash, Default, Clone)]
pub struct UserCache {
    osis: String,
    password: String,

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
async fn send_keys(client: &fantoccini::Client, id: &str, string: &str) -> Result<(), fantoccini::error::CmdError>{
    client
        .find(Locator::Id(id))
        .await
        .unwrap_or_else(|_| panic!("Failed to search for element with id {id}"))
        .send_keys(string)
        .await
        .unwrap_or_else(|_| panic!("Failed to write {string} into element with id {id}."));
    Ok(())
}

async fn create_usercache(html: &String) {

}

/// Modifies an instance of UserCache, can be used to update the request payload.
/// Uses Chromedriver on port 4444.
pub async fn login_jupiter(
    osis: &String,
    password: &String,
) -> Result<(), Box<dyn std::error::Error>> {

    if osis.is_empty() {
        error!("Some idiot didn't enter an osis.");
        return Err(Box::from("Enter your osis number."));
    }
    if osis.len() != 9 {
        error!("Invalid osis length: {}, length {}", osis, osis.len());
        return Err(Box::from("Enter a valid osis number."))
    }
    if password.is_empty() {
        error!("Some idiot didn't enter a password.");
        return Err(Box::from("Enter your password."));
    }

    // I like seeing numbers
    let log_timer = Stopwatch:: new();

    // Creates chrome capabilities
    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({
        "args": ["--headless"   , "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
    });
    caps.insert("goog:chromeOptions".to_string(), opts);

    // Connect to webdriver instance that is listening on port 4444
    let client = ClientBuilder::native()
    .capabilities(caps)
        .connect("http://localhost:4444")
        .await?;

    // Go to jupiter website
    client.goto(JUPITER_LOGIN).await?;

    // Wait for first box to show up in form (osis box)
    client
        .wait()
        .for_element(Locator::Id("text_studid1"))
        .await?;

    // i love saving 20 milliseconds
    // hello from the future, this actually saved a 
    // notable 0.4 seconds. very amazing
    let key_futures = vec![
        send_keys(&client, "text_studid1", osis)
        , send_keys(&client, "text_password1", password)
        , send_keys(&client, "text_school1", "Bronx High School Of Science")
        , send_keys(&client, "text_city1", "Bronx")
        ];

    join_all(key_futures).await;

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
    sleep(Duration::from_millis(75)).await;

    ny_statebutton.click().await?;

    //click login
    client.find(Locator::Id("loginbtn")).await?.click().await?;

    let html: String = client.source().await?;

    // TODO: handle invalid login using response.

    //grab fields from html and update cache struct
    // TODO: FIX DIABOLICAL CODE BLOCK
    let mut cache = UserCache {
        .. Default::default()
    };

    // This may be useless, but I don't want to remove it.
    cache.mini = client.wait().for_element(Locator::XPath("//input[contains(@name, 'mini')]")).await?.attr("value").await?.unwrap_or(String::new());
    
    // If webdriver cannot find session, the login has failed.
    let session_element = client.find(Locator::XPath("//input[contains(@name, 'session')]")).await;
    if let Ok(e) = session_element {
        cache.session = e.attr("value").await?.unwrap_or(String::new());
    }
    else {
        // Error has occured logging in. Get error string and send it back to the user. 
        let error_element = client.find(Locator::XPath("//div[contains(@class, 'alert center rad12')]")).await?;
        let error_str = error_element.text().await?;
        error!("Could not login: {error_str}");
        error!("Finished error task in {} seconds.", log_timer.elapsed_seconds());
        return Err(Box::from("Failed login."));
    }
    
    cache.server = client.find(Locator::XPath("//input[contains(@name, 'server')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.district = client.find(Locator::XPath("//input[contains(@name, 'district')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.school = client.find(Locator::XPath("//input[contains(@name, 'school')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.year = client.find(Locator::XPath("//input[contains(@name, 'school')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.stud = client.find(Locator::XPath("//input[contains(@name, 'stud')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.contact = client.find(Locator::XPath("//input[contains(@name, 'contact')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.datemenu = client.find(Locator::XPath("//input[contains(@name, 'datemenu')]")).await?.attr("value").await?.unwrap_or(String::new());
    cache.gterm = client.find(Locator::XPath("//input[contains(@name, 'gterm')]")).await?.attr("value").await?.unwrap_or(String::new());

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

    // println!("{:?}", cache.class_ids);
    
    cache.raw_cookies = client
        .get_all_cookies()
        .await?
        .iter()
        .map(|c| c.to_string())
        .collect();

    client.close().await?;

    info!("Grabbed cookies && session info in {} seconds.", log_timer.elapsed_seconds());

    let req_client = build_client(&cache.raw_cookies);

    // Lock mutex, allow inserting to map.
    let mut guard = CLIENT_CACHE_MAP.lock().await; 

    guard.insert(osis.to_string(), (cache, req_client));

    // Mutex unlocks when gjard goes out of scope.
    Ok(())
}

// TODO: make a global reqwest client, the "requestor" client.
// TODO: function that accepts &mut UserCache and uses global requestor to fetch a page with usercache stuff, then scrape some elements.

fn build_client(cookies: &Vec<String>) -> reqwest::Client {
    let cookie_jar = reqwest::cookie::Jar::default();

    for cookie in cookies {
        cookie_jar.add_cookie_str(cookie, &"https://login.jupitered.com/".parse::<reqwest::Url>().unwrap());
    }

    let client_builder = reqwest::ClientBuilder::new();
    
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


fn todo_endpoint(cache: &UserCache) -> String {
    format!("https://login.jupitered.com/0/student.php?w={},{},0&from=grades&to=todo&todo=&mini=0&session={}&server=1&district={}&school={}&year={}&stud={}&contact={}&gterm={}&ass=&pagecomplete=1&busymsg=Loading"
    , cache.school,
    cache.stud, 
    cache.session,//session
    cache.school, //district
    cache.school, //school
    cache.year,
    cache.stud,
    cache.contact,
    cache.gterm,
    )
}

fn course_endpoint(cache: &UserCache, course_id: &String) -> String {
    format!("https://login.jupitered.com/0/student.php?w={},{},0&from=todo&to=grades&todo=&mini=0&session={}&server=1&district={}&school={}&year={}&stud={}&contact={}&class1={}&gterm={}&ass=&pagecomplete=1&busymsg=Loading"
    , cache.school,
    cache.stud, 
    cache.session,//session
    cache.school, //district
    cache.school, //school
    cache.year,
    cache.stud,
    cache.contact,
    course_id,
    cache.gterm,
    )
}

// The screen that pops up when you click an assignment is so incredibly useless, I'm not even going to include it at all.
// fn assignment_endpoint(cache: &UserCache, assignment_id: &String) -> String {
//     format!("https://login.jupitered.com/0/student.php?w={},{},0&from=grades&to=assignment&todo=&mini=0&session={}&server=1&district={}&school={}&year={}&stud={}&contact={}&gterm={}&ass={}&pagecomplete=1&busymsg=Loading"
//     , cache.school,
//     cache.stud, 
//     cache.session,//session
//     cache.school, //district
//     cache.school, //school
//     cache.year,
//     cache.stud,
//     cache.contact,
//     cache.gterm,
//     assignment_id,
//     )
// }


async fn get_site_html(endpoint: &str, request_client: &reqwest::Client) -> scraper::Html {    
    
    let res = request_client
        .get(endpoint)
        .send()
        .await
        .expect("failed to get html response.");
    
    let html_string = res.text()
    .await
    .expect("failed to get text from html document.");

    scraper::Html::parse_document(
        &html_string
    )

}

#[derive(Default, Debug)]
struct Assignment {
    id: String,
    name: String,
    date_due: String,    
    score: String,
    worth: u16,
    impact: String,
    category: String,


}
#[derive(Debug)]
struct Course {
    name: String,
    grades: HashMap<String, String>,
    assignments: Vec<Assignment>,
}

// this function may be a SLIGHT problem for jupiter if my app ever scales up. maybe fix later =)
// nevermind. this is incredibly useless.
// async fn get_assignment_data(cache: &UserCache, assignment_id: &String, client: &reqwest::Client) -> Assignment {
//     let html = get_site_html(&assignment_endpoint(cache, assignment_id), client).await;

//     // println!("got assignment data lol");

//     Assignment {

//     }
// }

// clippy is making me ANGRY.
// This takes the <tbody> element with class "hi ..." (this contains all info about assignments)
async fn parse_assignment_from_element(element: scraper::ElementRef<'_>) -> Assignment {
    let s = Selector::parse("td").unwrap();

    let info_element = element.select(&s);

    let mut assignment = Assignment {
        ..Default::default()
    };

    for ie in info_element {
        
        let class_attr = ie.attr("class");

        if class_attr.is_none() {

            // if the inner html isn't empty and it doesn't have a class attr, it is the due date.
            let date_apparently = ie.inner_html();
            if !date_apparently.is_empty() {
                assignment.date_due = date_apparently; 
            }
            continue;
        }
        
        
        let class_str = class_attr.unwrap();
        let text = ie.inner_html();

        match class_str {

            // name
            "pad12 wrap asswidth" => assignment.name = text,

            // score (in x / y form)
            "pad20 right" => assignment.score = text.replace(' ', ""),

            // worth
            "right landonly" => {
                if let Ok(val) = text.parse::<u16>() {
                    assignment.worth = val;
                }
            },

            // impact
            "pad20 padr8 right alandonly" => assignment.impact = text,

            // category
            "pad20 alandonly" => assignment.category = text,
            _ => continue,
        }

    }
    assignment
}

async fn get_course_data(cache: &UserCache, course_id: &String, client: &reqwest::Client) -> Course {
    
    let html = get_site_html(&course_endpoint(cache, course_id), client).await;

    // Get assignment ids
    let id_selector = Selector::parse("tbody.hi ").expect("failed to parse selector");

    let element_iter = html.select(&id_selector);

    // println!("{:?}", assignment_ids);

    // Get assignments for this course 
    let futures: Vec<_> = element_iter
    .map(|element| {parse_assignment_from_element(element)})
    .collect();

    let assignments = join_all(futures).await;

    // println!("{:?}", assignments);

    // Get course name

    let name = "awf".to_string();
    
    let grade_field_map = HashMap::new();

    Course {
        name,
        grades: grade_field_map,
        assignments,
    }
}
use std::sync::Mutex;
// Takes a mutable reference to JupiterData to modify the elements of courses of within the function.
// Solves an async issue.
async fn get_courses(cache: &UserCache, client: &reqwest::Client, jd: &Mutex<JupiterData>) {
    
    let futures: Vec<_> = cache.class_ids
    .iter()
    .map(|course_id| {get_course_data(cache, course_id, client)})
    .collect();

    let courses = join_all(futures).await;

    let mut guard = jd.lock().unwrap();
    guard.courses = courses;
}

// this sounds very bad out of context.
// Also accepts a mut ref to juptierdata.
async fn get_personal_info(cache: &UserCache, client: &reqwest::Client, jd: &Mutex<JupiterData>) {
    let todo_endpoint = todo_endpoint(cache);
    let html = get_site_html(&todo_endpoint, client).await;

    let name = html
    .select(&Selector::parse("div.toptabnull").unwrap())
    .next()
    .unwrap()
    .inner_html();

    let mut guard = jd.lock().unwrap();
    guard.name = name.trim().to_string();
}

pub async fn get_all_data(osis: &String) {
    
    let log_timer = Stopwatch::new();

    let cachemap_guard = CLIENT_CACHE_MAP.lock().await; 

    let (cache, client) = &cachemap_guard.get(osis).unwrap();

    let jd = JupiterData {
        ..Default::default()
    };

    let jd = Mutex::new(jd);

    futures::join!(
        get_courses(cache, client, &jd),
        get_personal_info(cache, client, &jd),
    );

    let guard =  jd.lock().unwrap();

    // println!("{:?}", guard);

    info!("Finished fetching data for {} in {} seconds.", guard.name, log_timer.elapsed_seconds());
}

#[derive(Default, Debug)]
struct JupiterData {
    name: String,

    courses: Vec<Course>,
}