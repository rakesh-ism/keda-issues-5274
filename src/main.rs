mod consumer;
mod example_utils;
mod producer;

use crate::example_utils::setup_logger;
use consumer::consume_and_print;
use log::info;
use producer::produce;
use rdkafka::util::get_rdkafka_version;
use tokio::join;
use std::env;

#[allow(dead_code)]
struct Config {
    brokers: String,
    pod_type: String,
}

impl Config {
    fn prepare() -> Result<Config, &'static str> {
        let brokers = env::var("BROKERS").map_or("localhost:9092".to_string(), |x| x);
        let pod_type = env::var("POD_TYPE").unwrap();
        Ok(Config {
            brokers,
            pod_type,
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    setup_logger(true, Some("main"));
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let config = Config::prepare().unwrap();
    if config.pod_type == "C"{
        consume_loop(&config.brokers, "group1").await;
    }else if config.pod_type == "P"{
        produce_loop(&config.brokers).await;
    }
    else{
        let produce = produce_loop(&config.brokers);
        let consume = consume_loop(&config.brokers, "group1");
        join!(produce, consume);    
    }
}

async fn produce_loop(brokers: &str) {
    produce(brokers, "errors").await;
    loop {
        produce(brokers, "warnings").await;
    }
}
async fn consume_loop(brokers: &str, group_id: &str) {
    let topics: [&str; 2] = ["errors", "warnings"];
    println!("wating for one minute in consume_loop");
    tokio::time::sleep(std::time::Duration::from_secs(600)).await; //wait for one minute
    println!("One minute over in consume_loop");
    consume_and_print(brokers, group_id, &topics).await;
}
