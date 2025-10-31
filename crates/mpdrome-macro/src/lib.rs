#[cfg(feature = "derive")]
pub use mpdrome_derive::*;

use std::io::{Result, Write};

pub trait ToMpdResponse: Send + Sync + 'static {
    fn write_content<W: Write>(&self, w: &mut W) -> Result<()>;

    fn write_response<W: Write>(&self, w: &mut W) -> Result<()> {
        self.write_content(w)?;
        self.finalize(w)
    }

    fn finalize<W: Write>(&self, w: &mut W) -> Result<()> {
        write!(w, "OK\n")?;
        Ok(())
    }
}

impl<T: Send + Sync + ToMpdResponse + 'static> ToMpdResponse for Vec<T> {
    fn write_content<W: Write>(&self, w: &mut W) -> Result<()> {
        for item in self {
            item.write_content(w)?;
        }
        Ok(())
    }
}
