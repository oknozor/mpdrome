use mpdrome_macro::MpdResponse;

#[derive(Debug, MpdResponse)]
pub struct Song {
    pub file: String,
    pub last_modified: String,
    pub added: String,
    pub format: String,
    pub album: String,
    pub album_artist: String,
    pub artist: String,
    pub date: String,
    pub disc: u32,
    pub title: String,
    pub original_date: String,
    pub genres: Vec<String>,
    pub musicbrainz_album_id: String,
    pub musicbrainz_artist_id: String,
    pub musicbrainz_album_artist_id: String,
    pub musicbrainz_release_group_id: String,
    pub musicbrainz_track_id: String,
    pub track_number: u32,
    pub label: String,
    pub musicbrainz_release_track_id: String,
    pub time: u32,
    pub duration: f32,
}
