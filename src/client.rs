use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    thread::{self, sleep, JoinHandle},
    time::Duration,
};
use websocket::url::ParseError;
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};

use crate::Grid;

mod test;

pub struct Client {
    ths: (
        JoinHandle<Result<(), WebSocketError>>,
        JoinHandle<Result<(), WebSocketError>>,
    ),
}

#[derive(Debug)]
pub enum MyError {
    WebSocketError(WebSocketError),
    ParseError(ParseError),
}

impl Client {
    pub fn new(addr: &str) -> Result<(Self, Sender<Vec<u8>>, Receiver<Vec<u8>>), MyError> {
        let (user_sender, recv_in_serv) = channel();
        let (cli_sender, user_recv) = channel();

        let client = ClientBuilder::new(addr)
            .map_err(|e| MyError::ParseError(e))?
            .add_protocol("rust-websocket")
            // .add_protocols(protocols)
            .connect_insecure()
            .map_err(|e| MyError::WebSocketError(e))?;

        let (mut receiver, mut sender) = client.split().unwrap();

        let sender_ws = thread::spawn(move || -> Result<(), WebSocketError> {
            while let Ok(message) = recv_in_serv.recv() {
                sender.send_message(&OwnedMessage::Binary(message))?; // нужно отправлять (корды фигуры, MatricPoint4x)
                sleep(Duration::from_millis(200));
            }
            Ok(())
        });

        let receiver_ws = thread::spawn(move || -> Result<(), WebSocketError> {
            for message in receiver.incoming_messages() {
                match message? {
                    OwnedMessage::Text(_) => todo!(),
                    OwnedMessage::Binary(g) => {
                        if let Err(_e) = cli_sender.send(g) {
                            // print!("{:?}", e.0);
                            // todo!()
                        }
                    }
                    OwnedMessage::Close(_) => todo!(),
                    OwnedMessage::Ping(_) => todo!(),
                    OwnedMessage::Pong(_) => todo!(),
                };
            }
            Ok(())
        });

        Ok((
            Client {
                ths: (sender_ws, receiver_ws),
            },
            user_sender,
            user_recv,
        ))
    }
    pub fn join(self) -> std::thread::Result<Result<(), WebSocketError>> {
        self.ths.0.join().and(self.ths.1.join())
    }
}

#[allow(unused)]
struct Message {
    grid: Grid,
}

#[allow(unused)]
fn x2u16_from_string(mut s: String) -> (u16, u16) {
    let sl = &mut s[1..s.len() - 1].split(',');
    let first = sl.next().unwrap().parse().unwrap();
    let second = sl.next().unwrap().trim().parse().unwrap();
    (first, second)
}
