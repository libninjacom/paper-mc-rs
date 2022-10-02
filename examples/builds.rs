use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let project = "your project";
    let version = "your version";
    let response = client.builds(project, version).send().await.unwrap();
    println!("{:#?}", response);
}
