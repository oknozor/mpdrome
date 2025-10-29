use strum::{Display, EnumString, VariantNames};

pub mod error;
pub mod filter;
mod idle;
mod parser;

pub struct Command {
    pub request: Request,
    pub args: Args,
}

#[derive(Debug)]
pub struct Args(Vec<String>);

impl Args {
    pub fn contains(&self, term: impl ToString) -> bool {
        let term = term.to_string().to_lowercase();
        self.0.iter().any(|s| s.to_lowercase() == term)
    }

    pub fn into_iter(self) -> impl Iterator<Item = String> {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, EnumString, Display, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Request {
    Add,
    Addid,
    Addtagid,
    Albumart,
    Binarylimit,
    Channels,
    Clear,
    Clearerror,
    Cleartagid,
    Close,
    Commands,
    Config,
    Consume,
    Count,
    Crossfade,
    Currentsong,
    Decoders,
    Delete,
    Deleteid,
    Delpartition,
    Disableoutput,
    Enableoutput,
    Find,
    Findadd,
    Getfingerprint,
    Getvol,
    Idle,
    NoIdle,
    Kill,
    List,
    Listall,
    Listallinfo,
    Listfiles,
    Listmounts,
    Listpartitions,
    Listplaylist,
    Listplaylistinfo,
    Listplaylists,
    Load,
    Lsinfo,
    Mixrampdb,
    Mixrampdelay,
    Move,
    Moveid,
    Moveoutput,
    Newpartition,
    Next,
    Notcommands,
    Outputs,
    Outputset,
    Partition,
    Password,
    Pause,
    Ping,
    Play,
    Playid,
    Playlist,
    Playlistadd,
    Playlistclear,
    Playlistdelete,
    Playlistfind,
    Playlistid,
    Playlistinfo,
    Playlistlength,
    Playlistmove,
    Playlistsearch,
    Plchanges,
    Plchangesposid,
    Previous,
    Prio,
    Prioid,
    Protocol,
    Random,
    Rangeid,
    Readcomments,
    Readmessages,
    Readpicture,
    Rename,
    Repeat,
    ReplayGainMode,
    ReplayGainStatus,
    Rescan,
    Rm,
    Save,
    Search,
    Searchadd,
    Searchaddpl,
    Searchcount,
    Searchplaylist,
    Seek,
    Seekcur,
    Seekid,
    Sendmessage,
    Setvol,
    Shuffle,
    Single,
    Stats,
    Status,
    Stop,
    Subscribe,
    Swap,
    Swapid,
    Tagtypes,
    Toggleoutput,
    Unsubscribe,
    Update,
    Urlhandlers,
    Volume,
}
