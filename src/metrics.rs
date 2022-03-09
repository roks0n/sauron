use statsd::client::Client;
use lazy_static::lazy_static;
use crate::config::{get_config, Config};

lazy_static! {
    static ref CONFIG: Config = get_config();
    pub static ref CLIENT: Client = init_new_client(CONFIG.statsd_host.to_string(), &CONFIG.statsd_prefix);
}

pub fn init_new_client(host: String, prefix: &str) -> Client {
    Client::new(host, prefix).unwrap()
}

#[allow(dead_code)]
pub fn get_client() -> &'static CLIENT {
    &CLIENT
}

#[allow(dead_code)]
pub fn incr(metrics: &str) {
    CLIENT.incr(metrics);
}

#[allow(dead_code)]
pub fn decr(metrics: &str) {
    CLIENT.decr(metrics);
}

#[allow(dead_code)]
pub fn gauge(metrics: &str, value: f64) {
    CLIENT.gauge(metrics, value);
}

#[allow(dead_code)]
pub fn timer(metrics: &str, value: f64) {
    CLIENT.timer(metrics, value);
}

#[allow(dead_code)]
pub fn time<F, R>(metrics: &str, callable: F) -> R
where F: FnOnce() -> R,
{
    CLIENT.time(metrics, callable)
}