use spider::tokio;
use spider::website::Website;
// use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("https://music.163.com/playlist?id=422276169");
    let mut rx2 = website.subscribe(16).unwrap();

    tokio::spawn(async move {
        let mut stdout = stdout();

        while let Ok(res) = rx2.recv().await {
            let _ = stdout
                .write_all(format!("- {}\n", res.get_url()).as_bytes())
                .await;
        }
    });

    website.crawl().await;
}
