use rdkafka::client::Client;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::{ClientConfig, Message};
use tokio::time;
use crate::rhttp::temp_http::Temperature;

const SENSOR_URL: &str = "http://192.168.0.143";
const KAFKA_ADVERTISE_LISENERS: &str = "192.168.0.28:9092";

pub async fn repeat() {
    let mut timer = tokio::time::interval(time::Duration::from_secs(10));
    let producer = kafka_producer();
    let future = async move {
        println!("subscribing messages...");
        let consumer = kafka_consumer2();
        consumer.subscribe(&vec!["my_topic"]).expect("subscribe topic failed");
        println!("subscribed");
       //for msg in consumer.iter() {
        consumer
        .iter()
        .filter_map(|result| match result {
            Ok(_) => {
                println!("message received.");
                result.ok()
            },
            Err(err) => {
                eprintln!("error consuming from message stream: {}", err);
                None
            },
        })        
        .for_each(|msg|{
            println!("receiving messages....");
            println!(
                "key: '{:?}', 
                payload: '{}', 
                topic: {}, 
                partition: {}, 
                offset: {}, 
                timestamp: {:?}",
                msg.key(), msg.payload_view::<str>().unwrap().unwrap(), msg.topic(), msg.partition(), msg.offset(), msg.timestamp()
            );
        });
    };
    tokio::spawn(future);    

    loop {
        timer.tick().await;
        let producer = producer.clone();
        tokio::spawn(async move {
            let _ = get_request(producer).await;
        });
    }
}

async fn get_request(producer: BaseProducer) -> Result<(), reqwest::Error> {
    let response = reqwest::get(SENSOR_URL).await?; //ESP-52F261
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);
    let temp: Temperature = serde_json::from_str(&body).unwrap();
    
    //publish_message(body, producer);
    publish_temperature(temp,producer);
    println!("published message to kafka");
    Ok(())
}

fn kafka_producer() -> BaseProducer {
    ClientConfig::new()
    .set("bootstrap.servers", KAFKA_ADVERTISE_LISENERS)
    .create()
    .expect("Invalid producer config")
}


fn kafka_client(f: &dyn Fn(&mut ClientConfig) -> &mut ClientConfig) -> BaseConsumer {
    f(ClientConfig::new().set("bootstrap.servers", KAFKA_ADVERTISE_LISENERS))
        .create()
        .expect("Invalid client config")
}
fn kafka_consumer2() -> BaseConsumer {
    kafka_client(&|ccf: &mut ClientConfig|{
        ccf.set("group.id", "ubuntu2");
        ccf
    })
}
fn kafka_consumer() -> BaseConsumer {
    ClientConfig::new()
    .set("bootstrap.servers", KAFKA_ADVERTISE_LISENERS)
    .set("group.id", "ubuntu2")
    .create()
    .expect("Invalid consumer config")
}

fn publish_message(json: String, producer: BaseProducer) {
    println!("publishing sensor data....");
    let record: BaseRecord<'_, (), std::string::String> = 
        BaseRecord::to("my_topic")
            .payload(&json);
    producer.send(record).expect("failed to send message")
}

fn publish_temperature(temp_data: Temperature, producer: BaseProducer) {
    println!("publishing sensor data....");
    let binding = serde_json::to_string_pretty(&temp_data).expect("convert to Json failed");
    let record: BaseRecord<'_, (), std::string::String> = 
        BaseRecord::to("my_topic")
            .payload(&binding);
    producer.send(record).expect("failed to send message")
}