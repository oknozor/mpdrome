use mpdrome_macro::MpdResponse;

#[derive(MpdResponse)]
pub struct Artists {
    pub artist: Vec<String>,
}
