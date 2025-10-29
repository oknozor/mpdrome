use strum::{Display, EnumString, VariantNames};

#[derive(Debug, PartialEq, EnumString, Display, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Subsystem {
    Database,
    Update,
    Playlist,
    Queue,
    Player,
    Mixer,
    Output,
    Options,
    Partition,
    Sticker,
    Subscription,
    Message,
    Neighbor,
    Mount,
}
