use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
    time,
};

use crate::{
    Kind,
    body::{Body, BodySenders},
    sys::OSXSys,
};

/// An event driver that monitors clipboard updates and notify
#[derive(Debug)]
pub(crate) struct Driver {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Driver {
    /// Construct [`Driver`] and spawn a thread for monitoring clipboard events
    pub(crate) fn new(body_senders: Arc<Mutex<BodySenders>>) -> Self {
        let stop = Arc::new(AtomicBool::new(false));

        let stop_cl = stop.clone();

        let handle = std::thread::spawn(move || {
            let mut last_count = None;

            let mut sys = OSXSys;
            while !stop_cl.load(Ordering::Relaxed) {
                std::thread::sleep(time::Duration::from_millis(200));
                let change_count = match sys.get_change_count() {
                    Ok(c) => c,
                    Err(_) => {
                        let mut gurad = body_senders.lock().unwrap();
                        gurad.send_all_if_some(Err(crate::error::Error::GetItem));
                        continue;
                    }
                };

                if Some(change_count) == last_count {
                    continue;
                }
                last_count = Some(change_count);

                match sys.get_item() {
                    Ok(item) => {
                        let mut gurad = body_senders.lock().unwrap();
                        let body = Ok(Body::Utf8String(item));
                        if let Err(e) = gurad.try_send_if_some(body, &Kind::Utf8String) {
                            eprintln!("{}", e);
                        }
                    }
                    Err(_) => {
                        let mut gurad = body_senders.lock().unwrap();
                        if let Err(e) = gurad
                            .try_send_if_some(Err(crate::error::Error::GetItem), &Kind::Utf8String)
                        {
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        });

        Driver {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
