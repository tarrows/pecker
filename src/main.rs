extern crate reqwest;
use pecker::crawler::fetch;

#[tokio::main]
async fn main() {
  let res = fetch(3).await;
  match res {
    Ok(res) => res
      .into_iter()
      .map(|s| format!("{} {}", s.id, s.title))
      .for_each(|s| println!("{}", s)),
    Err(res) => println!("{:?}", res),
  };
}
