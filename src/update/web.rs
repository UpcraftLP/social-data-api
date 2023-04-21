use std::num::ParseIntError;

use regex::Regex;
use thiserror::Error;

const USER_AGENT: &'static str = "";
const INSTAGRAM_URL: &'static str = "https://www.instagram.com";

pub struct InstagramResponse {
    pub username: String,
    pub follower_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("HTTP Error")]
    HttpError(#[from] reqwest::Error),
    #[error("Parser Error")]
    ParseError(#[from] ParseIntError),
    #[error("Unknown Error")]
    Unknown,
}

fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Unable to create HTTP client")
}

pub async fn get_instagram_user_data(username: &String) -> Result<InstagramResponse, RequestError> {
    let regex: Regex = Regex::new(r"(?P<follower_count>\d+) Followers, (?P<following_count>\d+) Following, (?P<post_count>\d+) Posts").expect("Unable to compile regex");
    let client = create_client();
    let result = client
        .get(format!("{}/{}", INSTAGRAM_URL, username))
        .send()
        .await
        .expect("error making web request");

    if result.status().is_success() {
        let body = result
            .text()
            .await
            .expect("unable to load request response text"); //TODO error handling

        let captures = regex.captures(&body);

        match captures {
            Some(capture) => {
                return Ok(InstagramResponse {
                    username: username.clone(),
                    follower_count: capture["follower_count"].parse::<i32>()?,
                    following_count: capture["following_count"].parse::<i32>()?,
                    posts_count: capture["follower_count"].parse::<i32>()?,
                })
            }
            None => {
                return Err(RequestError::Unknown); //TODO better error?
            }
        }
    }
    Err(RequestError::Unknown) //TODO status code error?
}
