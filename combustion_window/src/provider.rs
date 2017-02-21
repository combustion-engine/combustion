use ::error::WindowError;

pub trait WindowProvider {
    //TODO
}

pub trait WindowBuilder {
    type Provider: WindowProvider;
    type Raw;

    fn new() -> Self;

    fn from_raw(builder: Self::Raw) -> Self;
    fn into_raw(self) -> Self::Raw;

    fn with_raw<F>(self, F) -> Self where F: FnOnce(Self::Raw) -> Self::Raw;

    fn size(self, width: u32, height: u32) -> Self;
    fn title(self, title: &str) -> Self;
    fn build(self) -> Result<Self::Provider, WindowError>;
}