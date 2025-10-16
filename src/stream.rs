use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Stream, channel::mpsc::Receiver};

use crate::body::{Body, BodySendersDropHandle};

/// Asynchronous stream for fetching clipboard item.
///
/// When the clipboard is updated, the [`ClipboardStream`] polls for the yields the new data.
///
/// # Example
/// ```
/// # use clipboard_stream::{Body, ClipboardStream};
/// # use futures::stream::StreamExt;
/// # async fn stream(mut stream: ClipboardStream) {
/// // stream: ClipboardStream
/// while let Some(body) = stream.next().await {
///     if let Body::Utf8String(text) = body {
///         println!("{}", text);
///     }
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct ClipboardStream {
    pub(crate) id: StreamId,
    pub(crate) body_rx: Pin<Box<Receiver<Body>>>,
    pub(crate) drop_handle: BodySendersDropHandle,
}

impl ClipboardStream {
    pub fn id(&self) -> &StreamId {
        &self.id
    }
}

impl Stream for ClipboardStream {
    type Item = Body;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.body_rx.as_mut().poll_next(cx)
    }
}

impl Drop for ClipboardStream {
    fn drop(&mut self) {
        self.body_rx.close();
        // drain messages inner channel
        loop {
            match self.body_rx.try_next() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(_) => continue,
            }
        }

        // remove Sender from HashMap
        self.drop_handle.drop(&self.id);
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct StreamId(pub(crate) usize);
