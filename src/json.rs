extern crate serde_json;
extern crate serde;

use serde_json::Value;
use controls::{ Root, Label, Slider, Button, Control, Sizer};
use control_updates as cu;
use string_defaults;

use stringerror;
use std::error::Error;
use std::collections::BTreeMap;

pub fn deserialize_root(data: &Value) -> Result<Box<Root>, Box<Error> >
{
  let obj = try_opt_resbox!(data.as_object(), "json value is not an object!");
  let title = 
    try_opt_resbox!(
      try_opt_resbox!(obj.get("title"), "'title' not found").as_string(), "title is not a string!");

  let rc = try_opt_resbox!(obj.get("rootControl"), "root_control not found");

  let rootcontrol = try!(deserialize_control(Vec::new(), rc)); 

  Ok(Box::new(Root { title: String::from(title), root_control: rootcontrol }))
}

fn deserialize_control(id: Vec<i32>, data: &Value) -> Result<Box<Control>, Box<Error> >
{
  // what's the type?
  let obj = try_opt_resbox!(data.as_object(), "unable to parse control as json");
  let objtype = 
    try_opt_resbox!(try_opt_resbox!(obj.get("type"), "'type' not found").as_string(), "'type' is not a string");

  match objtype {
    "button" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), 
                                                 "'name' not found!").as_string(), 
                                 "'name' is not a string!");
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = try_opt_resbox!(x.as_string(), "'label' is not a string!");
            Some(String::from(s))
            },
          None => None,
          };
      Ok(Box::new(Button { control_id: id.clone()
                         , name: String::from(name)
                         , label: label // String::from(label)
                         , pressed: false }))
    },
    "slider" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), 
                                                 "'name' not found!").as_string(), 
                                 "'name' is not a string!");
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = try_opt_resbox!(x.as_string(), "'label' is not a string!");
            Some(String::from(s))
            },
          None => None,
          };
      Ok(Box::new(Slider { control_id: id.clone()
                         , name: String::from(name)
                         , label: label 
                         , pressed: false
                         , location: 0.5 }))
    },
    "label" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "'name' not found!").as_string(), "'name' is not a string!");
      let label = try_opt_resbox!(try_opt_resbox!(obj.get("label"), "'label' not found!").as_string(), "'label' is not a string!");
      Ok(Box::new(Label { control_id: id.clone(), name: String::from(name), label: label.to_string() }))
    },
    "sizer" => { 
      let controls = 
        try_opt_resbox!(try_opt_resbox!(obj.get("controls"), "'controls' not found").as_array(), "'controls' is not an array");

      let mut controlv = Vec::new();

      // loop through array, makin controls.
      for (i, v) in controls.into_iter().enumerate() {
          let mut id = id.clone();
          id.push(i as i32); 
          let c = try!(deserialize_control(id, v));
          controlv.push(c);
          }
      
      Ok(Box::new(Sizer { control_id: id.clone(), controls: controlv }))
    },
    _ => Err(stringerror::string_box_err("objtype not supported!"))
  }
}

pub fn decode_update_message(data: &Value) -> Option<cu::UpdateMsg> {
  let obj = try_opt!(data.as_object());
  let contype = try_opt!(try_opt!(obj.get("controlType")).as_string());
  let conid = convarrayi32(try_opt!(try_opt!(obj.get("controlId")).as_array()));
  let mbst = obj.get("state").map(|wut| wut.as_string());
   
  match contype {
    "slider" => {
      let location = match obj.get("location").map(|l| l.as_f64())
        { Some(Some(loc)) => Some(loc)
        , _ => None };
      let optst = match mbst 
        { Some(Some("Press")) => Some( cu::SliderState::Pressed ) 
        , Some(Some("Unpress")) => Some( cu::SliderState::Unpressed )
        , _ => None
        };
      let lab = obj.get("label").and_then(|s| s.as_string()).map(|s| String::from(s));
      Some( cu::UpdateMsg::Slider { control_id: conid
                                  , state: optst
                                  , location: location 
                                  , label: lab
                                  } )
      },
    "button" => {
      let optst = match mbst 
        { Some(Some("Press")) => Some( cu::ButtonState::Pressed ) 
        , Some(Some("Unpress")) => Some( cu::ButtonState::Unpressed )
        , _ => None
        };
      let lab = obj.get("label").and_then(|s| s.as_string()).map(|s| String::from(s));
        
      Some( cu::UpdateMsg::Button { control_id: conid
                                  , state: optst 
                                  , label: lab } )
      },
    _ => None
    }
}

fn convi32array(inp: &Vec<i32>) -> Vec<Value> {
  inp.into_iter().map(|x|{Value::I64(*x as i64)}).collect()
}

fn convarrayi32(inp: &Vec<Value>) -> Vec<i32> {
  inp.into_iter().map(|x|{x.as_i64().unwrap() as i32}).collect()
}

pub fn encode_update_message(um: &cu::UpdateMsg) -> Value { 
  match um { 
    &cu::UpdateMsg::Button { control_id: ref cid 
                           , state: ref opt_state
                           , label: ref opt_label } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("button")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      if let &Some(ref st) = opt_state { 
        btv.insert(String::from("state"), 
          Value::String(String::from( 
            match st { &cu::ButtonState::Pressed => "Press", 
                       &cu::ButtonState::Unpressed => "Unpress", })));
        };
      if let &Some(ref lb) = opt_label { 
        btv.insert(String::from("label"), 
          Value::String(lb.clone()));
        };
      
      Value::Object(btv)
    }, 
    &cu::UpdateMsg::Slider { control_id: ref cid
                           , state: ref opt_state
                           , label: ref opt_label 
                           , location: ref opt_loc } => 
    {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), 
                 Value::String(String::from("slider")));
      btv.insert(String::from("controlId"), 
                 Value::Array(convi32array(cid)));
      if let &Some(ref st) = opt_state { 
        btv.insert(String::from("state"), 
          Value::String(String::from( 
            match st { &cu::SliderState::Pressed => "Press",
                       &cu::SliderState::Unpressed => "Unpress" })));
      };
      if let &Some(loc) = opt_loc { 
        btv.insert(String::from("location"), Value::F64(loc));
      };
      if let &Some(ref lb) = opt_label { 
        btv.insert(String::from("label"), 
          Value::String(lb.clone()));
        };
      
      Value::Object(btv)
    },
    &cu::UpdateMsg::Label { control_id: ref cid, 
                            label: ref labtext } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("label")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("label"), Value::String(labtext.clone()));
      Value::Object(btv)
    }, 
   } 
}
 
pub fn sample_gui_config() -> &'static str {
  string_defaults::SAMPLE_GUI_CONFIG
  }

