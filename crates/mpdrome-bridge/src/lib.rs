pub use color_eyre::eyre::Error as BridgeError;
pub use mpdrome_mpd_protocol::filter::{Expression, TagName, TagOp};

pub trait MpdBridge {
    fn list_artist(&self) -> color_eyre::Result<Vec<String>>;
    fn get_artist(&self) -> color_eyre::Result<Artist>;
    fn search(&self, exp: Vec<Expression>) -> color_eyre::Result<Artist>;
}

pub struct Artist {
    name: String,
}
