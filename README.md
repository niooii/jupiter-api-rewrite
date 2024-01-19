# Jupiter-Ed API 
A small program to fetch a user's information off the (arguably poorly written) Jupiter-Ed website.
### Only works for bronx science students, for now.

# Want to run?
1. Download `chromedriver` from this [here](https://chromedriver.chromium.org/downloads)
2. Start on port 4444 with `chromedriver --port=4444`.
3. Clone this repository, and run `cargo run --release`.

## This repository is a rewrite of my original one: [jupitered-api](https://github.com/niooii/jupitered-api)
### Some improvements:
- Not written in java
- Uses session data instead of the webdriver the whole time.
- Response time from *8.4s* -> *1.8s*, *<200ms* if the session is still alive. 

# How it works
### First steps:
No matter how much I tried, I couldn't find a way to login via a web request. The scraper uses the [fantoccini](https://github.com/jonhoo/fantoccini) library to interact with a webdriver on port 4444. Upon logging in via inputting the osis && pwd into the respective fields, session data && cookies are stored and the webdriver session is closed.

### Grabbing courses, assignments, other data:
Jupiter has endpoints that are reusable with session information as query parameters. One such endpoint recieves a course ID grabbed from the previous webdriver session, and cached for the user. This step is performed right after the initial login, so the session info should never be expired.

### How information is processed
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
- THe score string was found wrapped inside the <class="red"> element. 
