mod consumer;
mod example_utils;
mod producer;

use crate::example_utils::setup_logger;
use consumer::consume_and_print;
use log::info;
use producer::produce;
use rdkafka::util::get_rdkafka_version;
use std::env;
use tokio::join;

struct Config {
    brokers: String,
    pod_type: String,
    wait_sec: u32,
}

impl Config {
    fn prepare() -> Result<Config, &'static str> {
        let brokers = env::var("BROKERS").map_or("localhost:9092".to_string(), |x| x);
        let pod_type = env::var("POD_TYPE").unwrap();
        let wait_sec: u32 = env::var("WAIT_SEC").unwrap().parse().unwrap();
        Ok(Config {
            brokers,
            pod_type,
            wait_sec,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    setup_logger(true, Some("main"));
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let config = Config::prepare().unwrap();
    if config.pod_type == "C" {
        consume_loop(&config.brokers, "group1", config.wait_sec).await;
    } else if config.pod_type == "P" {
        produce_loop(&config.brokers).await;
    } else {
        let produce = produce_loop(&config.brokers);
        let consume = consume_loop(&config.brokers, "group1", config.wait_sec);
        join!(produce, consume);
    }
}

async fn produce_loop(brokers: &str) {
    produce(brokers, "errors").await;
    loop {
        produce(brokers, "warnings").await;
    }
}
async fn consume_loop(brokers: &str, group_id: &str, wait_time: u32) {
    let topics: [&str; 2] = ["errors", "warnings"];
    println!("wating for one minute in consume_loop");
    if wait_time > 0 {
        tokio::time::sleep(std::time::Duration::from_secs(600)).await;
    }
    println!("One minute over in consume_loop");
    consume_and_print(brokers, group_id, &topics).await;
}
