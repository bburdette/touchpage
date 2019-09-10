extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
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
