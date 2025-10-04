use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time,
};

use crate::{
    Kind,
    body::{Body, BodySenders},
    sys::OSXSys,
};

/// A trait observing clipboard change event and send data to receiver([`ClipboardStream`])
pub(super) trait Observer {
    fn observe(&self, body_senders: Arc<Mutex<BodySenders>>);
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
    fn observe(&self, body_senders: Arc<Mutex<BodySenders>>) {
        let mut last_count = None;

        while !self.stop.load(Ordering::Relaxed) {
            std::thread::sleep(time::Duration::from_millis(200));
            let change_count = self.sys.get_change_count();

            if Some(change_count) == last_count {
                continue;
            }
            last_count = Some(change_count);

            match self.sys.get_item() {
                Ok(item) => {
                    let mut gurad = body_senders.lock().unwrap();
                    let body = Ok(Body::Utf8String(item));
                    if let Err(e) = gurad.try_send_if_some(body, &Kind::Utf8String) {
                        eprintln!("{}", e);
                    }
                }
                Err(_) => {
                    let mut gurad = body_senders.lock().unwrap();
                    if let Err(e) =
                        gurad.try_send_if_some(Err(crate::error::Error::GetItem), &Kind::Utf8String)
                    {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}
