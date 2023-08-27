use async_trait::async_trait;
use reqwest;
use serde_json;
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait SourceTrait {
    async fn run(&self);
}

pub struct Source {
    name: String,
    tx: Sender<serde_json::Value>,
}

impl Source {
    pub fn new(name: String, tx: Sender<serde_json::Value>) -> Source {
        Source { name, tx }
    }
}
#[async_trait]
impl SourceTrait for Source {
    async fn run(&self) {
        println!("{}: run", self.name);
        let mut counter = 0;
        loop {
            let json = serde_json::json!({
                "name": self.name,
                "counter": counter,
            });
            println!("Sending: {}", json);
            let err = self.tx.send(json).await;
            match err {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            };
            counter += 1;
            sleep(Duration::from_secs(1)).await;
        }
    }
}

pub struct HttpSource {
    name: String,
    tx: Sender<serde_json::Value>,
    url: String,
}

impl HttpSource {
    pub fn new(name: String, tx: Sender<serde_json::Value>, url: String) -> HttpSource {
        HttpSource { name, tx, url }
    }
    async fn send_request(&self) -> serde_json::Value {
        let body = reqwest::get(&self.url).await.unwrap().text().await.unwrap();
        // convert body to json
        let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
        json_body
    }
}

#[async_trait]
impl SourceTrait for HttpSource {
    async fn run(&self) {
        println!("{}: run", self.name);
        loop {
            let res = self.send_request().await;
            let err = self.tx.send(res).await;
            match err {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            };
            sleep(Duration::from_secs(1)).await;
        }
    }
}
#[tokio::test]
async fn test_http_source() {
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Receiver;

    let (tx, _): (Sender<serde_json::Value>, Receiver<serde_json::Value>) = channel(100);
    let source = HttpSource::new(
        "source".to_string(),
        tx,
        "http://localhost:8000/rust/events".to_string(),
    );
    source.send_request().await;
}

#[tokio::test]
async fn test_source() {
    use crate::sinks::Sink;
    use crate::sinks::SinkTrait;
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Receiver;

    let (tx, rx): (Sender<serde_json::Value>, Receiver<serde_json::Value>) = channel(100);
    let source = Source::new("source".to_string(), tx);
    let mut sink = Sink::new("sink".to_string(), rx);

    // tokio::spawn(async move {
    //     source.run().await;
    // });

    // tokio::spawn(async move {
    //     sink.run().await;
    // });
    // println!("test_source: done");
    tokio::join!(source.run(), sink.run());
}
