use reqwest;
use scraper;
use tokio;

//use tokio::time::{sleep, Duration};
//use rand::{thread_rng, Rng};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let max_concurrent = 2;
    let ids: Vec<u64> = (1..=10).into_iter().collect();
    let mut join_set = JoinSet::new();
    
    for id in ids {
        while join_set.len() >= max_concurrent {
            join_set.join_next().await.unwrap().unwrap();
        }
        join_set.spawn(async move {
            let res = reqwest::Client::new()
                .get(format!("https://music.163.com/playlist?id={}", id))
                .send()
                .await?.text().await?
            let select = scraper::Selector::parse("div.user > span.name > a").unwrap();
            if let Some(name) = scraper::Html::parse_document(res).select(&select).next(){
                if name.value() == "PurionPurion"{
                    println!("{:?}", id);
                }
            }
        });
    }
    
    println!("DONE SPAWNING");
    
    while let Some(output) = join_set.join_next().await {
        output.unwrap();
    }
    
    println!("ALL DONE");
}

// async fn my_bg_task(id: u64) {
    // let num: u64 = thread_rng().gen_range(10..200);
    // println!("START id: {} with {}ms", id, num);
    // sleep(Duration::from_millis(num)).await;
    // println!("STOP id: {}", id);
// }