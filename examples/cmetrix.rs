use anyhow::Result;
use concurrency::CmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrix = CmapMetrics::new();

    println!("{}", metrix);

    // start N workers and M requests
    for idx in 0..N {
        task_worker(idx, metrix.clone())?; // Arc::clone(&metrix)
    }
    for _ in 0..M {
        request_worker(metrix.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrix);
    }
}

fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            // do long term stuff
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            // do long term stuff
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
