use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrix = Metrics::new();

    // start N workers and M requests
    for idx in 0..N {
        task_worker(idx, metrix.clone()); // Arc::clone(&metrix)
    }
    for _ in 0..M {
        process_requests(metrix.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrix.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        // do long term stuff
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
}

fn process_requests(metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        // do long term stuff
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..256);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
