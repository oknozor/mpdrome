use std::io::Write;

use mpdrome_bridge::MpdBridge;
use mpdrome_macro::ToMpdResponse;
use mpdrome_mpd_model::{artist::Artists, status::Status};
use mpdrome_mpd_protocol::{
    command::Args,
    filter::{self, Expression},
    request::Request,
};

use crate::{
    listener::{Listener, ListenerResult},
    responses::{Response, commands::Commands},
};

pub trait Handler {
    fn handle(&mut self, request: Request, args: Args) -> ListenerResult<()>;
}

impl<T: MpdBridge + Clone> Handler for Listener<T> {
    fn handle(&mut self, request: Request, args: Args) -> ListenerResult<()> {
        println!("{:?} - {:?}", request, args);
        let response = match request {
            Request::Commands => Response::Commands(Commands::default()),
            Request::Binarylimit => Response::Ok,
            Request::Status => Response::Status(Status::default()),
            Request::Playlistinfo => Response::Ok,
            Request::Idle => {
                self.idle_wait()?;
                Response::Ok
            }
            Request::Lsinfo => Response::Ok,
            Request::Listplaylists => Response::Ok,
            Request::List => self.handle_list(args)?,
            Request::Find => self.handle_find(args)?,
            command => unimplemented!("unimplemented command: {command}"),
        };

        response.write_response(self)?;
        self.flush()?;
        Ok(())
    }
}

impl<T: MpdBridge + Clone> Listener<T> {
    fn handle_list(&mut self, args: Args) -> ListenerResult<Response> {
        if args.contains("artist") {
            let artist = self.bridge().list_artist()?;
            return Ok(Response::ListArtist(Artists { artist }));
        }

        if args.contains("album") {
            let artist = self.bridge().list_artist()?;
            return Ok(Response::ListArtist(Artists { artist }));
        }

        todo!()
    }

    fn handle_find(&mut self, args: Args) -> ListenerResult<Response> {
        let expr = args
            .into_iter()
            .map(|arg| {
                let (_, exp) = filter::parse_expression(&arg).unwrap();
                exp
            })
            .collect::<Vec<Expression>>();

        println!("{:?}", expr);
        todo!()
    }
}
