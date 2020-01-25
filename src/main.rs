extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  // let endpoint = "https://hacker-news.firebaseio.com/v0";
  // let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await?;
  println!("status = {:?}", res.status());
  let stories: Vec<u32> = res.json().await?;
  println!("body:\n\n{:?}", stories);

  Ok(())
}
