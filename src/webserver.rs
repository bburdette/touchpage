use actix_web::fs::NamedFile;
use actix_web::http::{Method, StatusCode};
use actix_web::{server, App, HttpRequest, HttpResponse, Result};
use std::path::{Path, PathBuf};
use std::thread;
use string_defaults;

pub fn start<'a>(
  ip: &str,
  http_port: &str,
  websockets_port: &str,
  htmltemplatefile: Option<String>,
  block: bool,
) {
  if block {
    startwebserver_impl(ip, http_port, websockets_port, htmltemplatefile);
  } else {
    // Spawn a thread for the server.
    let ip = ip.to_string();
    let http_port = http_port.to_string();
    let websockets_port = websockets_port.to_string();
    thread::spawn(move || {
      startwebserver_impl(
        ip.as_str(),
        http_port.as_str(),
        websockets_port.as_str(),
        htmltemplatefile,
      );
    });
  }
}

fn startwebserver_impl<'a>(
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
