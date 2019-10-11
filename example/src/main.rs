extern crate touchpage;

use failure::err_msg;
use failure::Error as FError;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use touchpage::control_nexus::{ControlNexus, ControlUpdateProcessor};
use touchpage::control_updates as cu;
use touchpage::controls::Orientation::{Horizontal, Vertical};
use touchpage::guibuilder as G;
use touchpage::json as J;
use touchpage::webserver;
use touchpage::websocketserver;

fn main() {
  // when developing the elm code, I like to load the page from a file.  It requires
  // restarting the server when the file changes.  At least you don't have to recompile
  // the server though.
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

  let guijson = match rootv {
    Ok(s) => s,
    Err(e) => {
      println!("error loading controls! {}", e);
      ERRORUI.to_string()
    }
  };

  // see the json created above, if you want.
  // write_string(guijson.as_str(), "json.txt");

  // the 'ControlUpdateProcessor' does something when an update message comes in.
  let cup = ExampleUpdate {};

  // start the websocket server.  mandatory for receiving control messages.
  match websocketserver::start(guijson.as_str(), Box::new(cup), "0.0.0.0", "9001", false) {
    Ok(_) => (),
    Err(e) => println!("error starting websocket server: {},", e),
  }

  // start the webserver.  not necessary if you want to serve up the html with your
  // own server.
  webserver::start("0.0.0.0", "8000", "9001", mbhtml, true);
}

pub struct ExampleUpdate {}

impl ControlUpdateProcessor for ExampleUpdate {
  fn on_update_received(&mut self, update: &cu::UpdateMsg, cn: &mut ControlNexus) -> () {
    // println!("control update: {:?}", update);
    match update {
      cu::UpdateMsg::Slider {
        control_id,
        location,
        ..
      } => {
        cn.get_name(control_id).map(|name| {
          if name == "hs1" {
            location.map(|loc| cn.update_label("lb0", format!("{}", loc).as_str()));
          } else if name == "hs4" {
            location.map(|loc| cn.update_label("lb1", format!("{}", loc).as_str()));
          };
        });
        ()
      }
      cu::UpdateMsg::Button { control_id, .. } => {
        cn.get_name(control_id).map(|name| {
          if name == "b0" {
            cn.update_label("lb0", "");
          } else if name == "b1" {
            cn.update_label("lb1", "");
          };
        });
        ()
      }

      _ => (),
    };
  }
}

// build the UI with a series of rust function calls.
fn build_gui() -> Result<G::Gui, FError> {
  let mut gui = G::Gui::new_gui("test".to_string());
  gui
    .add_sizer(Vertical, Some(vec![0.1, 2.5, 0.4]))?
    .add_sizer(Horizontal, None)?
    .add_label("lb0".to_string(), "sliderpos1".to_string())?
    .add_label("lb1".to_string(), "sliderpos2".to_string())?
    .end_sizer()?
    .add_sizer(Horizontal, None)?
    .add_button("b0".to_string(), None)?
    .add_slider("hs1".to_string(), None, Vertical)?
    .add_slider("hs2".to_string(), None, Vertical)?
    .add_slider("hs3".to_string(), None, Vertical)?
    .add_slider("hs4".to_string(), None, Vertical)?
    .add_button("b1".to_string(), None)?
    .end_sizer()?
    .add_sizer(Horizontal, None)?
    .add_xy("xyleft".to_string(), Some("xy1".to_string()))?
    .add_xy("xyright".to_string(), Some("xy2".to_string()))?
    .end_sizer()?
    .end_sizer()?
    .set_color(G::Color::Controls, "001F00")
    .set_color(G::Color::Text, "1F0000");
  Ok(gui)
}

// you can also specify the controls in json, like so.
const ERRORUI: &'static str = r##"
{
  "title": "test",
  "rootControl": 
    { "type": "label"
       , "name": "lb3"
       , "label": "error loading controls!"
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
