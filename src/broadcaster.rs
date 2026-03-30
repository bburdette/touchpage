use std::io::{Read, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
// use websocket::message::Message;
use tungstenite::{Message, WebSocket};

// pub type SendBlah = Arc<Mutex<websocket::sender::Writer<std::net::TcpStream>>>;
pub type SendBlah<Stream> = Arc<Mutex<WebSocket<Stream>>>;

// pub type BStream<Stream> = dyn Stream + std::io::Read + std::io::Write;

#[derive(Clone)]
pub struct Broadcaster<Stream: Read + Write> {
  tvs: Arc<Mutex<Vec<SendBlah<Stream>>>>,
}

fn mysockeq(saleft: &SocketAddr, saright: &SocketAddr) -> bool {
  match (saleft, saright) {
    (&SocketAddr::V4(l), &SocketAddr::V4(r)) => l == r,
    (&SocketAddr::V6(l), &SocketAddr::V6(r)) => l == r,
    _ => false,
  }
}

impl<Stream: Read + Write> Broadcaster<Stream> {
  pub fn new() -> Broadcaster<Stream> {
    Broadcaster {
      tvs: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn register(&mut self, sender: SendBlah<Stream>) {
    let mut tvs = self.tvs.lock().unwrap();

    tvs.push(sender);
  }

  pub fn broadcast(&self, msg: Message) {
    let mut tvs = self.tvs.lock().unwrap();

    for tv in tvs.iter_mut() {
      let mut tvsend: WebSocket<Stream> = *tv.lock().unwrap();
      match tvsend.write_message(msg) {
        Err(e) => {
          println!("error from send_message: {:?}", e);
        }
        _ => {}
      }
    }
  }

  // broadcast to all except for ones with the same socket address.
  pub fn broadcast_others(&self, sa: &SocketAddr, msg: Message) {
    let mut tvs = self.tvs.lock().unwrap();
    for tv in tvs.iter_mut() {
      let mut tvsend = *tv.lock().unwrap();
      match tvsend.write_message(msg) {
        Err(e) => {
          println!("error from send_message: {:?}", e);
        }
        _ => {}
      }
    }
    // let sa_send = tvsend.peer_addr();
    // match sa_send {
    //   Ok(ss) => {
    //     if !mysockeq(sa, &ss) {
    //       match tvsend.write_message(msg) {
    //         Err(e) => {
    //           println!("error from send_message: {:?}", e);
    //         }
    //         _ => {}
    //       }
    //     }
    //   }
    //   _ => {}
    // }
  }
}
