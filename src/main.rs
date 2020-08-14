extern crate reqwest;
use pecker::crawler::crawl;

#[tokio::main]
async fn main() {
  let res = crawl(3).await;
  println!("{:?}", res);
}
