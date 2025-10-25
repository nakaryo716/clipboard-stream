use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures::channel::mpsc::Sender;

use crate::stream::StreamId;

/// Various kind of clipboard items.
///
/// The supporeted body is depend on OS. Check platform tab you use.
#[derive(Debug, Clone)]
pub enum Body {
    /// UTF-8 encoded String.
    Utf8String(String),
    /// Image type. It consist of [`MimeType`] and [`Vec<u8>`].
    #[cfg(target_os = "macos")]
    Image { mime: MimeType, data: Vec<u8> },
}

/// Indicates the media type of the [`Body`] variant.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MimeType {
    ImagePng,
}

#[derive(Debug)]
pub(crate) struct BodySenders {
    senders: Mutex<HashMap<StreamId, Sender<Body>>>,
}

impl BodySenders {
    pub(crate) fn new() -> Self {
        BodySenders {
            senders: Mutex::default(),
        }
    }

    /// Register Sender that was specified [`StreamId`].
    pub(crate) fn register(&self, id: StreamId, tx: Sender<Body>) {
        let mut gurad = self.senders.lock().unwrap();
        gurad.insert(id, tx);
    }

    /// Close channel and unregister sender that was specified [`StreamId`]
    fn unregister(&self, id: &StreamId) {
        let mut gurad = self.senders.lock().unwrap();
        gurad.remove(id);
    }

    pub(crate) fn send_all(&self, body: Body) {
        let mut senders = self.senders.lock().unwrap();

        for sender in senders.values_mut() {
            let body_c = body.clone();
            if let Err(e) = sender.try_send(body_c) {
                eprintln!("{}", e);
            }
        }
    }
}

/// Handler for Cleaning up buffer(channel).
///
/// Close channel and unregister a specified [`StreamId`] of sender.
#[derive(Debug)]
pub(crate) struct BodySendersDropHandle(Arc<BodySenders>);

impl BodySendersDropHandle {
    pub(crate) fn new(senders: Arc<BodySenders>) -> Self {
        BodySendersDropHandle(senders)
    }

    pub(crate) fn drop(&self, id: &StreamId) {
        self.0.unregister(id);
    }
}
