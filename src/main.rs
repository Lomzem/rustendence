use serde::Deserialize;
use std::env;
use std::error::Error;
use time::macros::offset;
use time::util::local_offset;
use tokio::main;

use reqwest::Url;

const ENDPOINT: &str = "https://canvas.csuchico.edu/api/v1/courses/35697/assignments";

#[derive(Deserialize, Debug)]
struct Assignment {
    quiz_id: Option<i32>,
    name: String,
    #[serde(with = "time::serde::iso8601")]
    due_at: time::OffsetDateTime,
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let access_token = env::var("CANVAS_ACCESS_TOKEN").unwrap();
    let url = Url::parse_with_params(ENDPOINT, &[("access_token", access_token)]).unwrap();
    let quizzes: Vec<Assignment> = reqwest::get(url).await?.json().await?;
    dbg!(&quizzes[0].due_at.to_offset(offset!(-6)));
    Ok(())
}
