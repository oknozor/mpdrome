pub use color_eyre::eyre::Error as BridgeError;

pub trait MpdBridge {
    fn list_artist(&self) -> color_eyre::Result<Vec<String>>;
    fn get_artist(&self) -> color_eyre::Result<Artist>;
}

pub struct Artist {
    name: String,
}
