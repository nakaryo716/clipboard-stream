use clipboard_stream::{Body, ClipboardEventListener, MimeType};
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[tokio::main]
async fn main() {
    let event_listener = ClipboardEventListener::spawn(32);
    let mut stream = event_listener.new_stream();
    let count = Arc::new(AtomicU32::new(0));

    while let Some(content) = stream.next().await {
        match content {
            // print text when text is copied
            Body::Utf8String(text) => {
                println!("got string: {}", text);
            }
            // create png image file when png image data is copied
            Body::Image {
                mime: mime_type,
                data: v,
            } => {
                println!("got: {:?}", mime_type);
                let cc = count.clone();
                // create png image file
                tokio::task::spawn(async move {
                    if mime_type == MimeType::ImagePng {
                        let num = cc.fetch_add(1, Ordering::SeqCst);
                        let file_name = format!("clip-img-{}.png", num);

                        let mut file = File::create(file_name).await.unwrap();
                        file.write_all(v.as_ref()).await.unwrap();
                    }
                });
            }
        }
    }
}
