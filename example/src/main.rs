extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use touchpage::control_nexus::PrintUpdateMsg;
use touchpage::controls;
use touchpage::controls::Orientation::{Horizontal, Vertical};
use touchpage::guibuilder as G;
use touchpage::json as J;
use touchpage::webserver::startwebserver;
use touchpage::websocketserver::startserver;

fn main() {
  // println!("Hello, world!");

  let meh = PrintUpdateMsg {};

  let mbhtml = match load_string("index.html") {
    Ok(html) => Some(html),
    _ => None,
  };

  let rootv: Result<String, FError> = build_gui()
    .and_then(|gui| gui.to_root())
    .map(|root| J::serialize_root(&root))
    .and_then(|rootv| serde_json::to_string_pretty(&rootv).map_err(|_| err_msg("uh oh")));
    // .and_then(|st| write_string(st.as_str(), "json.out"));

  let gooey = match rootv {
    Ok(s) => s,
    Err(e) => {
      println!("error loading controls! {}", e);
      GUI.to_string()
      },
  };

  match startserver(gooey.as_str(), Box::new(meh), "localhost", "9001", false) {
    Ok(_) => (),
    Err(e) => println!("error starting websocket server: {},", e),
  }
  startwebserver("localhost", "8000", "9001", mbhtml, true);
}

fn build_gui() -> Result<G::Gui, FError> {
  let mut gui = G::Gui::new_gui("test".to_string());
  gui
    .add_sizer(Vertical)?
    .add_label("lb3".to_string(), "blah".to_string())?
    .add_label("lb0".to_string(), "blah1".to_string())?
    .add_label("lb1".to_string(), "blah2".to_string())?
    .add_label("lb2".to_string(), "blah4444".to_string())?
    .add_sizer(Horizontal)?
    .add_button("b1".to_string(), None)?
    .add_slider("hs1".to_string(), None, Horizontal)?
    .add_slider("hs2".to_string(), None, Horizontal)?
    .add_slider("hs3".to_string(), None, Horizontal)?
    .add_slider("hs4".to_string(), None, Horizontal)?
    .end_sizer()?
    .add_xy("xy".to_string(), Some("xycontrol".to_string()))?
    .end_sizer()?;
  Ok(gui)
}

const GUI: &'static str = r##"
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
