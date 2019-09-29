extern crate touchpage;

use failure::Error as FError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use touchpage::control_nexus::PrintUpdateMsg;
use touchpage::webserver::startwebserver;
use touchpage::websocketserver::startserver;

fn main() {
  // println!("Hello, world!");

  let meh = PrintUpdateMsg {};

  let mbhtml = match load_string("index.html") {
    Ok(html) => Some(html),
    _ => None,
  };

  println!("before websocketserver");
  startserver(GUI, Box::new(meh), "localhost", "9001", false);
  println!("before webserver");
  startwebserver("localhost", "8000", "9001", mbhtml);
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
       , "label": "blah"
       }
      ,{ "type": "button"
       , "name": "b1"
       }
      ,{ "type": "slider"
       , "orientation": "horizontal"
       , "name": "hs2"
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
