use std::sync::Arc;

use mpdrome_bridge::{Expression, MpdBridge, TagName, TagOp};
use mpdrome_mpd_model::song::Song;
use submarine::{auth::AuthBuilder, data::{AlbumWithSongsId3, ArtistWithAlbumsId3, Child}, Client};

#[derive(Clone)]
pub struct SubSonicClient {
    client: Arc<Client>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl SubSonicClient {
    pub fn new(user: &str, password: &str, url: &str) -> Self {
        let client_name = format!("mpdrome-{user}");
        let auth = AuthBuilder::new(user, "v0.16.1")
            .client_name(&client_name)
            .hashed(password);
        let client = Client::new(url, auth);
        let rt = tokio::runtime::Runtime::new().unwrap();

        Self {
            client: Arc::new(client),
            rt: Arc::new(rt),
        }
    }
}

impl MpdBridge for SubSonicClient {
    fn list_artist(&self) -> color_eyre::Result<Vec<String>> {
        let response = self.rt.block_on(self.client.get_artists(None))?;

        response
            .into_iter()
            .flat_map(|indexi3| indexi3.artist.into_iter().map(|artist| Ok(artist.name)))
            .collect()
    }

    fn search(&self, exp: Vec<Expression>) -> color_eyre::Result<Vec<Song>> {
        let query = exp
            .first()
            .map(|exp| match exp {
                Expression::TagComparison {
                    tag: TagName::Artist,
                    op: TagOp::Eq,
                    value,
                } => value,
                _ => todo!("unsupported expression"),
            })
            .expect("no expression");

        let response = self.rt.block_on(self.client.search3(
            query,
            None,
            None,
            None,
            None,
            Some(250),
            None,
            Option::<String>::None,
        ))?;

        response.song.into_iter()
    }

    fn get_artist(&self) -> color_eyre::Result<mpdrome_bridge::Artist> {
        todo!()
    }
}

impl SubSonicClient {
    fn song(&self, id: &str) -> color_eyre::Result<Vec<Song>> {
        let fut = async {
            let (mut artist, mut album,   mut song) = (None, None, None);
            if let Ok(song) = self.client.get_song(id).await {
                song = song;
                if let Some(aid) = song.artist_id {
                   if let Ok(artist) = self.client.get_artist(aid).await {
                       artist = artist;
                   }
               }
               if let Some(album_id) = song.album_id {
                  if let Ok(album) = self.client.get_album(album_id).await {
                      album = album;
                  }
              }
            }
            (artist, song)
        };

        let response = self.rt.block_on(fut)?;

        response.song.into_iter()
    }
}

fn subsonic_song_to_mpd(artist: ArtistWithAlbumsId3, album: AlbumWithSongsId3, song: Child) -> Song {
    Song {
        file: song.path.unwrap(),
        last_modified: None,
        added: None,
        format: song.bit_rate.unwrap().to_string(),
        album: artist.album.iter().find(|a| a.s),
        album_artist: album.base.artist.unwrap(),
        artist: artist.base.name,
        date: album.base.created,
        disc: "1".to_string(),
        title: song.name,
        original_date: todo!(),
        genre: song.genre.unwrap(),
        musicbrainz_albumid: todo!(),
        musicbrainz_artistid: todo!(),
        musicbrainz_albumartistid: todo!(),
        musicbrainz_releasegroupid: todo!(),
        musicbrainz_trackid: todo!(),
        track: todo!(),
        label: todo!(),
        musicbrainz_releasetrackid: song.,
        time: todo!(),
        duration: song.duration,
    }
}
