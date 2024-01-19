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
- Response time from *8.4s* -> *1.8s*, faster if the session is still alive. 
