use std::thread::sleep;
use std::time;
use futures::future::join_all;
use tokio::task;
use std::io::Write;
use log::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn slowwly(delay_ms: u64) -> reqwest::Url {
    let url = "http://www.baidu.com";
    let dely = time::Duration::from_millis(delay_ms);
    sleep(dely);
    reqwest::Url::parse(&url).unwrap()
}

fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    // Let's spend as much time as we can and count them in two passes
    let ones = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt.iter().fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
}

// get_and_analyze 会同时处理 io 密集型和 CPU 密集型任务
async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    let response = reqwest::get(slowwly(1000)).await?;
    info!("Dataset {}", n);

    let txt = response.text().await?;

    // We send our analysis work to a thread where there is no runtime running
    // so we don't block the runtime by analyzing the data
    // 在线程池中执行 CPU 密集型任务
    let res = task::spawn_blocking(move || analyze(&txt)).await?;
    info!("Processed {}", n);
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env().format(move |buf, rec| {
        let t = start.elapsed().as_secs_f32();
        writeln!(buf, "{:.03} [{}] - {}", t, rec.level(),rec.args())
    }).init();

    let mut futures = vec![];
    for i in 1..=10 {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    info!("Ratio of ones/zeros: {:.02}",total_ones as f64 / total_zeros as f64);
    Ok(())
}
