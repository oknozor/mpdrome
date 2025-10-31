use std::io::{self, Write};

use mpdrome_macro::{MpdResponse, ToMpdResponse};
use mpdrome_mpd_model::{artist::Artists, song::Song, status::Status};
use mpdrome_mpd_protocol::request::Request;
use strum::Display;

use crate::responses::commands::Commands;

pub mod commands;

#[derive(MpdResponse)]
pub struct Ack {
    code: AckError,
    request: Request,
    message: String,
}

#[derive(Display)]
pub enum AckError {
    AckErrorNotList = 1,
    AckErrorArg = 2,
    AckErrorPassword = 3,
    AckErrorPermission = 4,
    AckErrorUnknown = 5,

    AckErrorNoExist = 50,
    AckErrorPlaylistMax = 51,
    AckErrorSystem = 52,
    AckErrorPlaylistLoad = 53,
    AckErrorUpdateAlready = 54,
    AckErrorPlayerSync = 55,
    AckErrorExist = 56,
}

pub enum Response {
    Commands(Commands),
    Status(Status),
    ListArtist(Artists),
    Songs(Vec<Song>),
    Ok,
}

impl ToMpdResponse for Response {
    fn write_content<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Response::Commands(response) => response.write_content(w),
            Response::Status(response) => response.write_content(w),
            Response::ListArtist(response) => response.write_content(w),
            Response::Songs(response) => response.write_content(w),
            Response::Ok => Ok(()),
        }
    }
}
