extern crate reqwest;
use pecker::crawler::fetch;

#[tokio::main]
async fn main() {
  let res = fetch(3).await;
  println!("{:?}", res);
}
