extern crate serde_json;
extern crate serde;

use serde_json::Value;
use controls::{ Root, Label, Slider, Button, Control, Sizer};
use control_updates as cu;
use string_defaults;
use std::collections::BTreeMap;
use failure::Error as FError;
use failure::err_msg;

pub fn deserialize_root(data: &Value) -> Result<Box<Root>, FError >
{
  let obj = data.as_object().ok_or(err_msg("root is not an object"))?;
  let t_o = obj.get("title").ok_or(err_msg("'title' not found"))?;
  let title = t_o.as_string().ok_or(err_msg("'title' not a string!"))?;
  let rc = obj.get("rootControl").ok_or(err_msg("'roolControl' not found"))?;

  let rootcontrol = deserialize_control(Vec::new(), rc)?; 

  Ok(Box::new(Root { title: String::from(title), root_control: rootcontrol }))
}

fn get_string<'a>(data: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a str, FError>
{
  data.get(name).ok_or(err_msg(format!("{} not found", name)))?
     .as_string().ok_or(err_msg(format!("{} not a string", name)))
}

fn deserialize_control(id: Vec<i32>, data: &Value) -> Result<Box<Control>, FError >
{
  // what's the type?
  let obj = data.as_object().ok_or(err_msg("control is not a valid json object"))?;
  let objtype = get_string(obj, "type")?; 

  match objtype {
    "button" => { 
      let name = get_string(obj, "name")?; 
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = x.as_string().ok_or(err_msg("'label' is not a string!"))?;
            Some(String::from(s))
            },
          None => None,
          };
      Ok(Box::new(Button { control_id: id.clone()
                         , name: String::from(name)
                         , label: label
                         , pressed: false }))
    },
    "slider" => { 
      let name = get_string(obj, "name")?; 
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = x.as_string().ok_or(err_msg("'label' is not a string!"))?;
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
      let name = get_string(obj, "name")?; 
      let label = get_string(obj, "label")?; 
      Ok(Box::new(Label { control_id: id.clone(), name: String::from(name), label: label.to_string() }))
    },
    "sizer" => { 
      let controls = obj.get("controls").ok_or(err_msg("'controls' not found"))?
                        .as_array().ok_or(err_msg("'controls' is not an array"))?;

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
    _ => Err(err_msg(format!("objtype '{}' not supported!", objtype)))
  }
}

pub fn decode_update_message(data: &Value) -> Option<cu::UpdateMsg> {
  let obj = data.as_object()?;
  let contype = obj.get("controlType")?.as_string()?;
  let conid = convarrayi32(obj.get("controlId")?.as_array()?);
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

