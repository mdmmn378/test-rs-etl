use async_trait::async_trait;
use serde_json;
use tokio::sync::mpsc::Receiver;
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

#[tokio::test]
async fn test_sink() {
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Sender;
    let (_, rx): (Sender<serde_json::Value>, Receiver<serde_json::Value>) = channel(100);
    let mut sink = Sink::new("sink".to_string(), rx);
    sink.run().await;
}
