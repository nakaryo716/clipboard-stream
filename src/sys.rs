//! System call wrapper module

#[cfg(target_os = "macos")]
use objc2::rc::Retained;
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSPasteboard, NSPasteboardTypePNG, NSPasteboardTypeString};

use crate::{Body, MimeType};

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

    pub(crate) fn get_body(&self) -> Option<Body> {
        // get Utf8String body type
        let string_data = unsafe { self.inner.dataForType(NSPasteboardTypeString) };

        // construct String if data is some
        if let Some(v) = string_data {
            let bytes = v.to_vec();
            // if String::from_utf8 is failed, we don't push.
            if let Ok(text) = String::from_utf8(bytes) {
                return Some(Body::Utf8String(text));
            }
        }

        // get PNG image data
        let png_data = unsafe { self.inner.dataForType(NSPasteboardTypePNG) };
        if let Some(v) = png_data {
            return Some(Body::Image {
                mime: MimeType::ImagePng,
                data: v.to_vec(),
            });
        }

        // when unhandled data type copied, return None
        None
    }
}
