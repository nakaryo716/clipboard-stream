use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::Stream;

use crate::{
    body::{Body, Kind},
    buffer::BufferReceiver,
};

/// Asynchronous stream for fetching clipboard item.
///
/// When the clipboard is updated, the [`ClipboardStream`] polls for the yields the new data.  
/// The return type is `Result<String>`. Other data formats are not **yet** supported.  
///
/// # Example
/// ```no_run
/// use clipboard_stream::ClipboardStream;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let mut stream = ClipboardStream::new();
///
///     while let Some(item) = stream.next().await {
///         if let Ok(v) = item {
///             println!("{}", v);
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct ClipboardStream {
    data: Arc<Mutex<BufferReceiver>>,
    kind: Kind,
}

impl ClipboardStream {
    pub(crate) fn new(data: Arc<Mutex<BufferReceiver>>, kind: Kind) -> Self {
        ClipboardStream { data, kind }
    }
}

impl Stream for ClipboardStream {
    type Item = Result<Body, crate::error::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.data.lock().unwrap().next(&self.kind, cx)
    }
}
