extern crate websocket;

#[macro_use]
extern crate log;
extern crate serde_json;
extern crate failure;
extern crate actix_web;
extern crate toml;

#[macro_use]
mod controls;
mod broadcaster;
pub mod json;
// mod server;
pub mod control_nexus;
pub mod control_updates;
pub mod guibuilder;
pub mod string_defaults;
pub mod webserver;
pub mod websocketserver;

// need to lock the control structs and stuff, refresh them, then send out the
// updates.

/*

fn stringify(x: u32) -> String { format!("error code: {:?}", x) }
// TODO: refactor to return a (rx/sx) pair for sending, recieving messages.
// library users start the websockets_main and get that pair of things.
// then, can send the various control structs and receive the messages.
fn websockets_main(
  ipaddr: String,
  ci: Arc<Mutex<ControlInfo>>,
  broadcaster: broadcaster::Broadcaster,
  cup: Arc<Mutex<Box<ControlUpdateProcessor>>>,
) -> Result<(), Box<std::error::Error>> {
  let server = Server::bind(&ipaddr[..])?;
  // let server : WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>= try!(WsServer::bind(&ipaddr[..]));

  for connection in server {
    // Spawn a new thread for each connection.
    println!("new websockets connection!");
    let half = connection.map_err(|x| format!("error {:?}", x)) ;
    let conn = half?.accept().map_err(|s| format!("wat: {:?}",s))?;
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

fn websockets_client(
  connection: websocket::sync::Client<std::net::TcpStream>,
  ci: Arc<Mutex<ControlInfo>>,
  mut broadcaster: broadcaster::Broadcaster,
  cup: Arc<Mutex<Box<ControlUpdateProcessor>>>,
) -> Result<(), Box<std::error::Error>> {
  // Get the request
  let request = try!(connection.read_request());
  // Keep the headers so we can check them
  let headers = request.headers.clone();

  try!(request.validate()); // Validate the request

  let mut response = request.accept(); // Form a response

  if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
    if protocols.contains(&("rust-websocket".to_string())) {
      // We have a protocol we want to use
      response
        .headers
        .set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
    }
  }

  let mut client = try!(response.send()); // Send the response

  let ip = try!(client.get_mut_sender().get_mut().peer_addr());

  println!("Websocket connection from {}", ip);

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
  }


  let (sender, mut receiver) = client.split();
  // register the sender with broadcaster.
  let sendmeh = Arc::new(Mutex::new(sender));
  broadcaster.register(sendmeh.clone());

  for msg in receiver.incoming_messages() {
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
  }

  Ok(())
}
*/
