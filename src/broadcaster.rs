use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use websocket::message::Message;
use websocket::sender;
use websocket::stream::Stream as WebSocketStream;
use websocket::stream::Stream;
use websocket::ws::Sender;

pub type SendBlah = Arc<Mutex<sender::Sender>>;

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
      match *tvsend.send_message(*tvsend, &msg) {
        Err(e) => {
          println!("error from send_message: {:?}", e);
        }
        _ => {}
      }
    }
  }
/*
  pub fn broadcast_others(&self, sa: &SocketAddr, msg: Message) {
    let mut tvs = self.tvs.lock().unwrap();

    for tv in tvs.iter_mut() {
      let mut tvsend = tv.lock().unwrap();
      sa_send = *tvsend.peer_addr();
      // println!("checking eq: {:?}, {:?}", sa, sa_send);
      if !mysockeq(sa, &sa_send) {
        // println!("sending to: {:?}", sa_send);
        match *tvsend.send_message(mut &*tvsend, &msg) {
          Err(e) => {
            println!("error from send_message: {:?}", e);
          }
          _ => {}
        }
      }
    }
  }
*/}
