#[cfg(target_os = "macos")]
use objc2::rc::Retained;
/// System call wrapper module
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeString};

use crate::Body;

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

    pub(crate) fn get_change_count(&self) -> isize {
        unsafe { self.inner.changeCount() }
    }

    pub(crate) fn get_bodies(&self) -> Vec<Body> {
        // the reason of capacity size is number of kind 'Body'
        let mut bodies = Vec::with_capacity(1);

        // get Utf8String body type
        let string_data = unsafe { self.inner.dataForType(NSPasteboardTypeString) };

        // construct String if data is some
        if let Some(v) = string_data {
            let bytes = v.to_vec();
            // if String::from_utf8 is failed, we don't push.
            if let Some(text) = String::from_utf8(bytes).ok() {
                bodies.push(Body::Utf8String(text));
            }
        }

        bodies
    }
}
