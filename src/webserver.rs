use std::thread;
use string_defaults;
use util::load_string;

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
  let config = load_config();
  env_logger::init();

  info!("server init!");

  let sys = actix::System::new("whatevs");

  {
    let s = server::new(move || {
      
        App::new()
            .resource("/public", |r| r.method(Method::POST).f(public))
            .resource("/model", |r| r.method(Method::POST).f(model))
            .resource(r"/static/{tail:.*}", |r| r.method(Method::GET).f(files))
            .resource("/favicon.ico", |r| r.method(Method::GET).f(favicon))
            .resource("/sitemap.txt", |r| r.method(Method::GET).f(sitemap))
            .resource(r"/{tail:.*}", |r| r.method(Method::GET).f(mainpage))

    });
 
    match optbuilder {
      Some(builder) => s.bind_ssl(format!("{}:{}", config.ip, config.port), builder),
      None => s.bind(format!("{}:{}", config.ip, config.port)),
    }
  }.expect(format!("Can not bind to port {}", config.port).as_str())
    .start();

  // do some http redirecting?
  let _ = match (config.redirectport, config.redirectdomain) {
    (Some(port), Some(domain)) => {
      Some(
        server::new(move || {
          App::with_state(domain.clone())
            .resource("{all:.*}", |r| {
              r.f(|r| {
                info!("redirect!{}", r.path());
                // info!("fmt:{}", format!("{}{}&{}", r.state(), r.path(), r.query_string()));
                HttpResponse::Found()
                  .header(http::header::LOCATION, format!("{}{}?{}", r.state(), r.path(), r.query_string()))
                  .finish()
              })
            })
            .middleware(Logger::default())
            .middleware(Logger::new("REDIRECTOR: %a %{User-Agent}i"))

        }).bind(format!("{}:{}", config.ip, port))
          .expect(format!("Can not bind to port {}", port).as_str())
          .start(),
      )
    }
    _ => None,
  };

  sys.run();
}

