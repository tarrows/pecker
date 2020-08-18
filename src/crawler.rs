extern crate reqwest;
use futures::future;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Story {
  pub id: i64,
  r#type: String,
  by: String,
  descendants: i64,
  kids: Option<Vec<i64>>,
  score: i64,
  time: i64,
  pub title: String,
  url: Option<String>,
}

pub async fn fetch(n: usize) -> Result<Vec<Story>, reqwest::Error> {
  let endpoint = "https://hacker-news.firebaseio.com/v0";
  let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get(&topstories).await?;

  let stories: Vec<u32> = res.json().await?;

  let bodies = future::join_all(stories.into_iter().take(n).map(|id| {
    async move {
      let item = format!("{}/item/{:?}.json", endpoint, id);
      let res = reqwest::get(&item).await?;
      let json = res.json::<Story>().await;
      json
    }
  }))
  .await;

  bodies.into_iter().collect()
}
