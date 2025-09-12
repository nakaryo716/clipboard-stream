use clipboard_stream::{Body, ClipboadEventListener, Kind};
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut event_listener = ClipboadEventListener::spawn();
    let mut stream = event_listener.new_stream(Kind::Utf8String, 32).unwrap();

    while let Some(content) = stream.next().await {
        match content {
            Ok(v) => {
                match v {
                    Body::Utf8String(v) => println!("got string: {}", v),
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
