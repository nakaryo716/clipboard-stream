use futures::channel::mpsc::{self, Receiver};

use crate::{Body, ClipboardStream, driver::Driver};

/// Clipboard event change listener.
///
/// Listen for clipboard change events and notifies [`ClipboardStream`].
pub struct ClipboardEventListener {
    driver: Driver,
    rx: Receiver<Body>,
}

impl ClipboardEventListener {
    /// Creates a new [`ClipboardEventListener`] that monitors clipboard changes in a dedicated OS thread.
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel(32);

        ClipboardEventListener {
            driver: Driver::new(tx),
            rx,
        }
    }

    /// Creates a [`ClipboardStream`] for receiving clipboard change items as [`Body`].
    /// [`Body`]: crate::Body
    pub fn new_stream(self) -> ClipboardStream {
        ClipboardStream {
            body_rx: Box::pin(self.rx),
            driver: self.driver,
        }
    }
}
