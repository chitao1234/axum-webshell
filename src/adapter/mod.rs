mod local;
pub use local::*;
use tokio::io::{AsyncRead, AsyncWrite};

// TODO: Make it platform dependent
pub struct WinSize {
    row: u32,
    col: u32,
    xpixel: u32,
    ypixel: u32,
}

pub trait ResizablePty {
    type BorrowedR<'a>: AsyncRead where Self: 'a;
    type BorrowedW<'a>: AsyncWrite where Self: 'a;
    type OwnedR: AsyncRead;
    type OwnedW: AsyncWrite;
    type Error;

    fn resize(&self, size: WinSize) -> Result<(), Self::Error>;
    fn split(&mut self) -> (Self::BorrowedR<'_>, Self::BorrowedW<'_>);
    fn into_split(self) -> (Self::OwnedR, Self::OwnedW);
}

