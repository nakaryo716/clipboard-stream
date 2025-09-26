/// System call wrapper module
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeString};

use crate::error::Error;

#[cfg(target_os = "macos")]
pub(crate) struct OSXSys;

#[cfg(target_os = "macos")]
impl OSXSys {
    pub(crate) fn get_change_count(&self) -> Result<i64, Error> {
        let inner = unsafe { NSPasteboard::generalPasteboard() };
        let count = unsafe { inner.changeCount() };
        Ok(count as i64)
    }

    pub(crate) fn get_item(&mut self) -> Result<String, Error> {
        let item = unsafe {
            let inner = NSPasteboard::generalPasteboard();
            inner.dataForType(NSPasteboardTypeString)
        };

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
