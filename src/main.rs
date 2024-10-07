use serde::Deserialize;
use std::env;
use tokio::main;

use reqwest::{Error, Url};

const ENDPOINT: &str = "https://canvas.csuchico.edu/api/v1/courses/35697/quizzes";

#[derive(Deserialize, Debug)]
struct Quiz {
    id: i32,
}

#[main]
async fn main() -> Result<(), Error> {
    let access_token = env::var("CANVAS_ACCESS_TOKEN").unwrap();
    let url = Url::parse_with_params(ENDPOINT, &[("access_token", access_token)]).unwrap();
    let quizzes: Vec<Quiz> = reqwest::get(url).await?.json().await?;
    dbg!(&quizzes);
    // let api_response: Vec<Quiz> = reqwest::Client::new().get(url).send().await?.text();
    Ok(())
}
