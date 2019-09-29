use actix_web::fs::NamedFile;
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::Logger;
use actix_web::Binary;
use actix_web::{
  http, server, App, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Responder, Result,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use string_defaults;
use toml;
use util::load_string;

pub fn startwebserver<'a>(
  ip: &str,
  http_port: &str,
  websockets_port: &str,
  htmltemplatefile: Option<String>,
) {
  env_logger::init();

  info!("touchpage init!");

  let sys = actix::System::new("touchpage");

  let htmltemp = match htmltemplatefile {
    Some(s) => s,
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
  info!(
    "remote ip: {:?}, request:{:?}",
    req.connection_info().remote(),
    req
  );
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
