use backon::ConstantBuilder;
use backon::Retryable;
use bytes::Bytes;
use reqwest;
use scraper;
use std::env::args;
use std::sync::Arc;
use std::time::Duration;
// use std::future::Future;
// use std::time::Duration;
use tokio;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
	let filed = Bytes::from_static(b"");
	let author = Bytes::from_static(b"");
    let mut arguments = args();
    let max_concurrent = arguments.nth(1).unwrap().parse::<usize>().unwrap();
    let begin = arguments.next().unwrap().parse::<u64>().unwrap();
    let end = arguments.next().unwrap().parse::<u64>().unwrap();
    let mut join_set: JoinSet<Result<Option<u64>, reqwest::Error>> = JoinSet::new();
    let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
    let client = reqwest::Client::new();

    for id in begin..end {
        // while join_set.len() >= max_concurrent {
            // join_set.join_next().await.unwrap().unwrap();
        // }
        let filed = filed.clone();
        let author = author.clone();
        let client_clone = client.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_set.spawn(async move {
            let req = move || client_clone
                .get(format!("https://music.163.com/playlist?id={}", id))
                .send();
            let res = req
                .retry(ConstantBuilder::default().with_delay(Duration::from_millis(0)))
                .await?
                .bytes()
                .await
                .unwrap();
            drop(permit);
            if check_bytes_sequence(&res, filed, author) {
            	return Ok(Some(id));
            }
            Ok(None)
        });
    }

    println!("DONE SPAWNING");

	let mut output = Vec::new();
	while let Some(res) = join_set.join_next().await{
        match res {
            Ok(Ok(Some(id))) => output.push(id),
            Ok(Ok(None)) => (),
            Ok(Err(e)) => eprintln!("Reqwest错误: {}", e),
            Err(err) => eprintln!("Join错误: {}", err),
        }
    }

    println!("ALL DONE");
}

pub fn check_bytes_sequence(haystack: &Bytes, needle1: &Bytes, needle2: &Bytes) -> bool {
    // 将Bytes转换为字节切片，因为Bytes实现了Deref<Target = [u8]>
    // let haystack = &**haystack;
    // 查找第一个子序列的位置
    if let Some(pos) = find_subsequence(haystack, needle1) {
        // 检查剩余部分是否以第二个子序列开头
        let remaining = &haystack[pos + needle1.len()..];
        remaining.starts_with(needle2)
    } else {
        false
    }
}

/// 辅助函数：在字节切片中查找子序列的位置。
fn find_subsequence(haystack: &Bytes, needle: &Bytes) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}




            // let select = scraper::Selector::parse("div.user > span.name > a").unwrap();
            // let html = scraper::Html::parse_document(&res);
            // if let Some(name) = html.select(&select).next() {
                // if name.value().name() == "PurionPurion" {
                    // println!("{:?}", id);
                // }
            // }