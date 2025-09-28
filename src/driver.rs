use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
};

use crate::{body::BodySenders, driver::observer::Observer};

#[cfg(target_os = "macos")]
use crate::driver::observer::OSXObserver;

mod observer;

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

        #[cfg(target_os = "macos")]
        let stop_cl = stop.clone();
        #[cfg(target_os = "macos")]
        let observer = OSXObserver::new(stop_cl);

        // spawn OS thread
        // observe clipboard change event and send item
        let handle = std::thread::spawn(move || {
            observer.observe(body_senders);
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
