# clipboard-stream
[![crates.io](https://img.shields.io/crates/v/clipboard-stream.svg)](https://crates.io/crates/clipboard-stream)
[![docs.rs](https://img.shields.io/docsrs/clipboard-stream/latest)](https://docs.rs/clipboard-stream)
![GitHub License](https://img.shields.io/github/license/nakaryo716/clipboard-stream)

Async stream of clipboard change events.
Provides real-time clipboard monitoring through an async Stream interface.

The main part of this crate is ClipboardStream. This struct implements Stream Trait.

## Example
The following example shows how to receive clipboard items:
```rust
use clipboard_stream::{Body, ClipboardEventListener, MimeType};
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[tokio::main]
async fn main() {
    let event_listener = ClipboardEventListener::spawn();
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
```

## Body types
In this library, we can handle thses body types.
- text
- image

## Runtime
Internally, this crate spawns a small dedicated OS thread to listen for clipboard events.
The API itself is Future-based and does not depend on any specific async runtime, so it works with tokio, smol, or any runtime compatible with futures.

## Platforms
- macOS

Currently supported on **macOS only**. Windows support is planned for a future release.

## License
clipboard-stream is provided under the MIT license.See [LICENSE](LICENSE)
