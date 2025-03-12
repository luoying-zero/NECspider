use reqwest;
use scraper;
use std::future::Future;
use std::time::Duration;
use std::env::args;
use tokio;
//use tokio::time::{sleep, Duration};
use tokio::task::JoinSet;
//use rand;

#[tokio::main]
async fn main() {
    let arguments = args();
    let max_concurrent = arguments.nth(1).unwrap().parse::<i32>().unwrap();
    let begin = arguments.next().unwrap().parse::<i32>().unwrap();
    let end = arguments.next().unwrap().parse::<i32>().unwrap();
    //let ids: Vec<u64> = (1..=10).into_iter().collect();
    let mut join_set = JoinSet::new();

    for id in begin..end {
        while join_set.len() >= max_concurrent {
            join_set.join_next().await.unwrap().unwrap();
        }
        join_set.spawn(retry_on_err(
            move || findAuthor(id)
        ));
    }

    println!("DONE SPAWNING");

    while let Some(output) = join_set.join_next().await {
        output.unwrap();
    }

    println!("ALL DONE");
}

async fn findAuthor(id: i32) -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get(format!("https://music.163.com/playlist?id={}", id))
        .send()
        .await?
        .text()
        .await
        .unwrap();
    let select = scraper::Selector::parse("div.user > span.name > a").unwrap();
    let html = scraper::Html::parse_document(&res);
    let name = html.select(&select).next().unwrap();
    if name.value().name() == "PurionPurion" {
        println!("{:?}", id);
    }

    Ok(())
}

async fn retry_on_err<T, E, F, Fut>(f: F)
where
    E: std::fmt::Debug
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let maxtry = 5;
    //let now = Instant::now();
    let backoff = Duration::from_millis(500);
    //let factor = 1.5;
    //let limit = Duration::from_secs(60 * 2);
    //let warn = Duration::from_secs(60 * 60);
    //let mut rng = rand::rngs::OsRng;
    //let mut jitter = || rng.gen_range(Duration::ZERO..backoff);

    for i in 0..=maxtry {
        match f().await {
            Ok(_) => {
                break;
            }
            Err(e) => {
                //let elapsed = now.elapsed();
                //if elapsed > warn {
                //let elapsed = humantime::format_duration(elapsed);
                //error!(%elapsed);
                //}
                //let retry_in = backoff.mul_f32(factor).min(limit) + jitter();
                if i == maxtry{
                    println!("{:?}", e);
                }
                tokio::time::sleep(backoff).await;
            }
        }
    }
}
// async fn my_bg_task(id: u64) {
// let num: u64 = thread_rng().gen_range(10..200);
// println!("START id: {} with {}ms", id, num);
// sleep(Duration::from_millis(num)).await;
// println!("STOP id: {}", id);
// }
