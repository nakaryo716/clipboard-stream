use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Stream, channel::mpsc::Receiver};

use crate::{
    Msg,
    body::{Body, BodySendersDropHandle, Kind},
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
    pub(crate) body_rx: Pin<Box<Receiver<Msg>>>,
    pub(crate) kind: Kind,
    pub(crate) drop_handle: BodySendersDropHandle,
}

impl Stream for ClipboardStream {
    type Item = Result<Body, crate::error::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.body_rx.as_mut().poll_next(cx)
    }
}
