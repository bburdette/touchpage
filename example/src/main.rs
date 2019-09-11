extern crate touchpage;

use touchpage::control_nexus::PrintUpdateMsg;
use touchpage::websocketserver::startserver;
use touchpage::webserver::startwebserver;

fn main() {
  // println!("Hello, world!");

  let meh = PrintUpdateMsg {};

  startwebserver("localhost", "8000", Some(touchpage::string_defaults::MAIN_HTML));
  startserver(GUI, Box::new(meh), "localhost", "8500", false);
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
