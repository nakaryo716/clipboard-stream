/// Various kind of clipboard items.
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
