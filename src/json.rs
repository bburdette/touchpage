extern crate serde_json;
extern crate serde;

use serde_json::Value;
use controls::{ Root, Label, Slider, Button, Control, Sizer};

use stringerror;
use control_updates as cu;
use std::error::Error;

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
