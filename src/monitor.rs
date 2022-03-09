use reqwest;
use serde::{Deserialize, Serialize};
use futures::{stream, StreamExt};
use std::time::{Instant};

const CONCURRENT_REQUESTS: usize = 10;

#[derive(Debug)]
struct HealthCheck {
    metric_name: String,
    latency: f64,
    // status: reqwest::StatusCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub target: String,
    pub metric_name: String,
}

async fn healthcheck(t: Target) -> Result<HealthCheck, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let success: bool;
    
    println!("Checking {}", t.target);
    let res = reqwest::get(&t.target).await;
    if res.is_err() {
        panic!("Err {:?}", res);
    }

    let res = res.unwrap();
    if res.status() == reqwest::StatusCode::OK {
        success = true;
        // println!("Request succeeded");
    } else {
        // println!("Request failed with status code: {:?}", res.status());
        success = false;
    }

    // Sleep
    // let dur = Duration::new(3, 0);
    // tokio::time::sleep(dur).await;
    let duration: f64;
    if success == true {
        duration = start.elapsed().as_secs_f64();
    } else {
        duration = 0.0;
    }

    Ok(HealthCheck {
        metric_name: t.metric_name,
        latency: duration,
        // status: res.status(),
    })
}

async fn report_metrics(hc: HealthCheck) {
    let client = &crate::metrics::CLIENT;
    let metric = format!("{}.latency", hc.metric_name);
    client.gauge(&metric, hc.latency);
}

pub async fn run_check(targets: Vec<Target>) {
    let requests = stream::iter(targets).map(|t| async move {
        healthcheck(t).await
    }).buffer_unordered(CONCURRENT_REQUESTS);

    requests.for_each(|hc| async move {
        match hc {
            Ok(hc) => report_metrics(hc).await,
            Err(e) => println!("There was an error: {}", e),
        }
    }).await;
}