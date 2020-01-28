extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let endpoint = "https://hacker-news.firebaseio.com/v0";
  let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get(&topstories).await?; // NG: get(topstories).await?
  println!("status = {:?}", res.status());
  let stories: Vec<u32> = res.json().await?;

  let mut stories = stories.into_iter();
  let id = stories.next().unwrap();
  println!("id = {:?}", id);
  let item = format!("{}/item/{:?}.json", endpoint, id);
  let res = reqwest::get(&item).await?;
  println!("status = {:?}", res.status());
  let story: serde_json::Value = res.json().await?; // NG: let story = ...

  println!("{:#?}", story);
  Ok(())
}
