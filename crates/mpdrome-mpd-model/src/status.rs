use mpdrome_macro::MpdResponse;
use strum::Display;

#[derive(MpdResponse)]
pub struct Status {
    pub partition: String,
    pub volume: u32,
    pub repeat: bool,
    pub random: bool,
    pub single: OnOffOneshot,
    pub consume: OnOffOneshot,
    pub playlistlength: u32,
    pub state: State,
    pub song: Option<u32>,
    pub songid: Option<u32>,
    pub nextsong: Option<u32>,
    pub nextsongid: Option<u32>,
    pub elapsed: u64,
    pub duration: u64,
    pub bitrate: Option<u32>,
    pub xfade: Option<u32>,
    pub mixrampdb: Option<String>,
    pub mixrampdelay: Option<String>,
    pub audio: Option<String>,
    pub updating_db: Option<u32>,
    pub error: Option<String>,
    pub lastloadedplaylist: Option<String>,
}

#[derive(Default, strum::EnumString, strum::AsRefStr, Display)]
#[strum(serialize_all = "lowercase")]
pub enum OnOffOneshot {
    #[strum(serialize = "0")]
    On,
    #[default]
    #[strum(serialize = "1")]
    Off,
    Oneshot,
}

#[derive(Default, strum::EnumString, strum::AsRefStr, Display)]
#[strum(serialize_all = "lowercase")]
pub enum State {
    Play,
    #[default]
    Stop,
    Pausse,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            volume: 0,
            repeat: false,
            random: false,
            single: OnOffOneshot::default(),
            consume: OnOffOneshot::default(),
            song: None,
            nextsong: None,
            elapsed: 0,
            duration: 0,
            bitrate: None,
            mixrampdelay: None,
            audio: None,
            updating_db: None,
            error: None,
            partition: "/subsonic".to_string(),
            playlistlength: 0,
            state: State::Stop,
            songid: None,
            nextsongid: None,
            xfade: None,
            mixrampdb: None,
            lastloadedplaylist: None,
        }
    }
}
