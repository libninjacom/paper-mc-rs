use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let project = "your project";
    let family = "your family";
    let response = client.family_builds(project, family).send().await.unwrap();
    println!("{:#?}", response);
}
