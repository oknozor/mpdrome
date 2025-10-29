use std::io::Write;

use mpdrome_bridge::MpdBridge;
use mpdrome_macro::ToMpdResponse;

use crate::{
    listener::{Listener, ListenerResult},
    protocol::{
        requests::{
            Args, Request,
            filter::{self, Expression},
        },
        responses::{Artists, OK, Response, commands::CommandsResponse, status::StatusResponse},
    },
};

pub trait Handler {
    fn handle(&mut self, request: Request, args: Args) -> ListenerResult<()>;
}

impl<T: MpdBridge + Clone> Handler for Listener<T> {
    fn handle(&mut self, request: Request, args: Args) -> ListenerResult<()> {
        println!("{:?} - {:?}", request, args);
        let response = match request {
            Request::Commands => Response::Commands(CommandsResponse::default()),
            Request::Binarylimit => Response::Ok(OK),
            Request::Status => Response::Status(StatusResponse::default()),
            Request::Playlistinfo => Response::Ok(OK),
            Request::Idle => {
                self.idle_wait()?;
                Response::Ok(OK)
            }
            Request::Lsinfo => Response::Ok(OK),
            Request::Listplaylists => Response::Ok(OK),
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
