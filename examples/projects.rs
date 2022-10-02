use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let response = client.projects().send().await.unwrap();
    println!("{:#?}", response);
}
