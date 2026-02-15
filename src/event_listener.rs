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
    ///
    /// # Buffer size
    /// Clipboard Item is  bounded, this EventListener provides backpressure.
    /// `buffer` specify this size.  
    /// Actualy, the buffer size is equal to `buffer + 1`.
    /// # Behavior when the buffer is full
    /// Once buffer is full, new body that is copied will be **ignored**.  
    /// Dose **not** block, dose **not** retry, and dose **not** signal an error.
    pub fn spawn(buffer: usize) -> Self {
        let (tx, rx) = mpsc::channel(buffer);

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
