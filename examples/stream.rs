use clipboard_stream::{Body, ClipboardEventListener};
use futures::StreamExt;

use std::{fs::File, io::Write};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut event_listener = ClipboardEventListener::spawn();
    let mut stream = event_listener.new_stream(32);

    while let Some(content) = stream.next().await {
        match content {
            Body::Utf8String(text) => {
                println!("got string: {}", text);
            }
            #[cfg(target_os = "macos")]
            Body::PNG(v) => {
                println!("got PNG");
                tokio::task::spawn_blocking(move || {
                    let mut file = File::create("clip-img.png").unwrap();
                    file.write_all(v.as_ref()).unwrap();
                });
            }
        }
    }
}
