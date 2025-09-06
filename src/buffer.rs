use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{
    Stream,
    channel::mpsc::{self, Receiver, Sender},
};

use crate::body::{Body, Kind};

#[derive(Debug)]
pub(crate) struct BufferReceiver {
    utf8_buf: Pin<Box<Receiver<Result<Body, crate::error::Error>>>>,
    img_buf: Pin<Box<Receiver<Result<Body, crate::error::Error>>>>,
}

impl BufferReceiver {
    pub(crate) fn next(
        &mut self,
        kind: &Kind,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Body, crate::error::Error>>> {
        match kind {
            Kind::Utf8 => self.utf8_buf.as_mut().poll_next(cx),
            Kind::Img => self.img_buf.as_mut().poll_next(cx),
        }
    }
}

pub(crate) struct BufferSender {
    pub utf8_tx: Sender<Result<Body, crate::error::Error>>,
    pub img_tx: Sender<Result<Body, crate::error::Error>>,
}

pub(crate) fn create_buffer() -> (BufferReceiver, BufferSender) {
    let (utf8_tx, utf8_rx) = mpsc::channel(128);
    let (img_tx, img_rx) = mpsc::channel(128);

    let buffer = BufferReceiver {
        utf8_buf: Box::pin(utf8_rx),
        img_buf: Box::pin(img_rx),
    };

    let buffer_handle = BufferSender { utf8_tx, img_tx };

    (buffer, buffer_handle)
}
