#[derive(Debug, Clone)]
pub enum Body {
    Utf8String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Utf8String,
}
