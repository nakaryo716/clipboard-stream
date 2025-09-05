use std::{
    sync::{
        atomic::{AtomicBool, Ordering}, Arc, Mutex
    },  thread::JoinHandle, time::Duration
};

use crate::{
    body::Kind,
    buffer::BufferSender,
    sys::{ClipBoardSys, OSXSys},
    waker::WakersMap,
};

/// An event driver that monitors clipboard updates and notify
#[derive(Debug)]
pub(crate) struct Driver {
    flag: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Driver {
    pub(crate) fn new(mut wakers: Arc<Mutex<WakersMap>>, mut buffer_handle: BufferSender) -> Self {
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
                        buffer_handle.utf8_tx.try_send(v).unwrap();
                        wakers.lock().unwrap().get_waker(Kind::Utf8).unwrap().wake_by_ref();
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
