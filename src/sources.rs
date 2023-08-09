use async_trait::async_trait;

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
