extern crate websocket;

use broadcaster;
use control_nexus::{ControlInfo, ControlNexus, ControlUpdateProcessor};
use controls;
use json;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::thread;
use websocket::message::Message;
use websocket::sync::{Client, Server};
use websocket::OwnedMessage;

pub fn start<'a>(
  guistring: &str,
  cup: Box<dyn ControlUpdateProcessor>,
  ip: &str,
  websockets_port: &str,
  block: bool,
) -> Result<ControlNexus, Box<dyn std::error::Error>> {
  let guival = serde_json::from_str(guistring)?;

  let root = try!(json::deserialize_root(&guival));

  println!(
    "starting websocket server for control page: {} ",
    root.title
  );

  // from control tree, make a map of ids->controls.
  let mapp = controls::make_control_map(&*root.root_control);
  let cnm = controls::control_map_to_name_map(&mapp);

  let ci = ControlInfo {
    cm: mapp,
    cnm: cnm,
    guijson: String::new() + guistring,
  };

  let cmshare = Arc::new(Mutex::new(ci));
  let bc = broadcaster::Broadcaster::new();

  let cn_ret = ControlNexus {
    ci: cmshare,
    bc: bc,
  };
  let cn_ws = cn_ret.clone();

  let mut websockets_ip = String::from(ip);
  websockets_ip.push_str(":");
  websockets_ip.push_str(&websockets_port);

  if block {
    match websockets_main(websockets_ip, cn_ws, Arc::new(Mutex::new(cup))) {
      Ok(_) => (),
      Err(e) => println!("error in websockets_main: {:?}", e),
    }
  } else {
    // Spawn a thread for the websockets handler.
    thread::spawn(
      move || match websockets_main(websockets_ip, cn_ws, Arc::new(Mutex::new(cup))) {
        Ok(_) => (),
        Err(e) => println!("error in websockets_main: {:?}", e),
      },
    );
  }

  Ok(cn_ret)
}

fn sendcontrols(
  ci: &Arc<Mutex<ControlInfo>>,
  client: &mut Client<std::net::TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
  let sci = ci.lock().unwrap();

  let updarray = controls::cm_to_update_array(&sci.cm);

  // build json message containing both guijson and the updarray.
  let mut updvals = Vec::new();

  for upd in updarray {
    let um = json::encode_update_message(&upd);
    updvals.push(um);
  }

  let mut guival: Value = serde_json::from_str(&sci.guijson[..])?;

  match guival.as_object_mut() {
    Some(obj) => {
      obj.insert("state".to_string(), Value::Array(updvals));
      ()
    }
    None => (),
  }

  let guistring = serde_json::ser::to_string(&guival)?;
  let message = Message::text(guistring);
  client.send_message(&message)?;
  Ok(())
}

fn websockets_main(
  ipaddr: String,
  acn: ControlNexus,
  cup: Arc<Mutex<Box<dyn ControlUpdateProcessor>>>,
) -> Result<(), Box<dyn std::error::Error>> {
  println!("websockets address {:?}", ipaddr);
  let server = Server::bind(&ipaddr[..])?;
  for request in server.filter_map(Result::ok) {
    let mut scn = acn.clone();
    let cup = cup.clone();
    // Spawn a new thread for each connection.
    thread::spawn(move || {
      if !request.protocols().contains(&"rust-websocket".to_string()) {
        request.reject().unwrap();
        return;
      }

      let mut client: Client<std::net::TcpStream> =
        request.use_protocol("rust-websocket").accept().unwrap();

      let ip = client.peer_addr().unwrap();
      println!("Connection from {}", ip);

      // send up controls.
      match sendcontrols(&scn.ci, &mut client) {
        Err(e) => println!("error sending controls to client: {}", e),
        Ok(_) => (),
      }

      //(websocket::receiver::Reader<std::net::TcpStream>, websocket::sender::Writer<std::net::TcpStream> )
      let (mut receiver, sender) = client.split().unwrap();

      let sendmeh = Arc::new(Mutex::new(sender));
      scn.bc.register(sendmeh.clone());

      for message in receiver.incoming_messages() {
        let message = message.unwrap();

        match message {
          OwnedMessage::Close(_) => {
            let message = OwnedMessage::Close(None);
            let mut s = sendmeh.lock().unwrap();
            s.send_message(&message).unwrap();
            println!("Client {} disconnected", ip);
            return;
          }
          OwnedMessage::Ping(ping) => {
            let message = OwnedMessage::Pong(ping);
            let mut s = sendmeh.lock().unwrap();
            s.send_message(&message).unwrap();
          }
          OwnedMessage::Text(txt) => match serde_json::from_str(txt.as_str()) {
            Err(e) => println!("error {:?}", e),
            Ok(jsonval) => {
              let s_um = json::decode_update_message(&jsonval);
              match s_um {
                Some(updmsg) => {
                  {
                    let mut sci = scn.ci.lock().unwrap();
                    {
                      let mbcntrl = sci.cm.get_mut(controls::get_um_id(&updmsg));
                      match mbcntrl {
                        Some(cntrl) => {
                          (*cntrl).update(&updmsg);
                          scn.bc.broadcast_others(&ip, Message::text(txt.clone()));
                          ()
                        }
                        None => println!("none"),
                      }
                    }
                  }

                  let mut scup = cup.lock().unwrap();
                  scup.on_update_received(&updmsg, &mut scn);
                }
                _ => println!("decode_update_message failed on websockets msg: {:?}", txt),
              }
            }
          },
          _ => {
            println!("unrecognized message type: {:?}", message);
          }
        }
      }
    });
  }

  Ok(())
}
