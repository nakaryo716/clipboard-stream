use std::sync::{Arc, Mutex};

use futures::channel::mpsc;

use crate::{
    ClipboardStream, Error,
    body::{BodySenders, BodySendersDropHandle, Kind},
    driver::Driver,
};

pub struct ClipboadEventListener {
    driver: Option<Driver>,
    body_senders: Arc<Mutex<BodySenders>>,
}

impl ClipboadEventListener {
    pub fn spawn() -> Self {
        let body_senders = Arc::new(Mutex::new(BodySenders::new()));

        let driver = Driver::new(body_senders.clone());
        ClipboadEventListener {
            driver: Some(driver),
            body_senders,
        }
    }

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
