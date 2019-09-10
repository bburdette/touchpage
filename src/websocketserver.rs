extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

pub fn startserver<'a>(
  guistring: &str,
  cup: Box<ControlUpdateProcessor>,
  ip: &str,
  websockets_port: &str,
) -> Result<ControlServer, Box<std::error::Error>> {
  let guival: Value = try!(serde_json::from_str(guistring));

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

  let cs_ret = ControlServer {
    ci: cmshare,
    bc: bc,
  };

  // Spawn a thread for the websockets handler.
  thread::spawn(move || {
    match websockets_main(websockets_ip, wscmshare, wsbc, Arc::new(Mutex::new(cup))) {
      Ok(_) => (),
      Err(e) => println!("error in websockets_main: {:?}", e),
    }
  });

  Ok(cs_ret)
}

fn websockets_main(
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

fn websockets_main() {
  let server = Server::bind("127.0.0.1:8500").unwrap();

  for request in server.filter_map(Result::ok) {
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

      let message = OwnedMessage::Text("Hello".to_string());
      client.send_message(&message).unwrap();

      //(websocket::receiver::Reader<std::net::TcpStream>, websocket::sender::Writer<std::net::TcpStream> )
      let (mut receiver, mut sender) = client.split().unwrap();

      for message in receiver.incoming_messages() {
        // TODO message handler here.
        let message = message.unwrap();

        match message {
          OwnedMessage::Close(_) => {
            let message = OwnedMessage::Close(None);
            sender.send_message(&message).unwrap();
            println!("Client {} disconnected", ip);
            return;
          }
          OwnedMessage::Ping(ping) => {
            let message = OwnedMessage::Pong(ping);
            sender.send_message(&message).unwrap();
          }
          _ => sender.send_message(&message).unwrap(),
        }
      }
    });
  }
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
