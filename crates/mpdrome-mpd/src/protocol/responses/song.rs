use mpdrome_macro::MpdResponse;

#[derive(Debug, MpdResponse)]
pub struct Song {
    pub file: String,
    #[mpd(rename = "Last-Modified")]
    pub last_modified: String,
    #[mpd(case = "PascalCase")]
    pub added: String,
    #[mpd(case = "PascalCase")]
    pub format: String,
    #[mpd(case = "PascalCase")]
    pub album: String,
    #[mpd(case = "PascalCase")]
    pub album_artist: String,
    #[mpd(case = "PascalCase")]
    pub artist: String,
    #[mpd(case = "PascalCase")]
    pub date: String,
    #[mpd(case = "PascalCase")]
    pub disc: u32,
    #[mpd(case = "PascalCase")]
    pub title: String,
    #[mpd(case = "PascalCase")]
    pub original_date: String,
    #[mpd(case = "PascalCase")]
    pub genre: Vec<String>,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_albumid: String,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_artistid: String,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_albumartistid: String,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_releasegroupid: String,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_trackid: String,
    #[mpd(case = "PascalCase")]
    pub track: u32,
    #[mpd(case = "PascalCase")]
    pub label: String,
    #[mpd(case = "UPPER_SNAKE")]
    pub musicbrainz_releasetrackid: String,
    #[mpd(case = "PascalCase")]
    pub time: u32,
    pub duration: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpdrome_macro::ToMpdResponse;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_mpd_response() {
        let response = Song {
            file: "MG Ultra/Machine Girl - MG Ultra - 01 - Until I Die.flac".to_string(),
            last_modified: "2025-10-25T17:35:17Z".to_string(),
            added: "2025-10-29T13:52:14Z".to_string(),
            format: "44100:16:2".to_string(),
            album: "MG Ultra".to_string(),
            album_artist: "Machine Girl".to_string(),
            artist: "Machine Girl".to_string(),
            date: "2024-10-18".to_string(),
            disc: 1,
            title: "Until I Die".to_string(),
            original_date: "2024-10-18".to_string(),
            genre: vec![
                "Breakcore".to_string(),
                "Digital Hardcore".to_string(),
                "Drum And Bass".to_string(),
                "Hardcore Breaks".to_string(),
            ],
            musicbrainz_albumid: "9b878585-3a94-40c7-ad5b-387f440d587a".to_string(),
            musicbrainz_artistid: "c7df5ec5-380a-40f9-856f-28fb4a33d946".to_string(),
            musicbrainz_albumartistid: "c7df5ec5-380a-40f9-856f-28fb4a33d946".to_string(),
            musicbrainz_releasegroupid: "9086d743-fca1-40e4-a0a8-0c3847feb1a8".to_string(),
            musicbrainz_trackid: "e039e17a-2448-4fcf-8e60-68d7931400f9".to_string(),
            track: 1,
            label: "Future Classic".to_string(),
            musicbrainz_releasetrackid: "d385a468-f850-4a4f-b440-5bba07d2365b".to_string(),
            time: 247,
            duration: 246.933,
        };

        let mut buffer = Vec::new();
        response.write_response(&mut buffer).unwrap();
        let actual = String::from_utf8_lossy(&buffer);

        assert_eq!(
            actual,
            r#"file: MG Ultra/Machine Girl - MG Ultra - 01 - Until I Die.flac
Last-Modified: 2025-10-25T17:35:17Z
Added: 2025-10-29T13:52:14Z
Format: 44100:16:2
Album: MG Ultra
AlbumArtist: Machine Girl
Artist: Machine Girl
Date: 2024-10-18
Disc: 1
Title: Until I Die
OriginalDate: 2024-10-18
Genre: Breakcore
Genre: Digital Hardcore
Genre: Drum And Bass
Genre: Hardcore Breaks
MUSICBRAINZ_ALBUMID: 9b878585-3a94-40c7-ad5b-387f440d587a
MUSICBRAINZ_ARTISTID: c7df5ec5-380a-40f9-856f-28fb4a33d946
MUSICBRAINZ_ALBUMARTISTID: c7df5ec5-380a-40f9-856f-28fb4a33d946
MUSICBRAINZ_RELEASEGROUPID: 9086d743-fca1-40e4-a0a8-0c3847feb1a8
MUSICBRAINZ_TRACKID: e039e17a-2448-4fcf-8e60-68d7931400f9
Track: 1
Label: Future Classic
MUSICBRAINZ_RELEASETRACKID: d385a468-f850-4a4f-b440-5bba07d2365b
Time: 247
duration: 246.933
OK
"#
        );
    }
}
