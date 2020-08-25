extern crate reqwest;
use futures::future;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn save(n: usize) {
  let stories = fetch(n).await;
  match stories {
    Ok(stories) => {
      stories.into_iter().for_each(|story| {
        let file = File::create(format!("{}.json", story.id));
        match file {
          Ok(file) => to_writer(&file, &story),
          _ => (),
        };
      });
    }
    Err(_) => {}
  }
}
