use std::fs::File;
use std::io::BufReader;

pub mod zk;
pub mod message;

fn main() {
    // zk::client::build_zk_client();
    let broker = "192.168.5.251:9092".to_owned();
    let topic = "test".to_owned();
    let group = "my-group".to_owned();

    if let Err(e) = message::receiver::receive(broker, topic, group) {
        println!("Failed consuming messages: {}", e);
    }
}
