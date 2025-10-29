use std::io::{self, Write};

use mpdrome_macro::{MpdResponse, ToMpdResponse};
use strum::Display;

use crate::protocol::{
    requests::Request,
    responses::{commands::CommandsResponse, status::StatusResponse},
};

pub mod commands;
pub mod readpicture;
pub mod status;

pub struct OK;

impl ToMpdResponse for OK {
    fn write_response<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"OK\n")?;
        w.flush()
    }
}

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

#[derive(MpdResponse)]
pub struct Artists {
    pub artist: Vec<String>,
}

pub enum Response {
    Commands(CommandsResponse),
    Status(StatusResponse),
    ListArtist(Artists),
    Ok(OK),
}

impl ToMpdResponse for Response {
    fn write_response<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Response::Commands(response) => response.write_response(w),
            Response::Status(response) => response.write_response(w),
            Response::Ok(response) => response.write_response(w),
            Response::ListArtist(response) => response.write_response(w),
        }
    }
}
