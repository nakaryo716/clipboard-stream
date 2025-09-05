use std::sync::{Arc, Mutex};

use crate::{body::Kind, buffer::{create_buffer, BufferReceiver}, driver::Driver, waker::{WakerHandle, WakersMap}, ClipboardStream};

pub struct ClipboardEventListener {
    buffer: Arc<Mutex<BufferReceiver>>, 
    driver: Driver,
    waker_handle: WakerHandle,
}

impl ClipboardEventListener {
    pub fn new() -> Self {
        let wakers = WakersMap::new();
        let waker_handle = WakerHandle::new(wakers.clone());

        let (buf_rx, buf_tx) = create_buffer();

        let driver = Driver::new(wakers, buf_tx);

        ClipboardEventListener {
            buffer: Arc::new(Mutex::new(buf_rx)),
            driver,
            waker_handle,
        }
    }

    pub fn new_stream(&self, kind: Kind) -> Result<ClipboardStream, crate::error::Error> {
        let stream = ClipboardStream::new(self.waker_handle.clone(), self.buffer.clone(), kind);
        Ok(stream)

    }
}
