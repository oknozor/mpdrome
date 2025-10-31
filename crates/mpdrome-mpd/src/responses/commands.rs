use mpdrome_macro::MpdResponse;
use mpdrome_mpd_protocol::request::Request;
use strum::VariantNames;

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
