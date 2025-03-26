use backon::ExponentialBuilder;
use backon::Retryable;
use bytes::Bytes;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ProgressStyle;
use reqwest;
//use scraper;
use std::collections::HashMap;
use std::env::args;
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
    let mut join_set: JoinSet<String> = JoinSet::new();
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

    println!("begin: \"https://music.lliiiill.com/playlist/{begin}\",");
    println!("end: \"https://music.lliiiill.com/playlist/{end}\",");

    for id in begin..end {
        while join_set.len() >= max_concurrent {
            let res = join_set.join_next().await.unwrap();
            match res {
                Ok(s) => println!("{s}"),
                Err(err) => eprintln!("Join Error: {err:#?}"),
            }
        }
        let filed = filed.clone();
        let author = author.clone();
        let client_clone = client.clone();
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
                    .text()
                    .await?;
                Ok::<String, reqwest::Error>(bytes)
            };
            match req
                .retry(
                    ExponentialBuilder::default()
                        .with_jitter()
                        .with_factor(3.0)
                        .with_min_delay(Duration::from_secs(10))
                        .with_max_times(5),
                )
                .await
            {
                Ok(bytes) => bytes,
                Err(e) => format!("Err pid {id} {e:#?}"),
            }
        });
    }

    eprintln!("{:#?}, {}", bar.duration(), bar.per_sec());
    bar.finish();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(s) => println!("{s}"),
            Err(err) => eprintln!("Join Error: {err:#?}"),
        }
    }
}

pub fn check_bytes_sequence(haystack: Bytes, needle1: Bytes, needle2: Bytes) -> bool {
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
