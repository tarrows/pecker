extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let endpoint = "https://hacker-news.firebaseio.com/v0";
  let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get(&topstories).await?; // NG: get(topstories).await?
  println!("status = {:?}", res.status());
  let stories: Vec<u32> = res.json().await?;

  let stories = stories.into_iter();
  stories.take(3).for_each(|id| println!("{:?}", id));
  Ok(())
}
