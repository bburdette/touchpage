use std::thread;
use string_defaults;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use util::{load_string};

pub fn startwebserver<'a>(
    guistring: &str,
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
