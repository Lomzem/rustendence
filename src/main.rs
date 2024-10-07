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
    #[serde(with = "time::serde::rfc3339")]
    due_at: time::OffsetDateTime,
    has_submitted_submissions: bool,
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let access_token = env::var("CANVAS_ACCESS_TOKEN").unwrap();
    let url = Url::parse_with_params(
        ENDPOINT,
        &[
            ("access_token", access_token.to_string()),
            ("per_page", "10000".to_string()),
        ],
    )
    .unwrap();

    let quizzes: Vec<Assignment> = reqwest::get(url)
        .await?
        .json::<Vec<Assignment>>()
        .await?
        .into_iter()
        .filter(|x| x.quiz_id.is_some())
        .filter(|x| !x.has_submitted_submissions)
        .collect();

    // let quizzes = reqwest::get(url).await?.json::<Vec<Assignment>>().await?.iter().map(|f|);

    dbg!(&quizzes);
    Ok(())
}
