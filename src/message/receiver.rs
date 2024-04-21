use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;


pub fn receive(address: String, topic: String, group: String) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(vec![address])
    .with_topic(topic)
    .with_group(group)
    .with_fallback_offset(FetchOffset::Earliest)
    .with_offset_storage(GroupOffsetStorage::Kafka)
    .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!(
                    "{}:{}@{}: {:?}",
                    ms.topic(),
                    ms.partition(),
                    m.offset,
                    m.value
                );
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}
