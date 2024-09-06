use std::{thread, time::Duration};

use concurrency::AmapMetrics;
use rand::Rng;

const N: usize = 4;
const M: usize = 8;
fn main() -> anyhow::Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "call.thread.worker.2",
        "call.thread.worker.3",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
        "req.page.5",
        "req.page.6",
        "req.page.7",
        "req.page.8",
        "req.page.9",
    ]);
    println!("{:?}", metrics);
    for idx in 0..N {
        task_worker(idx, metrics.clone(), 1)?;
    }
    for _ in 0..M {
        request_worker(metrics.clone(), 1)?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        // let map = metrics;
        // let mut count = 0;
        // loop {
        //     count += 1;
        //     if metrics.data.is_empty() {
        //         break;
        //     } else {
        //         println!("Old M count:{}, {:?}", count, map);
        //         // thread::sleep(Duration::from_secs(2));
        //         if count > 10 {
        //             break;
        //         }
        //     }
        // }
        println!("New M {}", metrics);
    }
    // println!("{:?}", metrics.snapshot());
}

fn task_worker(idx: usize, metrics: AmapMetrics, value: i64) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.increase(format!("call.thread.worker.{}", idx).as_str(), value)?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: AmapMetrics, value: i64) -> anyhow::Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..10);
            metrics
                .increase(format!("req.page.{}", page).as_str(), value)
                .map_err(|e| {
                    eprintln!("error: {}", e);
                })
                .ok();
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}
