// extern crate websocket;

use broadcaster;
use control_nexus::{ControlInfo, ControlNexus, ControlUpdateProcessor};
use controls;
use json;
use serde_json::Value;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::{Message, WebSocket};
// use websocket::message::Message;
// use websocket::sync::{Client, Server};
// use websocket::OwnedMessage;
use std::net::TcpListener;
use tungstenite::accept_hdr;

pub fn start<'a>(
  guistring: &str,
  cup: Box<dyn ControlUpdateProcessor<std::net::TcpStream>>,
  ip: &str,
  websockets_port: &str,
  block: bool,
) -> Result<ControlNexus<std::net::TcpStream>, Box<dyn std::error::Error>> {
  let guival = serde_json::from_str(guistring)?;

  let root = (json::deserialize_root(&guival))?;

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
    ci: cmshare.clone(),
    bc: bc,
  };
  let cn_ws = ControlNexus {
    ci: cmshare,
    bc: bc,
  };

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

// fn sendcontrols(
//   ci: &Arc<Mutex<ControlInfo>>,
//   client: &mut Client<std::net::TcpStream>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//   let sci = ci.lock().unwrap();

//   let updarray = controls::cm_to_update_array(&sci.cm);

//   // build json message containing both guijson and the updarray.
//   let mut updvals = Vec::new();

//   for upd in updarray {
//     let um = json::encode_update_message(&upd);
//     updvals.push(um);
//   }

//   let mut guival: Value = serde_json::from_str(&sci.guijson[..])?;

//   match guival.as_object_mut() {
//     Some(obj) => {
//       obj.insert("state".to_string(), Value::Array(updvals));
//       ()
//     }
//     None => (),
//   }

//   let guistring = serde_json::ser::to_string(&guival)?;
//   let message = Message::text(guistring);
//   client.send_message(&message)?;
//   Ok(())
// }

fn sendcontrols<Stream: Read + Write>(
  ci: &Arc<Mutex<ControlInfo>>,
  client: &mut WebSocket<Stream>,
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
  client.write_message(message)?;
  Ok(())
}

fn websockets_main<Stream: Read + Write + Send>(
  ipaddr: String,
  acn: ControlNexus<Stream>,
  cup: Arc<Mutex<Box<dyn ControlUpdateProcessor<Stream>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
  println!("websockets address {:?}", ipaddr);
  let server = TcpListener::bind(&ipaddr[..])?;
  for ts in server.incoming() {
    match ts {
      Ok(tcpstream) => {
        //server.filter_map(Result::ok) {
        // let mut scn = acn.clone();
        let cup = cup.clone();

        // Spawn a new thread for each connection.
        thread::spawn(move || {
          let callback = |req: &Request, mut response: Response| {
            println!("Received a new ws handshake");
            println!("The request's path is: {}", req.uri().path());
            println!("The request's headers are:");
            for (ref header, _value) in req.headers() {
              println!("* {}", header);
            }

            // Let's add an additional header to our response to the client.
            // let headers = response.headers_mut();
            // headers.append("MyCustomHeader", ":)".parse()?);
            // headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

            Ok(response)
          };

          // Can't do a block that returns a Result.  But we can
          // do a closure and use the ? operator inside that.
          let r = || -> Result<(), Box<dyn std::error::Error>> {
            let mut websocket = accept_hdr(tcpstream, callback)?;

            sendcontrols(&acn.ci, &mut websocket)?;

            loop {
              let msg = websocket.read_message()?;
              if msg.is_binary() || msg.is_text() {
                websocket.write_message(msg)?;
              }
            }
          };
          // invoke the closure
          match r() {
            Err(e) => {
              error!("{:?}", e);
            }
            Ok(_) => (),
          };
        });
      }
      Err(e) => {
        error!("{:?}", e);
      }
    };

    /*
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
    */
  }

  Ok(())
}
