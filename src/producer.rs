use std::time::Duration;

use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

pub async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..100)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            info!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

// async fn produce() {
//     let matches = App::new("producer example")
//         .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
//         .about("Simple command line producer")
//         .arg(
//             Arg::with_name("brokers")
//                 .short("b")
//                 .long("brokers")
//                 .help("Broker list in kafka format")
//                 .takes_value(true)
//                 .default_value("localhost:9092"),
//         )
//         .arg(
//             Arg::with_name("log-conf")
//                 .long("log-conf")
//                 .help("Configure the logging format (example: 'rdkafka=trace')")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("topic")
//                 .short("t")
//                 .long("topic")
//                 .help("Destination topic")
//                 .takes_value(true)
//                 .required(true),
//         )
//         .get_matches();

//     setup_logger(true, matches.value_of("log-conf"));

//     let (version_n, version_s) = get_rdkafka_version();
//     info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

//     let topic = matches.value_of("topic").unwrap();
//     let brokers = matches.value_of("brokers").unwrap();

//     produce(brokers, topic).await;
// }