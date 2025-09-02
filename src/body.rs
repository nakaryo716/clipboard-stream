#[derive(Debug)]
pub enum Body {
    Utf8(String),
    Img(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Utf8,
    Img,
}
