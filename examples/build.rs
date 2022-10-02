use paper_mc::PaperMcClient;
use paper_mc::model::*;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let project = "your project";
    let version = "your version";
    let build = 1;
    let response = client.build(project, version, build).send().await.unwrap();
    println!("{:#?}", response);
}
