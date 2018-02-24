use nanomsg;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::result::{Result};

// Server setting type for protocol Server
pub struct ServerSettings<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

// Result type for Serve
pub type ServeResult = Result<(), nanomsg::Error>;
// Result type for Send
pub type SendResult = Result<String, nanomsg::Error>;

// Handler result type
pub type HandlerResult = Result<Vec<u8>, String>;

// Handler for server response
pub trait ServerHandler {
    fn handler(&self, msg: &Vec<u8>) -> HandlerResult;
}

// Send event message
pub fn send(config: &ServerSettings, message: &String) -> SendResult {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&config.url[..]).unwrap();
    let mut reply = String::new();

    debug!("send.socket.write_all: {}", message);

    socket.write_all(message.as_bytes())
        .map_err(|err| { error!("send.socket.write_all: {}", err); err })
        .and_then(|_| {
            socket.read_to_string(&mut reply)
                .map_err(|err| { error!("send.socket.read_to_string: {}", err); err })
        })
        .map_err(|err| {
            let _ = endpoint.shutdown()
                .map_err(|err| error!("send.endpoint.shutdown: {}", err) );
            err
        })?;

    debug!("send.reply: {}", reply);

    let _ = endpoint.shutdown()
        .map_err(|err| error!("send.endpoint.shutdown: {}", err) );
    return Ok(reply);
}