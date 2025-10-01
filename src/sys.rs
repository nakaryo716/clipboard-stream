#[cfg(target_os = "macos")]
use objc2::rc::Retained;
/// System call wrapper module
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeString};

use crate::error::Error;

#[cfg(target_os = "macos")]
pub(crate) struct OSXSys {
    inner: Retained<NSPasteboard>,
}

#[cfg(target_os = "macos")]
impl OSXSys {
    pub(crate) fn new() -> Self {
        let inner = unsafe { NSPasteboard::generalPasteboard() };
        OSXSys { inner }
    }

    pub(crate) fn get_change_count(&self) -> Result<i64, Error> {
        let count = unsafe { self.inner.changeCount() };
        Ok(count as i64)
    }

    pub(crate) fn get_item(&self) -> Result<String, Error> {
        let item = unsafe { self.inner.dataForType(NSPasteboardTypeString) };

        match item {
            Some(v) => {
                let data = v.to_vec();
                let text = String::from_utf8(data).map_err(Error::FromUtf8Error)?;
                Ok(text)
            }
            None => Err(Error::GetItem),
        }
    }
}
