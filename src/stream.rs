use std::{
    pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll}
};

use futures::{ready, Stream};

use crate::{
    body::{Body, Kind},
    buffer::BufferReceiver,
    waker::WakerHandle,
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
    waker_handle: WakerHandle,
    data: Arc<Mutex<BufferReceiver>>,
    kind: Kind,
}

impl ClipboardStream {
    pub(crate) fn new(waker_handle: WakerHandle, data: Arc<Mutex<BufferReceiver>>, kind: Kind) -> Self {
        ClipboardStream {
            waker_handle,
            data,
            kind,
        }
    }
}

impl Stream for ClipboardStream {
    type Item = Result<Body, crate::error::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("polled");
        match self.kind {
            Kind::Utf8 => {
                self.waker_handle.register(self.kind.clone(),cx.waker().clone());
                if let Ok(Some(v)) = self.data.lock().unwrap().next_utf8() {
                    return Poll::Ready(Some(Ok(Body::Utf8(v))));
                }
                self.waker_handle
                    .register(self.kind.clone(), cx.waker().clone());
                Poll::Pending
            }
            Kind::Img => {
                if let Ok(Some(v)) = self.data.lock().unwrap().next_img() {
                    return Poll::Ready(Some(Ok(Body::Img(v))));
                }
                self.waker_handle
                    .register(self.kind.clone(), cx.waker().clone());
                Poll::Pending
            }
        }
    }
}
