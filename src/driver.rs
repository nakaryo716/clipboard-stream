use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

use crate::{
    body::Body,
    buffer::BufferSender,
    sys::{ClipBoardSys, OSXSys},
};

/// An event driver that monitors clipboard updates and notify
#[derive(Debug)]
pub(crate) struct Driver {
    flag: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Driver {
    pub(crate) fn new(mut buffer_handle: BufferSender) -> Self {
        let flag = Arc::new(AtomicBool::new(false));

        let flag_cl = flag.clone();
        let handle = std::thread::spawn(move || {
            let mut sys = OSXSys;
            let mut last_count = None;
            while !flag_cl.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(200));
                let change_count = match sys.get_change_count() {
                    Ok(c) => c,
                    Err(e) => {
                        continue;
                    }
                };

                if Some(change_count) == last_count {
                    continue;
                }

                last_count = Some(change_count);
                match sys.get_item() {
                    Ok(v) => {
                        let data = Ok(Body::Utf8(v));
                        buffer_handle.utf8_tx.try_send(data).unwrap();
                    }
                    Err(e) => {}
                }
            }
        });

        Driver {
            flag,
            handle: Some(handle),
        }
    }
}
