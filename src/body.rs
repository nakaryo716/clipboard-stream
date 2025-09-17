use std::sync::{Arc, Mutex};

use futures::channel::mpsc::{Sender, TrySendError};

use crate::{Error, Msg};

/// Various kind of clipboard items.
#[derive(Debug, Clone)]
pub enum Body {
    /// UTF-8 encoded String.
    Utf8String(String),
}

/// Specifies the kind of [`ClipboardStream`].
///
/// [`ClipboardStream`]: crate::ClipboardStream
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// UTF-8 encoded String.
    Utf8String,
}

#[derive(Debug)]
pub(crate) struct BodySenders {
    utf8_string_tx: Option<Sender<Msg>>,
}

impl BodySenders {
    pub(crate) fn new() -> Self {
        BodySenders {
            utf8_string_tx: None,
        }
    }

    /// Register Sender that was specified kind. If, sender already exists, return Err(Error::StreamAlreadyExists).
    pub(crate) fn register(&mut self, tx: Sender<Msg>, kind: &Kind) -> Result<(), Error> {
        if self.is_some(kind) {
            return Err(Error::StreamAlreadyExists);
        }

        match kind {
            Kind::Utf8String => self.utf8_string_tx = Some(tx),
        }

        Ok(())
    }

    fn is_some(&self, kind: &Kind) -> bool {
        match kind {
            Kind::Utf8String => self.utf8_string_tx.is_some(),
        }
    }

    /// Close channel and unregister sender that was specified kind
    fn unregister(&mut self, kind: &Kind) {
        match kind {
            Kind::Utf8String => {
                if let Some(ref mut v) = self.utf8_string_tx {
                    if !v.is_closed() {
                        v.close_channel();
                    }
                    self.utf8_string_tx = None;
                }
            }
        }
    }

    /// When specified kind's Sender is Some, send message.
    pub(crate) fn try_send_if_some(
        &mut self,
        msg: Msg,
        kind: &Kind,
    ) -> Result<(), TrySendError<Msg>> {
        match kind {
            Kind::Utf8String => try_send(&mut self.utf8_string_tx, msg)?,
        }
        Ok(())
    }

    /// Send a message to the sender if available.
    /// Errors are logged but do not stop other sends.
    pub(crate) fn send_all_if_some(&mut self, msg: Msg) {
        send_ignore_err(&mut self.utf8_string_tx, msg);
    }
}

#[inline]
fn try_send(tx: &mut Option<Sender<Msg>>, msg: Msg) -> Result<(), TrySendError<Msg>> {
    if let Some(v) = tx {
        v.try_send(msg)?
    }
    Ok(())
}

#[inline]
fn send_ignore_err(tx: &mut Option<Sender<Msg>>, msg: Msg) {
    if let Some(v) = tx
        && let Err(e) = v.try_send(msg)
    {
        eprintln!("{}", e);
    }
}

/// Handler for Cleaning up buffer(channel).
///
/// Close channel and unregister a specified kined of sender.
#[derive(Debug)]
pub(crate) struct BodySendersDropHandle(Arc<Mutex<BodySenders>>);

impl BodySendersDropHandle {
    pub(crate) fn new(senders: Arc<Mutex<BodySenders>>) -> Self {
        BodySendersDropHandle(senders)
    }

    pub(crate) fn drop(&mut self, kind: &Kind) {
        let mut gurad = self.0.lock().unwrap();
        gurad.unregister(kind);
    }
}
