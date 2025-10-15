use clipboard_stream::{Body, ClipboardEventListener};
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut event_listener = ClipboardEventListener::spawn();
    let mut stream = event_listener.new_stream(32);

    while let Some(content) = stream.next().await {
        match content {
            Body::Utf8String(text) => {
                println!("got string: {}", text);
            }
        }
    }
}
