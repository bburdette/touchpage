use std::thread;
use string_defaults;
use util::load_string;
use toml;
use std::path::{Path, PathBuf};
/*use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
*/
use actix_web::{AsyncResponder, server, App, Result, HttpMessage, HttpRequest, Responder,
                HttpResponse, http};
use actix_web::middleware::Logger;
use actix_web::fs::NamedFile;
use actix_web::http::{Method,StatusCode};
use actix_web::Binary;


/*pub fn startwebserver_iron<'a>(
  ip: &str,
  http_port: &str,
  websockets_port: &str,
  htmltemplatefile: Option<&str>,
) -> Result<(), Box<std::error::Error>> {
  let mut http_ip = String::from(ip);
  http_ip.push_str(":");
  http_ip.push_str(&http_port);

  let mut websockets_ip = String::from(ip);
  websockets_ip.push_str(":");
  websockets_ip.push_str(&websockets_port);

  let htmltemplate = {
    match htmltemplatefile {
      Some(fname) => try!(load_string(fname)),
      None => string_defaults::MAIN_HTML.to_string(),
    }
  };

  let htmlstring = htmltemplate.replace("<websockets-port>", &websockets_port);
  // println!("{}", htmlstring);

  thread::spawn(move || {
    // use this thread for the web server.
    let _ = Iron::new(move |_: &mut Request| {
      let content_type = "text/html".parse::<Mime>().unwrap();
      Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    })
    .http(&http_ip[..]);
    // return when the web server dies, if it ever does.
  });

  Ok(())
}

*/

pub fn startwebserver<'a>(
  ip: &str,
  http_port: &str,
  websockets_port: &str,
  htmltemplatefile: Option<&str>,
) {
  env_logger::init();

  info!("server init!");

  let sys = actix::System::new("whatevs");

  {
    let s = server::new(move || {
      
        App::new()
            .resource(r"/static/{tail:.*}", |r| r.method(Method::GET).f(files))
            .resource(r"/{tail:.*}", |r| r.method(Method::GET).f(mainpage))

    });
 
    s.bind(format!("{}:{}", ip, http_port))
    
  }.expect(format!("Can not bind to port {}", http_port).as_str())
    .start();

  sys.run();
}

/*#[derive(Deserialize, Debug)]
struct Config {
  ip: String,
  port: u16,
  uid: Option<String>,
  pwd: Option<String>,
  tlskey: Option<String>,
  tlscerts: Option<String>,
  redirectport: Option<u16>,
  redirectdomain: Option<String>,
}

fn defcon() -> Config {
  Config {
    ip: "127.0.0.1".to_string(),
    port: 8000,
    uid: None,
    pwd: None,
    tlskey: None,
    tlscerts: None,
    redirectport: None,
    redirectdomain: None,
  }
}


fn load_config() -> Config {
  match load_string("config.toml") {
    Err(e) => {
      error!("error loading config.toml: {:?}", e);
      defcon()
    }
    Ok(config_str) => {
      match toml::from_str(config_str.as_str()) {
        Ok(c) => c,
        Err(e) => {
          error!("error loading config.toml: {:?}", e);
          defcon()
        }
      }
    }
  }
}

*//// simple index handler
fn mainpage(req: &HttpRequest) -> Result<HttpResponse> {
    info!("remote ip: {:?}, request:{:?}", req.connection_info().remote(), req);

    match load_string("static/index.html") { 
      Ok(s) => {
        // response
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(s))
      }
      Err(e) => Err(e.into())
    }
}

fn files(req: &HttpRequest) -> Result<NamedFile> {
  let path: PathBuf = req.match_info().query("tail")?;
  info!("files: {:?}", path); 
  let stpath = Path::new("static/").join(path);
  Ok(NamedFile::open(stpath)?)
}


