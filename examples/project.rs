use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let project = "your project";
    let response = client.project(project).send().await.unwrap();
    println!("{:#?}", response);
}
