use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    thread::{self, sleep, JoinHandle},
    time::Duration,
};
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};

use crate::Grid;

mod test;

pub struct Client {
    ths: (
        JoinHandle<Result<(), WebSocketError>>,
        JoinHandle<Result<(), WebSocketError>>,
    ),
}

impl Client {
    pub fn new(addr: &str) -> Result<(Self, Sender<Vec<u8>>, Receiver<Vec<u8>>), WebSocketError> {
        let (user_sender, recv_in_serv) = channel();
        let (cli_sender, user_recv) = channel();

        let client = ClientBuilder::new(addr)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()?;

        let (mut receiver, mut sender) = client.split().unwrap();

        let sender_ws = thread::spawn(move || -> Result<(), WebSocketError> {
            while let Ok(message) = recv_in_serv.recv() {
                sender.send_message(&OwnedMessage::Text(format!("{:?}", message)))?; // нужно отправлять (корды фигуры, MatricPoint4x)
                sleep(Duration::from_millis(200));
            }
            Ok(())
        });

        let receiver_ws = thread::spawn(move || -> Result<(), WebSocketError> {
            for message in receiver.incoming_messages() {
                let message = message?;
                match message {
                    OwnedMessage::Text(_) => todo!(),
                    OwnedMessage::Binary(g) => {
                        if let Err(_) = cli_sender.send(g) {
                            todo!()
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
