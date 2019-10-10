extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use touchpage::control_nexus::PrintUpdateMsg;
use touchpage::controls::Orientation::{Horizontal, Vertical};
use touchpage::guibuilder as G;
use touchpage::json as J;
use touchpage::webserver;
use touchpage::websocketserver;

fn main() {
  // when developing elm, I like to load the page from a file.  it does require
  // restarting the server when the file changes.
/*  let mbhtml = match load_string("index.html") {
    Ok(html) => Some(html),
    _ => None,  
  };
*/
  // html == None means use the precompiled elm/html in string_defaults.rs
  let mbhtml = None;

  let rootv: Result<String, FError> = build_gui()
    .and_then(|gui| gui.to_root())
    .map(|root| J::serialize_root(&root))
    .and_then(|rootv| serde_json::to_string_pretty(&rootv).map_err(|_| err_msg("uh oh")));
  // .and_then(|st| write_string(st.as_str(), "json.out"));

  let guijson = match rootv {
    Ok(s) => s,
    Err(e) => {
      println!("error loading controls! {}", e);
      ERRORUI.to_string()
    }
  };

  // PrintUpdateMsg is a simple "control update processor" that just prints the
  // control update messages as the come in.  Write your own depending on what
  // you want the controls to do on the rust side.
  let meh = PrintUpdateMsg {};

  // start the websocket server.  mandatory for receiving control messages.
  match websocketserver::start(guijson.as_str(), Box::new(meh), "0.0.0.0", "9001", false) {
    Ok(_) => (),
    Err(e) => println!("error starting websocket server: {},", e),
  }

  // start the webserver.  not necessary if you want to serve up the html with your
  // own server.
  webserver::start("0.0.0.0", "8000", "9001", mbhtml, true);
}

// build the UI with a series of rust function calls.
fn build_gui() -> Result<G::Gui, FError> {
  let mut gui = G::Gui::new_gui("test".to_string());
  gui
    .add_sizer(Vertical)?
    .add_sizer(Horizontal)?
    .add_label("lb0".to_string(), "blah".to_string())?
    .add_label("lb3".to_string(), "blah2".to_string())?
    .end_sizer()?
    .add_sizer(Horizontal)?
    .add_button("b1".to_string(), None)?
    .add_slider("hs1".to_string(), None, Vertical)?
    .add_slider("hs2".to_string(), None, Vertical)?
    .add_slider("hs3".to_string(), None, Vertical)?
    .add_slider("hs4".to_string(), None, Vertical)?
    .add_button("b2".to_string(), None)?
    .end_sizer()?
    .add_sizer(Horizontal)?
    .add_xy("xyleft".to_string(), Some("xy1".to_string()))?
    .add_xy("xyright".to_string(), Some("xy2".to_string()))?
    .end_sizer()?
    .end_sizer()?;
  Ok(gui)
}

// you can also specify the controls in json, like so.
const ERRORUI: &'static str = r##"
{
  "title": "test",
  "rootControl": 
    { "type": "sizer",
      "orientation": "vertical",
      "proportions": [0.1,0.3,0.6],
      "controls": [
       { "type": "label"
       , "name": "lb3"
       , "label": "error loading controls!"
       }
      ]
    }
}"##;

pub fn load_string(file_name: &str) -> Result<String, FError> {
  let path = &Path::new(&file_name);
  let mut inf = File::open(path)?;
  let mut result = String::new();
  inf.read_to_string(&mut result)?;
  Ok(result)
}

pub fn write_string(text: &str, file_name: &str) -> Result<(), FError> {
  let path = &Path::new(&file_name);
  let mut inf = File::create(path)?;
  inf.write(text.as_bytes())?;
  Ok(())
}
