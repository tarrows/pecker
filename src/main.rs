extern crate reqwest;
// if no use StreamExt, said "note: the method `map` exists but the following trait bounds were not satisfied"
use futures::{stream, StreamExt};
use serde_json::json;
const PARALLEL_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let endpoint = "https://hacker-news.firebaseio.com/v0";
  let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get(&topstories).await?; // NG: get(topstories).await?

  let stories: Vec<u32> = res.json().await?;

  let bodies = stream::iter(stories)
    .take(3)
    .map(|id| {
      let item = format!("{}/item/{:?}.json", endpoint, id);
      async move {
        let res = reqwest::get(&item).await?;
        let json = res.json::<serde_json::Value>().await; // cannot infer type for T
        json
      }
    })
    .buffer_unordered(PARALLEL_REQUESTS);

  bodies
    .for_each(|b| {
      async {
        match b {
          Ok(b) => println!(
            "{:#?}",
            b.get("title").unwrap_or(&json!({ "title": "<NONE>" }))
          ),
          Err(e) => println!("Error: {}", e),
        }
      }
    })
    .await;

  Ok(())
}
