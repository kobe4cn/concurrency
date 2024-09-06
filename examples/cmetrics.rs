use std::{thread, time::Duration};

use concurrency::CmapMetrics;
use rand::Rng;

const N: usize = 4;
const M: usize = 8;
fn main() -> anyhow::Result<()> {
    let metrics = CmapMetrics::new();
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

fn task_worker<T>(idx: usize, metrics: CmapMetrics<T>, value: T) -> anyhow::Result<()>
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + Default + Send + 'static + Sync,
{
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

fn request_worker<T>(metrics: CmapMetrics<T>, value: T) -> anyhow::Result<()>
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + Default + Send + 'static + Sync,
{
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
