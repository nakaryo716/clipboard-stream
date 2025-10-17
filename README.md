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
use clipboard_stream::{Body, ClipboardEventListener};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    // Spawn a clipboard event listener
    let mut event_listener = ClipboardEventListener::spawn();
    // Create a ClipboardStream with buffer
    let mut stream = event_listener.new_stream(32);

    // Text is printed when the clipboard is updated(Copy operation)
    while let Some(content) = stream.next().await {
        match content {
            Body::Utf8String(text) => {
                println!("got string: {}", text);
            }
        }
    }
}
```

## Runtime
Internally, this crate spawns a small dedicated OS thread to listen for clipboard events.
The API itself is Future-based and does not depend on any specific async runtime, so it works with tokio, smol, or any runtime compatible with futures.

## Platforms
- macOS

Currently supported on **macOS only**. Windows support is planned for a future release.

## License
clipboard-stream is provided under the MIT license.See [LICENSE](LICENSE)
