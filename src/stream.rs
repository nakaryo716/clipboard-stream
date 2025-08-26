use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Stream, ready};

use crate::{driver::Driver, sys::OSXSys};

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
    driver: Driver,
}

impl ClipboardStream {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        let sys = OSXSys;

        ClipboardStream {
            driver: Driver::new(sys),
        }
    }
}

impl Default for ClipboardStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for ClipboardStream {
    type Item = Result<String, crate::error::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(self.driver.poll_clipboard(cx)) {
            Ok(v) => Poll::Ready(Some(Ok(v))),
            Err(e) => Poll::Ready(Some(Err(e))),
        }
    }
}
