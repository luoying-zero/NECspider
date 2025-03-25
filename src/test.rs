use backon::ConstantBuilder;
use backon::Retryable;
use bytes::Bytes;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ProgressStyle;
use reqwest;
//use scraper;
use std::collections::HashMap;
use std::env::args;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let filed = Bytes::from_static(b"\"userId\":");
    let author = Bytes::from_static(b"62696289,");
    let mut arguments = args();
    let max_concurrent = arguments.nth(1).unwrap().parse::<usize>().unwrap();
    let begin = arguments.next().unwrap().parse::<u64>().unwrap();
    let end = arguments.next().unwrap().parse::<u64>().unwrap();
    let mut join_set: JoinSet<Result<Option<u64>, String>> = JoinSet::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let bar = ProgressBar::new(end - begin);
    bar.set_style(
        ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} | {elapsed}/{eta} | {per_sec}")
            .unwrap(),
    );
    bar.set_draw_target(ProgressDrawTarget::stderr_with_hz(1));

    let client = reqwest::Client::builder()
        .http1_only()
        .timeout(Duration::from_secs(20))
        .pool_idle_timeout(Duration::from_secs(72))
        .pool_max_idle_per_host(usize::MAX)
        .gzip(true)
        .brotli(true)
        .zstd(true)
        .deflate(true)
        .build()
        .unwrap();

    tokio::spawn(async move {
        loop {
            while let Some(res) = join_set.join_next().await {
                match res {
                    Ok(Ok(Some(id))) => println!("\"https://music.lliiiill.com/playlist/{id}\","),
                    Ok(Ok(None)) => (),
                    Ok(Err(e)) => eprintln!("{e}"),
                    Err(err) => eprintln!("Join Error: {err:#?}"),
                }
            }
            if let Ok(_) = rx.try_recv() {
                break;
            }
        }
    });

    for id in begin..end {
        let filed = filed.clone();
        let author = author.clone();
        let client_clone = client.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        // if (id - begin) % ((end - begin) / 100) == 0 {
        // bar.inc((end - begin) / 100);
        // }
        bar.inc(1);
        join_set.spawn(async move {
            let mut params = HashMap::new();
            params.insert("id", format!("{id}"));
            let req = || async {
                let bytes = client_clone
                    .post("http://music.163.com/api/v6/playlist/detail")
                    .form(&params)
                    .send()
                    .await?
                    .bytes()
                    .await?;
                Ok::<bytes::Bytes, reqwest::Error>(bytes)
            };
            let res = match req
                .retry(
                    ConstantBuilder::default()
                        .with_delay(Duration::from_millis(0))
                        .with_max_times(5),
                )
                .await
            {
                Ok(bytes) => bytes,
                Err(e) => return Err(format!("Err pid {id} {e:#?}"))
            };
            drop(permit);
            match check_bytes_sequence(res, filed, author).await {
                true => Ok(Some(id)),
                false => Ok(None),
            }
        });
    }

    tx.send(());
    eprintln!("{:#?}, {}", bar.duration(), bar.per_sec());
    bar.finish();
}

pub async fn check_bytes_sequence(haystack: Bytes, needle1: Bytes, needle2: Bytes) -> bool {
    if let Some(pos) = find_subsequence(&haystack, &needle1) {
        let remaining = &haystack[pos + needle1.len()..];
        remaining.starts_with(&needle2)
    } else {
        false
    }
}

fn find_subsequence(haystack: &Bytes, needle: &Bytes) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// let select = scraper::Selector::parse("div.user > span.name > a").unwrap();
// let html = scraper::Html::parse_document(&res);
// if let Some(name) = html.select(&select).next() {
// if name.value().name() == "PurionPurion" {
// println!("{:?}", id);
// }
// }
