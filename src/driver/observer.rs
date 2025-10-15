use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time,
};

use crate::{
    body::{Body, BodySenders},
    sys::OSXSys,
};

/// A trait observing clipboard change event and send data to receiver([`ClipboardStream`])
pub(super) trait Observer {
    fn observe(&self, body_senders: Arc<BodySenders>);
}

/// Observer for MacOS
pub(super) struct OSXObserver {
    stop: Arc<AtomicBool>,
    sys: OSXSys,
}

impl OSXObserver {
    pub(super) fn new(stop: Arc<AtomicBool>) -> Self {
        OSXObserver {
            stop,
            sys: OSXSys::new(),
        }
    }
}

impl Observer for OSXObserver {
    fn observe(&self, body_senders: Arc<BodySenders>) {
        let mut last_count = self.sys.get_change_count();

        while !self.stop.load(Ordering::Relaxed) {
            std::thread::sleep(time::Duration::from_millis(200));
            let change_count = self.sys.get_change_count();

            if change_count == last_count {
                continue;
            }
            last_count = change_count;

            if let Ok(item) = self.sys.get_item() {
                body_senders.send_all(Body::Utf8String(item));
            }
        }
    }
}
