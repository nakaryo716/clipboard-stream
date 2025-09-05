use futures::channel::mpsc::{self, Receiver, Sender, TryRecvError};

#[derive(Debug)]
pub(crate) struct BufferReceiver {
    utf8_buf: Receiver<String>,
    img_buf: Receiver<Vec<u8>>,
}

impl BufferReceiver {
    pub(crate) fn next_utf8(&mut self) -> Result<Option<String>, TryRecvError> {
        self.utf8_buf.try_next()
    }

    pub(crate) fn next_img(&mut self) -> Result<Option<Vec<u8>>, TryRecvError> {
        self.img_buf.try_next()
    }
}

pub(crate) struct BufferSender {
    pub utf8_tx: Sender<String>,
    pub img_tx: Sender<Vec<u8>>,
}

pub(crate) fn create_buffer() -> (BufferReceiver, BufferSender) {
    let (utf8_tx, utf8_rx) = mpsc::channel(128);
    let (img_tx, img_rx) = mpsc::channel(128);

    let buffer = BufferReceiver {
        utf8_buf: utf8_rx,
        img_buf: img_rx,
    };

    let buffer_handle = BufferSender { utf8_tx, img_tx };

    (buffer, buffer_handle)
}
