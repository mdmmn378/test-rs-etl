use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{sleep, Duration};
#[async_trait]
trait SinkTrait {
    async fn run(&mut self);
}

struct Sink {
    name: String,
    rx: Receiver<String>,
}

impl Sink {
    fn new(name: String, rx: Receiver<String>) -> Sink {
        Sink { name, rx }
    }
}

#[async_trait]
impl SinkTrait for Sink {
    async fn run(&mut self) {
        println!("{}: run", self.name);
        loop {
            let msg = (self.rx).recv().await.unwrap();
            println!("Sink {}", msg);
        }
    }
}

#[tokio::test]
async fn test_sink() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel(100);
    let mut sink = Sink::new("sink".to_string(), rx);
    sink.run().await;
    loop {
        let msg = format!("main: {}", 1);
        println!("{}", msg);
        tx.send(msg).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}
