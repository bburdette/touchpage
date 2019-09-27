extern crate websocket;

use broadcaster;
use control_nexus::{ControlInfo, ControlNexus, ControlUpdateProcessor};
use controls;
use json;
use std::sync::{Arc, Mutex};
use std::thread;
use websocket::message::Message;
use websocket::sync::Server;
use websocket::OwnedMessage;

pub fn startserver<'a>(
  guistring: &str,
  cup: Box<ControlUpdateProcessor>,
  ip: &str,
  websockets_port: &str,
  block: bool,
) -> Result<ControlNexus, Box<std::error::Error>> {
  let guival = serde_json::from_str(guistring)?;

  let blah = try!(json::deserialize_root(&guival));

  println!(
    "title: {} rootcontroltype: {} ",
    blah.title,
    blah.root_control.control_type()
  );
  println!("controls: {:?}", blah.root_control);

  // from control tree, make a map of ids->controls.
  let mapp = controls::make_control_map(&*blah.root_control);
  let cnm = controls::control_map_to_name_map(&mapp);

  let ci = ControlInfo {
    cm: mapp,
    cnm: cnm,
    guijson: String::new() + guistring,
  };

  let cmshare = Arc::new(Mutex::new(ci));
  let wscmshare = cmshare.clone();
  // for sending, bind to this.  if we bind to localhost, we can't
  // send messages to other machines.
  let bc = broadcaster::Broadcaster::new();
  let wsbc = bc.clone();

  let cs_ret = ControlNexus {
    ci: cmshare,
    bc: bc,
  };

  let mut websockets_ip = String::from(ip);
  websockets_ip.push_str(":");
  websockets_ip.push_str(&websockets_port);

  if block {
    match websockets_main(websockets_ip, wscmshare, wsbc, Arc::new(Mutex::new(cup))) {
      Ok(_) => (),
      Err(e) => println!("error in websockets_main: {:?}", e),
    }
  } else {
    // Spawn a thread for the websockets handler.
    thread::spawn(move || {
      match websockets_main(websockets_ip, wscmshare, wsbc, Arc::new(Mutex::new(cup))) {
        Ok(_) => (),
        Err(e) => println!("error in websockets_main: {:?}", e),
      }
    });
  }

  Ok(cs_ret)
}

fn websockets_main(
  ipaddr: String,
  ci: Arc<Mutex<ControlInfo>>,
  broadcaster: broadcaster::Broadcaster,
  cup: Arc<Mutex<Box<ControlUpdateProcessor>>>,
) -> Result<(), Box<std::error::Error>> {
  println!("websockets_main {:?}", ipaddr);
  let server = Server::bind(&ipaddr[..])?;
  for request in server.filter_map(Result::ok) {
    let sci = ci.clone();
    let mut broadcaster = broadcaster.clone();
    let cup = cup.clone();
    println!("request: {:?}", request.protocols());
    // Spawn a new thread for each connection.
    thread::spawn(move || {
      if !request.protocols().contains(&"rust-websocket".to_string()) {
        request.reject().unwrap();
        return;
      }

      let mut client = request.use_protocol("rust-websocket").accept().unwrap();

      let ip = client.peer_addr().unwrap();

      println!("Connection from {}", ip);

      // TODO send up controls.

      /*      let (sender, mut receiver) = client.split();
            // register the sender with broadcaster.
            let sendmeh = Arc::new(Mutex::new(sender));
            broadcaster.register(sendmeh.clone());
      */

      /*      let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();
      */
      //(websocket::receiver::Reader<std::net::TcpStream>, websocket::sender::Writer<std::net::TcpStream> )
      let (mut receiver, mut sender) = client.split().unwrap();

      let sendmeh = Arc::new(Mutex::new(sender));
      broadcaster.register(sendmeh.clone());

      for message in receiver.incoming_messages() {
        // TODO message handler here.
        let message = message.unwrap();

        println!("received msg: {:?}", message);

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
          OwnedMessage::Text(txt) => {
            println!("txt {}", txt);
            // let message = OwnedMessage::Pong(ping);
            // sender.send_message(&message).unwrap();
            match serde_json::from_str(txt.as_str()) {
              Err(e) => println!("error {:?}", e),
              Ok(jsonval) => {
                let s_um = json::decode_update_message(&jsonval);
                match s_um {
                  Some(updmsg) => {
                    {
                      println!("updmsg: {:?}", updmsg);
                      let mut sci = sci.lock().unwrap();
                      {
                        let mbcntrl = sci.cm.get_mut(controls::get_um_id(&updmsg));
                        match mbcntrl {
                          Some(cntrl) => {
                            (*cntrl).update(&updmsg);
                            broadcaster.broadcast_others(&ip, Message::text(txt.clone()));
                            ()
                          }
                          None => println!("none"),
                        }
                      }
                    }
                    let mut scup = cup.lock().unwrap();
                    let sci = match sci.lock() {
                      Ok(sci) => sci,
                      Err(poisoned) => poisoned.into_inner(),
                    };

                    scup.on_update_received(&updmsg, &*sci);
                  }
                  _ => println!("decode_update_message failed on websockets msg: {:?}", txt),
                }
              }
            }
          }
          _ => {
            println!("unrecognized message type: {:?}", message);
          }
        }
      }
    });
  }

  Ok(())
}

