use anyhow::{Error, anyhow};
use backon;
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
    let filed = Bytes::from("\"userId\":");
    let author = Bytes::from("62696289,");
    let mut arguments = args();
    let max_concurrent = arguments.nth(1).unwrap().parse::<usize>().unwrap();
    let begin = arguments.next().unwrap().parse::<u64>().unwrap();
    let end = arguments.next().unwrap().parse::<u64>().unwrap();
    let mut join_set: JoinSet<Result<Option<u64>, String>> = JoinSet::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let bar = ProgressBar::new(end - begin);
    bar.set_style(
        ProgressStyle::with_template(
            "{bar:40} {pos:>7}/{len:7} | {elapsed_precise}/{eta_precise} | {per_sec}",
        )
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
        let filed = filed.clone();
        let author = author.clone();
        let client_clone = client.clone();
        let permit = tokio::select! {
            biased;
            Ok(permit) = semaphore.clone().acquire_owned() => permit,
            _ = async {
                while let Some(res) = join_set.join_next().await {
                    match res {
                        Ok(Ok(Some(id))) => println!("\"https://music.lliiiill.com/playlist/{id}\","),
                        Ok(Ok(None)) => (),
                        Ok(Err(e)) => eprintln!("{e}"),
                        Err(err) => eprintln!("Join Error: {err:#?}"),
                    }
                }
                false
            } => semaphore.clone().acquire_owned().await.unwrap(),
        };
        // if (id - begin) % ((end - begin) / 100) == 0 {
        // bar.inc((end - begin) / 100);
        // }
        bar.inc(1);
        join_set.spawn(async move {
            let mut params = HashMap::new();
            params.insert("id", format!("{id}"));
            params.insert("n", format!("0"));
            params.insert("s",format!("0"));
            let req = || async {
                let bytes = client_clone
                    .post("http://interface.music.163.com/api/v6/playlist/detail")
                    .form(&params)
                    .send()
                    .await?
                    .error_for_status()?
                    .chunk()
                    .await?
                    .unwrap();
                match check_bytes_sequence(
                    bytes.clone(),
                    Bytes::from("\"code\":"),
                    Bytes::from("406"),
                ) {
                    true => Err(anyhow!("Request is limited")),
                    false => Ok::<bytes::Bytes, Error>(bytes),
                }
            };
            let res = match req
                .retry(
                    backon::ExponentialBuilder::default()
                        .with_jitter()
                        .with_factor(2.0)
                        .with_max_times(5),
                )
                .await
            {
                Ok(bytes) => bytes,
                Err(e) => return Err(format!("Err pid {id} {e:#?}")),
            };
            drop(permit);
            match check_bytes_sequence(res, filed, author) {
                true => Ok(Some(id)),
                _ => Ok(None),
            }
        });
    }

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(Ok(Some(id))) => println!("\"https://music.lliiiill.com/playlist/{id}\","),
            Ok(Ok(None)) => (),
            Ok(Err(e)) => eprintln!("{e}"),
            Err(err) => eprintln!("Join Error: {err:#?}"),
        }
    }

    eprintln!("{:#?}, {}", bar.elapsed(), bar.per_sec());
}

fn check_bytes_sequence(haystack: Bytes, needle1: Bytes, needle2: Bytes) -> bool {
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
