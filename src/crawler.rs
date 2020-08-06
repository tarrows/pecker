extern crate reqwest;

use futures::{stream, StreamExt};
const PARALLEL_REQUESTS: usize = 2;

#[derive(Deserialize, Debug)]
pub struct Story {
  id: i64,
  type: String,
  by: String,
  descendants: i64,
  kids: Option<Vec<i64>>,
  score: i64,
  time: i64,
  title: String,
  url: Option<String>,
}

pub fn crawl(n: i32) -> Result<(), reqwest::Error> {
  let endpoint = "https://hacker-news.firebaseio.com/v0";
  let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get(&topstories).await?;

  let stories: Vec<u32> = res.json().await?;

  let bodies = stream::iter(stories)
    .take(n)
    .map(|id| {
      async move {
        let item = format!("{}/item/{:?}.json", endpoint, id);
        let res = reqwest::get(&item).await?;
        let json = res.json::<serde_json::Value>().await;
        json
      }
    })
    .buffer_unordered(PARALLEL_REQUESTS);

  bodies
    .for_each(|b| {
      async {
        match b {
          Ok(b) => println!("{}", form(b)),
          Err(e) => println!("Error: {}", e),
        }
      }
    })
    .await;

  Ok(())
}

fn form(j: serde_json::Value) -> String {
  let id = j.get("id").and_then(|x| x.as_i64()).unwrap_or(-1);
  let title = j.get("title").and_then(|x| x.as_str()).unwrap_or("");
  format!("{}: {}", id, title)
}
