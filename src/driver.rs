use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    thread::JoinHandle,
    time,
};

use futures::task::AtomicWaker;

use crate::sys::ClipBoardSys;

/// An event driver that monitors clipboard updates and notify
#[derive(Debug)]
pub(crate) struct Driver {
    queue: Arc<Mutex<VecDeque<Result<String, crate::error::Error>>>>,
    waker: Arc<AtomicWaker>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Driver {
    /// Construct [`Driver`] and spawn a thread for monitoring clipboard events
    pub(crate) fn new<S>(sys: S) -> Self
    where
        S: ClipBoardSys + Send + 'static,
    {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let waker = Arc::new(AtomicWaker::new());
        let stop = Arc::new(AtomicBool::new(false));

        let queue_cl = queue.clone();
        let stop_cl = stop.clone();
        let waker_cl = waker.clone();

        let handle = std::thread::spawn(move || {
            let mut last_count = None;

            let mut sys = sys;
            while !stop_cl.load(Ordering::Relaxed) {
                std::thread::sleep(time::Duration::from_millis(200));
                let change_count = match sys.get_change_count() {
                    Ok(c) => c,
                    Err(e) => {
                        let mut queue = queue_cl.lock().unwrap();
                        queue.push_back(Err(e));
                        waker_cl.wake();
                        continue;
                    }
                };

                if Some(change_count) == last_count {
                    continue;
                }
                last_count = Some(change_count);

                match sys.get_item() {
                    Ok(item) => {
                        let mut queue = queue_cl.lock().unwrap();
                        queue.push_back(Ok(item));
                        waker_cl.wake();
                    }
                    Err(e) => {
                        let mut queue = queue_cl.lock().unwrap();
                        queue.push_back(Err(e));
                        waker_cl.wake();
                    }
                }
            }
        });

        Driver {
            queue,
            waker,
            stop,
            handle: Some(handle),
        }
    }

    /// Poll clipboard item
    /// When clipboard is updated, will return Poll::Ready
    pub(crate) fn poll_clipboard(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<String, crate::error::Error>> {
        if let Some(v) = self.queue.lock().unwrap().pop_front() {
            return Poll::Ready(v);
        }

        self.waker.register(cx.waker());

        let mut queue = self.queue.lock().unwrap();
        let data = queue.pop_front();

        match data {
            Some(v) => Poll::Ready(v),
            None => Poll::Pending,
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
