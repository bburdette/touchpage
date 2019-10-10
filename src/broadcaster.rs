use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use websocket::message::Message;

pub type SendBlah = Arc<Mutex<websocket::sender::Writer<std::net::TcpStream>>>;

#[derive(Clone)]
pub struct Broadcaster {
  tvs: Arc<Mutex<Vec<SendBlah>>>,
}

fn mysockeq(saleft: &SocketAddr, saright: &SocketAddr) -> bool {
  match (saleft, saright) {
    (&SocketAddr::V4(l), &SocketAddr::V4(r)) => l == r,
    (&SocketAddr::V6(l), &SocketAddr::V6(r)) => l == r,
    _ => false,
  }
}

impl Broadcaster {
  pub fn new() -> Broadcaster {
    Broadcaster {
      tvs: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn register(&mut self, sender: SendBlah) {
    let mut tvs = self.tvs.lock().unwrap();

    tvs.push(sender);
  }

  pub fn broadcast(&self, msg: Message) {
    let mut tvs = self.tvs.lock().unwrap();

    for tv in tvs.iter_mut() {
      let mut tvsend = tv.lock().unwrap();
      match tvsend.send_message(&msg) {
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
      let mut tvsend = tv.lock().unwrap();
      let sa_send = tvsend.stream.peer_addr();
      match sa_send {
        Ok(ss) => {
          if !mysockeq(sa, &ss) {
            match tvsend.send_message(&msg) {
              Err(e) => {
                println!("error from send_message: {:?}", e);
              }
              _ => {}
            }
          }
        }
        _ => {}
      }
    }
  }
}
