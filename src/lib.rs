
extern crate websocket;

use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use std::io::Write;
use std::io::{Error,ErrorKind};
use std::string::*;
use std::collections::BTreeMap;
use std::fmt::format;

use std::net::UdpSocket;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::message::Type;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

#[macro_use]
mod tryopt;
mod stringerror;
mod controls;
mod broadcaster;
mod string_defaults; 
mod control_updates;

extern crate serde_json;
use serde_json::Value;


fn load_string(file_name: &str) -> Result<String, Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::open(path));
  let mut result = String::new();
  try!(inf.read_to_string(&mut result));
  Ok(result)
}

fn write_string(text: &str, file_name: &str) -> Result<(), Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::create(path));
  match inf.write(text.as_bytes()) { 
    Ok(_) => Ok(()),
    Err(e) => Err(Box::new(e)),
    }
}

pub struct ControlInfo {
  cm: controls::ControlMap,
  guijson: String,
}

pub fn startserver(guifilename: &str, ip: &str, http_port: &str, websockets_port: &str, htmltemplatefile: Option<&str> ) -> Result<(), Box<std::error::Error> >
{
    let mut http_ip = String::from(ip);
    http_ip.push_str(":");
    http_ip.push_str(&http_port);

    let mut websockets_ip = String::from(ip);
    websockets_ip.push_str(":");
    websockets_ip.push_str(&websockets_port);
    
    let htmltemplate = {  
      match htmltemplatefile {
        Some(fname) => { 
          try!(load_string(fname))
        }
        None => string_defaults::MAIN_HTML.to_string()
      }
    };
    
    let htmlstring = htmltemplate.replace("<websockets-port>", &websockets_port);

    // println!("{}", htmlstring);

    let guistring = try!(load_string(&guifilename[..]));
    let guival: Value = try!(serde_json::from_str(&guistring[..])); 

    let blah = try!(controls::deserialize_root(&guival));

    println!("title: {} rootcontroltype: {} ", 
      blah.title, blah.root_control.control_type());
    println!("controls: {:?}", blah.root_control);

    // from control tree, make a map of ids->controls.
    let mapp = controls::make_control_map(&*blah.root_control);
    let guijson = guistring.clone();

    let ci = ControlInfo { cm: mapp, guijson: guijson };

    let cmshare = Arc::new(Mutex::new(ci));
    let wscmshare = cmshare.clone();
    // let oscsocket = try!(UdpSocket::bind(&oscrecvip[..]));
    // for sending, bind to this.  if we bind to localhost, we can't
    // send messages to other machines.  
    // let oscsendsocket = try!(UdpSocket::bind("0.0.0.0:0"));
    let bc = broadcaster::Broadcaster::new();
    // let wsos = try!(oscsendsocket.try_clone());
    let wsbc = bc.clone();
    // let wsoscsendip = oscsendip.clone();

    /*
    thread::spawn(move || { 
      match oscmain(oscsocket, bc, cmshare) {
        Err(e) => println!("oscmain exited with error: {:?}", e),
        Ok(_) => (),
      }
      }); 
    */

    // Spawn a thread for the websockets handler.
    thread::spawn(move || { 
      match websockets_main(websockets_ip, wscmshare, wsbc) {
        Ok(_) => (),
        Err(e) => println!("error in websockets_main: {:?}", e),
      }
    });


    thread::spawn(move || { 
      // use this thread for the web server.
      Iron::new(move | _: &mut Request| {
          let content_type = "text/html".parse::<Mime>().unwrap();
          Ok(Response::with((content_type, status::Ok, &*htmlstring)))
      }).http(&http_ip[..]);
      // return when the web server dies, if it ever does. 
      });
      
    Ok(())

}

// need to lock the control structs and stuff, refresh them, then send out the 
// updates.

static mut mutie: Mutex<i32> = Mutex::default();

pub fn loadguistring(guistring: &str) -> Result<(), Box<std::error::Error> >
{
  match serde_json::from_str(guistring) { 
    Ok(guival) => { 
      match controls::deserialize_root(&guival) {
        Ok(controltree) => { 
          println!("new control layout recieved!");

          println!("title: {} count: {} ", 
            controltree.title, controltree.root_control.control_type());
          println!("controls: {:?}", controltree.root_control);

          // from control tree, make a map of ids->controls.
          let mapp = controls::make_control_map(&*controltree.root_control);
          let cnm = controls::control_map_to_name_map(&mapp);
/*
          sci.cm = mapp;
          sci.guijson = guistring.to_string();
          bc.broadcast(Message::text(guistring.to_string()));
*/
          Ok(())
        },
        Err(e) => { 
          println!("error reading guiconfig from json: {:?}", e);
          let s = format!("error reading guiconfig json: {:?}", e);
          Err(Box::new(Error::new(ErrorKind::Other, s))) },
      }
    },
    Err(e) => {
      // println!("error reading guiconfig json: {:?}", e);
      let s = format!("error reading guiconfig json: {:?}", e);
      Err(Box::new(Error::new(ErrorKind::Other, s))) },
  }
}

