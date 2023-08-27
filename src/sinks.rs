use async_trait::async_trait;
use serde_json;
use tokio::sync::mpsc::Receiver;

use crate::mongo_client::{convert_serde_to_document, insert_document};
#[async_trait]
pub trait SinkTrait {
    async fn run(&mut self);
}

pub struct Sink {
    name: String,
    rx: Receiver<serde_json::Value>,
}

impl Sink {
    pub fn new(name: String, rx: Receiver<serde_json::Value>) -> Sink {
        Sink { name, rx }
    }
}

#[async_trait]
impl SinkTrait for Sink {
    async fn run(&mut self) {
        println!("{}: run", self.name);
        loop {
            let msg = (self.rx).recv().await.unwrap();
            println!("Received {}", msg);
        }
    }
}

pub struct MongoSink {
    name: String,
    rx: Receiver<serde_json::Value>,
    url: String,
}

impl MongoSink {
    pub fn new(name: String, rx: Receiver<serde_json::Value>, url: String) -> MongoSink {
        MongoSink { name, rx, url }
    }
    async fn push_to_mongo(&self, msg: serde_json::Value) {
        println!("{}: push_to_mongo", self.name);
        println!("{}: {}", self.name, msg);
        let client = crate::mongo_client::get_connection(self.url.as_str()).await;
        let collection_name = "mongo_sink";
        let msg = convert_serde_to_document(msg);
        insert_document(&client, collection_name, msg)
            .await
            .unwrap();
    }
}

#[async_trait]
impl SinkTrait for MongoSink {
    async fn run(&mut self) {
        println!("{}: run", self.name);
        loop {
            let msg = (self.rx).recv().await.unwrap();
            self.push_to_mongo(msg).await;
        }
    }
}

#[tokio::test]
async fn test_mongo_sink() {
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Sender;
    let (_, rx): (Sender<serde_json::Value>, Receiver<serde_json::Value>) = channel(100);
    let mut sink = MongoSink::new(
        "sink".to_string(),
        rx,
        "mongodb://localhost:27017".to_string(),
    );
    sink.run().await;
}

#[tokio::test]
async fn test_sink() {
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Sender;
    let (_, rx): (Sender<serde_json::Value>, Receiver<serde_json::Value>) = channel(100);
    let mut sink = Sink::new("sink".to_string(), rx);
    sink.run().await;
}
