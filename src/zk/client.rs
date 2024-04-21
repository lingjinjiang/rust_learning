
use std::time::Duration;

use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
pub fn build_zk_client()  {
    let zk = ZooKeeper::connect("192.168.5.251:2181", Duration::from_secs(15), LoggingWatcher).unwrap();
    let children = zk.get_children("/", true);
    match children {
        Ok(c) => {
            print!("{:?}", c)
        },
        Err(e)=>{
            print!("{:?}", e)
        }
    }
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
    }
}