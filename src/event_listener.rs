use std::sync::{Arc, Mutex};

use crate::{
    ClipboardStream,
    body::Kind,
    buffer::{BufferReceiver, create_buffer},
    driver::Driver,
};

pub struct ClipboardEventListener {
    buffer: Arc<Mutex<BufferReceiver>>,
    driver: Driver,
}

impl ClipboardEventListener {
    pub fn new() -> Self {
        let (buf_rx, buf_tx) = create_buffer();

        let driver = Driver::new(buf_tx);

        ClipboardEventListener {
            buffer: Arc::new(Mutex::new(buf_rx)),
            driver,
        }
    }

    pub fn new_stream(&self, kind: Kind) -> Result<ClipboardStream, crate::error::Error> {
        let stream = ClipboardStream::new(self.buffer.clone(), kind);
        Ok(stream)
    }
}
