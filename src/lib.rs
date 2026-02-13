//! Async stream of clipboard change events.
//!
//! Provides real-time clipboard monitoring through an async [`Stream`] interface.
//!
//! The main part of this crate is [`ClipboardStream`].
//! This struct implements [`Stream`].
//!
//! # Example
//! The following example shows how to receive clipboard items:
//!
//! ```no_run
//! use clipboard_stream::{Body, MimeType, ClipboardEventListener};
//! use futures::StreamExt;
//! use tokio::{io::AsyncWriteExt, fs::File};
//!
//! use std::sync::{atomic::{AtomicU32, Ordering}, Arc};
//!
//! #[tokio::main]
//! async fn main() {
//!     let event_listener = ClipboardEventListener::spawn(32);
//!     let mut stream = event_listener.new_stream();
//!     let count = Arc::new(AtomicU32::new(0));
//!
//!     while let Some(content) = stream.next().await {
//!         match content {
//!             // print text when text is copied
//!             Body::Utf8String(text) => {
//!             println!("got string: {}", text);
//!         }
//!             // create png image file when png image data is copied
//!             Body::Image {
//!                 mime: mime_type,
//!                 data: v,
//!         } => {
//!             println!("got: {:?}", mime_type);
//!             let cc = count.clone();
//!             tokio::task::spawn(async move {
//!                 if mime_type == MimeType::ImagePng {
//!                     let num = cc.fetch_add(1, Ordering::SeqCst);
//!                     let file_name = format!("clip-img-{}.png", num);
//!
//!                     let mut file = File::create(file_name).await.unwrap();
//!                     file.write_all(v.as_ref()).await.unwrap();
//!                 }
//!             });
//!         }
//!         }
//!     }
//! }
//! ```
//!
//! # Body types
//! In this library, we can handle thses body types.
//! - text
//! - image
//!
//! # Runtime
//! Internally, this crate spawns a small dedicated OS thread to listen for clipboard events.
//! The API itself is `Future`-based and does not depend on any specific async runtime,
//! so it works with [`tokio`](https://docs.rs/tokio), [`smol`](https://docs.rs/smol), or any runtime compatible with
//! [`futures`](https://docs.rs/futures).
//!
//! # Platforms
//! - macOS
//!
//! Currently supported on **macOS only**. Windows support is planned for a future release.
//!
//! [`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
//! [`ClipboardStream`]: crate::stream::ClipboardStream
mod body;
mod driver;
mod event_listener;
mod stream;
mod sys;

pub use crate::body::{Body, MimeType};
pub use crate::event_listener::ClipboardEventListener;
pub use crate::stream::ClipboardStream;
