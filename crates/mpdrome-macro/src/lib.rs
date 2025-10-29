#[cfg(feature = "derive")]
pub use mpdrome_derive::*;

use std::io::{Result, Write};

pub trait ToMpdResponse: Send + Sync + 'static {
    fn write_response<W: Write>(&self, w: &mut W) -> Result<()>;
}
