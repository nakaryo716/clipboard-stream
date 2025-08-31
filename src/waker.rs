use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::Waker,
};

use crate::body::Kind;

#[derive(Debug)]
pub(crate) struct WakersMap {
    wakers: HashMap<Kind, Waker>,
}

impl WakersMap {
    pub(crate) fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(WakersMap {
            wakers: HashMap::new(),
        }))
    }
}

#[derive(Debug)]
pub(crate) struct WakerHandle(Arc<Mutex<WakersMap>>);

impl WakerHandle {
    pub(crate) fn new(wakers: Arc<Mutex<WakersMap>>) -> Self {
        WakerHandle(wakers)
    }

    pub(crate) fn register(&self, kind: Kind, waker: Waker) {
        let mut gurad = self.0.lock().unwrap();
        gurad.wakers.insert(kind, waker);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        body::Kind,
        waker::{WakerHandle, WakersMap},
    };
    use futures::task::noop_waker;

    #[test]
    fn test_wakers_handle() {
        let wakers_map = WakersMap::new();

        let handle_one = WakerHandle::new(wakers_map.clone());
        let handle_two = WakerHandle::new(wakers_map.clone());

        let waker = noop_waker();

        let waker_cl = waker.clone();
        let t1 = std::thread::spawn(move || {
            handle_one.register(Kind::Utf8, waker_cl);
        });

        let t2 = std::thread::spawn(move || {
            handle_two.register(Kind::Img, waker);
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let gurad = wakers_map.lock().unwrap();
        let utf8_waker = gurad.wakers.get(&Kind::Utf8);
        let img_waker = gurad.wakers.get(&Kind::Img);

        assert!(utf8_waker.is_some());
        assert!(img_waker.is_some());
    }
}
