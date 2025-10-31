use mpdrome_macro::MpdResponse;
use strum::VariantNames;

use crate::protocol::requests::Request;

#[derive(MpdResponse)]
pub struct Commands {
    command: Vec<&'static str>,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            command: Request::VARIANTS.to_vec(),
        }
    }
}
