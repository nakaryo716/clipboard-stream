//! Async stream of clipboard change events.
//!
//! Provides real-time clipboard monitoring through an async [`Stream`] interface.
//!
//! The main part of this crate is [`ClipboardSream`].  
//! This struct implements [`Stream`].
//!
//! # Example
//! The following example shows how to receive clipboard items:
//!
//! ```no_run
//! use clipboard_stream::ClipboardStream;
//! use futures::stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut stream = ClipboardStream::new();
//!
//!     while let Some(item) = stream.next().await {
//!         if let Ok(v) = item {
//!             println!("{}", v);
//!         }
//!     }
//! }
//! ```
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
//! [`ClipboardSream`]: crate::stream::ClipboardStream
mod body;
mod driver;
mod error;
mod stream;
mod sys;

pub use crate::error::Error;
pub use crate::stream::ClipboardStream;
