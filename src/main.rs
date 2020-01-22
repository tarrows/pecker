extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  // let endpoint = "https://hacker-news.firebaseio.com/v0";
  // let topstories = endpoint.clone().to_owned() + "/topstories.json";
  let res = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await?;
  println!("status = {:?}", res.status());
  let body = res.text().await?;
  println!("body:\n\n{}", body);

  Ok(())
}
