use std::path::{Path, PathBuf};
use std::thread;
use string_defaults;
use toml;
use util::load_string;
use actix_web::fs::NamedFile;
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::Logger;
use actix_web::Binary;
use actix_web::{
  http, server, App, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use std::sync::Arc;

/*use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
*/
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

  info!("touchpage init!");

  let sys = actix::System::new("touchpage");

  let htmltemp = match htmltemplatefile {
    Some(s) => s.to_string(),
    None => string_defaults::MAIN_HTML.to_string(),
  };

  let shtml: String = htmltemp.replace("{{websockets-port}}", &websockets_port);
  {
    let s = server::new(move || {
      let html = shtml.clone();
      App::new()
        .resource(r"/static/{tail:.*}", |r| r.method(Method::GET).f(files))
        .resource(r"/{tail:.*}", move |r| {
          r.method(Method::GET).f(mainpage(html))
        })
    });

    s.bind(format!("{}:{}", ip, http_port))
  }
  .expect(format!("Can not bind to port {}", http_port).as_str())
  .start();

  sys.run();
}

// simple index handler
fn mainpage(html: String) -> impl Fn(&HttpRequest) -> Result<HttpResponse> {
  move |req| mainpage_impl(html.clone(), req)
}

fn mainpage_impl(html: String, req: &HttpRequest) -> Result<HttpResponse> {
  info!("remote ip: {:?}, request:{:?}", req.connection_info().remote(), req);
  Ok(
    HttpResponse::build(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(html.clone()),
  )
}

fn files(req: &HttpRequest) -> Result<NamedFile> {
  let path: PathBuf = req.match_info().query("tail")?;
  info!("files: {:?}", path);
  let stpath = Path::new("static/").join(path);
  Ok(NamedFile::open(stpath)?)
}
