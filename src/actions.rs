use serde_json;
use std::io::{Read, Write, Seek, SeekFrom};
use std::sync::{Mutex, Arc};
use tokio::time::{sleep, Duration};
use crate::storage::get_storage;
use crate::utils::is_valid_url;
use crate::config::{get_config_file, get_config};

pub fn add(target: &str, metric: &str) {
    let item = crate::monitor::Target {
        target: target.to_string(),
        metric_name: metric.to_string(),
    };
    
    let mut file = get_storage();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut targets: Vec<crate::monitor::Target> =  serde_json::from_str(&content).unwrap_or_default();

    if !is_valid_url(&item.target) {
        println!("\"{}\" is not a valid url", item.target);
        return
    }

    let exists = targets.iter().any(|i| i.target == item.target || i.metric_name == item.metric_name);
    if exists {
        println!("Item with this target or metric_name value already exists!");
        return
    }

    targets.push(item);

    let updated_data = serde_json::to_string(&targets).unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write(&updated_data.as_bytes()).unwrap();
    println!("{} added successfully.", target);
}

pub fn remove(target: &str) {
    let mut file = get_storage();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let targets: Vec<crate::monitor::Target> =  serde_json::from_str(&content).unwrap_or_default();

    let mut is_removed = false;
    let updated_targets: Vec<crate::monitor::Target> = targets.into_iter().filter(|i| {
        if i.target == target {
            is_removed = true;
            return false
        }
        true
    }).collect();

    if !is_removed {
        println!("\"{}\" does not exist!", target);
        return
    }

    let updated_data = serde_json::to_string(&updated_targets).unwrap();

    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write(&updated_data.as_bytes()).unwrap();
    println!("{} removed successfully.", target);
}

pub fn list() {
    let mut file = get_storage();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let items: Vec<crate::monitor::Target> =  serde_json::from_str(&content).unwrap_or_default();
    if items.len() == 0 {
        println!("No targets have been added yet");
        return
    }

    println!("Monitoring:");
    println!("---");
    for item in items.iter() {
        println!("Target: {}", item.target);
        println!("Metric: {}", item.metric_name);
        println!("---");
    }
}

pub async fn check() {
    let mut file = get_storage();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let items: Vec<crate::monitor::Target> =  serde_json::from_str(&content).unwrap_or_default();
    if items.len() == 0 {
        println!("No targets have been added yet");
        return
    }

    crate::monitor::run_check(items).await;
}

pub async fn watch() {
    println!("Watching...");
    let break_out = Arc::new(Mutex::new(false));
    let break_out_copy = break_out.clone();

    ctrlc::set_handler( move || {
        println!(" Stopping watcher...");
        let mut break_out = break_out_copy.lock().unwrap();
        *break_out = true;
    }).expect("Error occured when settings up a CTRL + C handler");

    loop {
        if *break_out.lock().unwrap() == true {
            break;
        }

        // check().await;
        sleep(Duration::from_secs(5)).await;
    }
    println!("Stopped watching");
}

fn get_value_from_stdin(label: String) -> String {
    print!("{label} ");
    std::io::stdout().flush().unwrap();

    let mut val = String::new();
    std::io::stdin().read_line(&mut val).unwrap();
    let val = val.trim();
    val.to_string()
}

pub fn setup() {
    println!("Setup config - Any already set values will be overriden.");

    let config_data = get_config();
    print!("\nCurrent settings:\n- Stasd Host: {}\n- Statsd Prefix: {}\n\n", config_data.statsd_host, config_data.statsd_prefix);

    let move_on = get_value_from_stdin("Do you want to continue? (y/N)".to_string());
    if move_on.to_lowercase() != String::from("y") {
        return;
    }

    let host = get_value_from_stdin("Statsd server (ip and port):".to_string());
    let prefix = get_value_from_stdin("Statsd prefix:".to_string());

    let config = crate::config::Config {
        statsd_host: host.to_string(),
        statsd_prefix: prefix.to_string(),
    };
    
    let toml_content = toml::to_string(&config).unwrap();
    let mut file = get_config_file();
    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write(toml_content.as_bytes()).unwrap();
    
    println!("\nConfiguration values set:");
    println!("- Statsd Host: {host}");
    println!("- Statsd Prefix: {prefix}");
}