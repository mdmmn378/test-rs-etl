mod bridges;
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

#[tokio::main]
async fn main() {
    let p1 = dummy_to_http();
    let p2 = dummy_to_http();
    tokio::join!(p1, p2);
}
