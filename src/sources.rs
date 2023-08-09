use async_trait::async_trait;

use tokio::time::{sleep, Duration};

use tokio::sync::mpsc::Sender;

#[async_trait]
trait SourceTrait {
    async fn run(&self);
}

struct Source {
    name: String,
    tx: Sender<String>,
}

impl Source {
    fn new(name: String, tx: Sender<String>) -> Source {
        Source { name, tx }
    }
}
#[async_trait]
impl SourceTrait for Source {
    async fn run(&self) {
        println!("{}: run", self.name);
        let mut counter = 0;
        loop {
            let msg = format!("{}: {}", self.name, counter);
            println!("{}", msg);
            self.tx.send(msg).await.unwrap();
            counter += 1;
            sleep(Duration::from_secs(1)).await;
        }
    }
}
#[tokio::test]
async fn test_source() {
    use tokio::sync::mpsc::channel;
    use tokio::sync::mpsc::Receiver;

    let (tx, mut rx): (Sender<String>, Receiver<String>) = channel(100);
    let source = Source::new("source".to_string(), tx);
    source.run().await;
    loop {
        let msg = rx.recv().await.unwrap();
        println!("main: {}", msg);
    }
}
