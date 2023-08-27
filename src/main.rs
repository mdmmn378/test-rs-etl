mod bridges;
mod mongo_client;
mod sinks;
mod sources;
use serde_json;
use sinks::SinkTrait;
use sources::SourceTrait;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

async fn bridge(source: &mut dyn SourceTrait, sink: &mut dyn SinkTrait) {
    tokio::join!(source.run(), sink.run());
}

fn get_channel() -> (Sender<serde_json::Value>, Receiver<serde_json::Value>) {
    channel(100)
}

async fn dummy_to_http() {
    let (tx, rx) = get_channel();
    let mut source = sources::Source::new("source".to_string(), tx);
    let mut sink = sinks::Sink::new("sink".to_string(), rx);
    bridge(&mut source, &mut sink).await;
}

async fn basic() {
    let (tx, rx) = get_channel();
    let mut mongo_source = sources::HttpSource::new(
        "mongo_source".to_string(),
        tx,
        "http://localhost:8000/rust/event".to_string(),
    );
    let mut sink = sinks::MongoSink::new(
        "mongo_sink".to_string(),
        rx,
        "mongodb://localhost:27017".to_string(),
    );
    bridge(&mut mongo_source, &mut sink).await;
}

#[tokio::main]
async fn main() {
    let p1 = dummy_to_http();
    let p2 = dummy_to_http();
    let p3 = basic();
    let p4 = basic();
    tokio::join!(p1, p2, p3, p4);
}