/*
 // send up the json of the current controls.
 {
   let sci = ci.lock().unwrap();

   let updarray = controls::cm_to_update_array(&sci.cm);

   // build json message containing both guijson and the updarray.
   let mut updvals = Vec::new();

   for upd in updarray {
     let um = json::encode_update_message(&upd);
     updvals.push(um);
   }

   let mut guival: Value = try!(serde_json::from_str(&sci.guijson[..]));

   match guival.as_object_mut() {
     Some(obj) => {
       obj.insert("state".to_string(), Value::Array(updvals));
       ()
     }
     None => (),
   }

   let guistring = try!(serde_json::ser::to_string(&guival));
   let message = Message::text(guistring);
   try!(client.send_message(&message));
}*/

/*
   Message handler

{
    let message: Message = try!(msg);
    // println!("message: {:?}", message);

    match message.opcode {
      Type::Close => {
        let message = Message::close();
        // let mut sender = try!(sendmeh.lock());
        let mut sender = sendmeh.lock().unwrap();
        try!(sender.send_message(&mut *sender, &message));
        println!("Client {} disconnected", ip);
        return Ok(());
      }
      Type::Ping => {
        println!("Message::Ping(data)");
        let message = Message::pong(message.payload);
        let mut sender = sendmeh.lock().unwrap();
        try!(sender.send_message(*sender, &message));
      }
      Type::Text => {
        let u8 = message.payload.to_owned();
        let str = try!(std::str::from_utf8(&*u8));
        let jsonval: Value = try!(serde_json::from_str(str));
        let s_um = json::decode_update_message(&jsonval);
        match s_um {
          Some(updmsg) => {
            {
              let mut sci = ci.lock().unwrap();
              {
                let mbcntrl = sci.cm.get_mut(controls::get_um_id(&updmsg));
                match mbcntrl {
                  Some(cntrl) => {
                    (*cntrl).update(&updmsg);
                    broadcaster.broadcast_others(&ip, Message::text(str));
                    ()
                  }
                  None => println!("none"),
                }
              }
            }
            let mut scup = cup.lock().unwrap();
            let sci = match ci.lock() {
              Ok(sci) => sci,
              Err(poisoned) => poisoned.into_inner(),
            };

            scup.on_update_received(&updmsg, &*sci);
          }
          _ => println!(
            "decode_update_message failed on websockets msg: {:?}",
            message
          ),
        }
      }
      _ => {
        println!("unknown websockets msg: {:?}", message);
      }
    }
  }*/

/*fn websockets_main(
  ipaddr: String,
  ci: Arc<Mutex<ControlInfo>>,
  broadcaster: broadcaster::Broadcaster,
  cup: Arc<Mutex<Box<ControlUpdateProcessor>>>,
) -> Result<(), Box<std::error::Error>> {
  let server = try!(Server::bind(&ipaddr[..]));

  for connection in server {
    // Spawn a new thread for each connection.
    println!("new websockets connection!");
    let conn = try!(connection);
    let sci = ci.clone();
    let broadcaster = broadcaster.clone();
    let cup = cup.clone();
    thread::spawn(
      move || match websockets_client(conn, sci, broadcaster, cup) {
        Ok(_) => (),
        Err(e) => {
          println!("error in websockets thread: {:?}", e);
          ()
        }
      },
    );
  }

  Ok(())
}

*/
