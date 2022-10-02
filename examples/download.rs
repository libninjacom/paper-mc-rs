use paper_mc::PaperMcClient;
use paper_mc::model::*;
use paper_mc::request::DownloadRequired;
#[tokio::main]
async fn main() {
    let client = PaperMcClient::from_env();
    let args = DownloadRequired {
        version: "your version",
        project: "your project",
        build: 1,
        download: "your download",
    };
    let response = client.download(args).send().await.unwrap();
    println!("{:#?}", response);
}