// TODO: refactor to return a (rx/sx) pair for sending, recieving messages.
// library users start the websockets_main and get that pair of things.
// then, can send the various control structs and receive the messages.  
fn websockets_main( ipaddr: String, 
                    ci: Arc<Mutex<ControlInfo>>,
                    broadcaster: broadcaster::Broadcaster)
                  -> Result<(), Box<std::error::Error> >
{
	let server = try!(Server::bind(&ipaddr[..]));

	for connection in server {

    println!("new websockets connection!");
		// Spawn a new thread for each connection.
    
    let sci = ci.clone();
//    let osock = try!(oscsocket.try_clone());
 //   let osend = oscsendip.clone();
    let broadcaster = broadcaster.clone();

    let conn = try!(connection);
    thread::spawn(move || {
      match websockets_client(conn,
                            sci,
                            broadcaster) {
        Ok(_) => (), 
        Err(e) => {
          println!("error in websockets thread: {:?}", e);
          ()
        },
      }
    });
  } 

  Ok(())
}

fn websockets_client(connection: websocket::server::Connection<websocket::stream::WebSocketStream, websocket::stream::WebSocketStream>,
                    ci: Arc<Mutex<ControlInfo>>,
                    mut broadcaster: broadcaster::Broadcaster, 
                    ) -> Result<(), Box<std::error::Error> >
{
  // Get the request
  let request = try!(connection.read_request());
  // Keep the headers so we can check them
  let headers = request.headers.clone(); 
  
  try!(request.validate()); // Validate the request
  
  let mut response = request.accept(); // Form a response
  
  if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
    if protocols.contains(&("rust-websocket".to_string())) {
      // We have a protocol we want to use
      response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
    }
  }
  
  let mut client = try!(response.send()); // Send the response
  
  let ip = try!(client.get_mut_sender()
                  .get_mut()
                  .peer_addr());
  
  println!("Websocket connection from {}", ip);

  // send up the json of the current controls.
  {
    let sci = ci.lock().unwrap();

    let updarray = controls::cm_to_update_array(&sci.cm);
  
    // build json message containing both guijson and the updarray.
    // let updvals = updarray.into_iter().map(|x|{controls::encode_update_message(&x)}).collect();

    let mut updvals = Vec::new();

    for upd in updarray { 
      let um = controls::encode_update_message(&upd);
      updvals.push(um);
    }
   
    let mut guival: Value = try!(serde_json::from_str(&sci.guijson[..]));

    match guival.as_object_mut() {
      Some(obj) => {
        obj.insert("state".to_string(), Value::Array(updvals));
        ()
      },
      None => (),
    }
  
    let guistring = try!(serde_json::ser::to_string(&guival));
    let message = Message::text(guistring);
    try!(client.send_message(&message));
  }
 
  let (sender, mut receiver) = client.split();

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
        try!(sender.send_message(&message));
        println!("Client {} disconnected", ip);
        return Ok(());
      }
      Type::Ping => {
        println!("Message::Ping(data)");
        let message = Message::pong(message.payload);
        let mut sender = sendmeh.lock().unwrap();
        try!(sender.send_message(&message));
      }
      Type::Text => {
        let u8 = message.payload.to_owned();
        let str = try!(std::str::from_utf8(&*u8));
        let jsonval: Value = try!(serde_json::from_str(str));
        let s_um = controls::decode_update_message(&jsonval);
        match s_um { 
          Some(updmsg) => {
            let mut sci  = ci.lock().unwrap();
            let mbcntrl = sci.cm.get_mut(controls::get_um_id(&updmsg));
            match mbcntrl {
              Some(cntrl) => {
                (*cntrl).update(&updmsg);
                broadcaster.broadcast_others(&ip, Message::text(str));
                
                // TODO: callback ftn??  
                /*
                match ctrl_update_to_osc(&updmsg, &**cntrl) { 
                  Ok(v) => match oscsocket.send_to(&v, &oscsendip[..]) {
                    Ok(_) => (),
                    Err(e) => 
                      println!("error sending osc message: {:?}", e),
                    },
                  Err(e) => 
                    println!("error building osc message: {:?}", e),
                };
                */
                
                println!("websockets control update recieved: {:?}", updmsg);
                ()
              },
              None => println!("none"),
            }
          },
          _ => println!("decode_update_message failed on websockets msg: {:?}", message),
        }
      },
      _ => { 
        println!("unknown websockets msg: {:?}", message);
      }
    }
  }

  Ok(())
}

