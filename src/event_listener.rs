use std::sync::{Arc, Mutex};

use futures::channel::mpsc;

use crate::{
    ClipboardStream, Error,
    body::{BodySenders, BodySendersDropHandle, Kind},
    driver::Driver,
};

/// Clipboard event change listener.
///
/// Listen for clipboard change events and notifies [`ClipboardStream`].
pub struct ClipboadEventListener {
    driver: Option<Driver>,
    body_senders: Arc<Mutex<BodySenders>>,
}

impl ClipboadEventListener {
    /// Creates a new [`ClipboadEventListener`] that monitors clipboard changes in a dedicated OS thread.
    pub fn spawn() -> Self {
        let body_senders = Arc::new(Mutex::new(BodySenders::new()));

        let driver = Driver::new(body_senders.clone());
        ClipboadEventListener {
            driver: Some(driver),
            body_senders,
        }
    }
    /// Creates a [`ClipboardSream`] for receiving clipboard change items as [`Body`].
    /// If a stream for the same [`Kind`] already exists, returns [`Error::StreamAlreadyExists`].
    ///
    /// # Buffer size
    /// This method takes a buffer size. Items are buffered when not received immediately.
    /// The actual buffer capacity is `buf_size + 2`, where the extra `2` accounts for the
    /// number of internal senders used by the library.
    ///
    /// # Example
    /// ```
    /// # use clipboard_stream::{Kind, ClipboadEventListener, ClipboardStream};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut event_listener = ClipboadEventListener::spawn();
    ///
    ///     let buf_size = 32;
    ///     let stream = event_listener.new_stream(Kind::Utf8String, buf_size)?;
    /// #   Ok(())
    /// # }
    /// ```
    /// [`Body`]: crate::Body
    pub fn new_stream(&mut self, kind: Kind, buffer: usize) -> Result<ClipboardStream, Error> {
        let (tx, rx) = mpsc::channel(buffer);
        self.body_senders.lock().unwrap().register(tx, &kind)?;
        let drop_handle = BodySendersDropHandle::new(self.body_senders.clone());

        let stream = ClipboardStream {
            body_rx: Box::pin(rx),
            kind,
            drop_handle,
        };
        Ok(stream)
    }
}

impl Default for ClipboadEventListener {
    fn default() -> Self {
        ClipboadEventListener::spawn()
    }
}

impl Drop for ClipboadEventListener {
    fn drop(&mut self) {
        drop(self.driver.take())
    }
}
