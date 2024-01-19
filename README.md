# Jupiter-Ed API 
A small program to fetch a user's information off the (arguably poorly written) Jupiter-Ed website.
### Only works for bronx science students, for now.

# Want to run?
1. Download `chromedriver` from this [here](https://chromedriver.chromium.org/downloads)
2. Start on port 4444 with `chromedriver --port=4444`.
3. Clone this repository, and run `cargo run --release`.

## This repository is a rewrite of my original one: [jupitered-api](https://github.com/niooii/jupitered-api)
### Some improvements:
- Not written in java.
- Grabs a lot more info.
- Uses session data instead of the webdriver the whole time.
- Response time from *8.4s* -> *1.8s*, *<200ms* if the session is still alive (abusing async)

# How it works
### First steps:
No matter how much I tried, I couldn't find a way to login via a web request. The scraper uses the [fantoccini](https://github.com/jonhoo/fantoccini) library to interact with a webdriver on port 4444. Upon logging in via inputting the osis && pwd into the respective fields, session data && cookies are stored and the webdriver session is closed.

### Grabbing courses, assignments, other data:
Jupiter has endpoints that are reusable with session information as query parameters. One such endpoint recieves a CourseID Cached from the previous webdriver session, returning HTML of the user's course page. This step is performed right after the initial login, so the session info should never be expired.

Course endpoint:
```rs
// unfortunately, most of these fields are necessary.
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
```

**Your password is NOT stored.** Here's what is stored though:
```rs
// the query parameters for most of jupiters endpoints, grabbed from the webdriver session
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
    gterm: String,

    class_ids_names: Vec<(String, String)>,
    raw_cookies: Vec<String>,
}
```

### How information is processed && sent back
This program gives three statuses to assignments:
```rs
pub enum AssignmentStatus {
  Missing,
  Graded,
  Ungraded
}
```
Assignments default to `Ungraded`. 

Assignments are `Graded` if: 
- The assignment is not missing.
- The score string is not empty, and first character can be converted into a radix of 10.
- The score does NOT start with "/", as ungraded assignments are usually formatted as "/3" or "/100".

Assignments are `Missing` if:
- The score string was found wrapped inside the <class="red"> element. 

All the information that's sent back:
```rs
#[derive(Default, Debug, Serialize)]
struct Assignment {
    name: String,
    date_due: String,    
    score: String,
    worth: u16,
    impact: String,
    category: String,
    status: AssignmentStatus
}
#[derive(Debug, Serialize)]
struct Course {
    name: String,
    teacher_name: String,
    place_and_time: String,
    num_missing: u16,
    num_graded: u16,
    num_ungraded: u16,
    num_total: u16,
    grades: Vec<GradeData>,
    assignments: Vec<Assignment>,
}

#[derive(Default, Debug, Serialize)]
struct GradeData {
    category: String,
    percent_grade: Option<f32>,
    fraction_grade: Option<String>,
    additional_info: Option<String>,
}

#[derive(Default, Debug, Serialize)]
// data is sent back in this form.
pub struct JupiterData {
    name: 
    String,
    osis: String,
    courses: Vec<Course>,
}
```
