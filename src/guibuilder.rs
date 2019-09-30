use controls::{Button, Control, Label, Root, Sizer, Slider};
use failure::err_msg;
use failure::Error as FError;
use serde_json::Value;

pub struct gui<'a> {
  pub title: String,
  pub root_control: Option<Box<dyn Control>>,
  sizerstack: Vec<&'a Sizer>,
}


impl gui<'_> {
  fn add_button(&mut self, name: String, label: Option<String>) -> Result<&gui, FError> {
let wat = [1,2,3];
    let newbutton = Box::new(Button {
      control_id: Vec::new(),
      name: String::from(name),
      label: label,
      pressed: false,
    });
    match self.root_control {
      None => {
        self.root_control = Some(newbutton);
        Ok(self)
      }

      Some(_) => match self.sizerstack.last_mut() {
        None => Err(err_msg("no active sizer, can't add Button element!")),
        Some(mut s) => {
          // add new elt to sizer.
          // *s.control_id = wat[..];
          // s.controls.push(newbutton);
          Ok(self)
        }
      },
    }
  }
}

/*
fn deserialize_control(id: Vec<i32>, data: &Value) -> Result<Box<Control>, FError> {
  // what's the type?
  let obj = data
    .as_object()
    .ok_or(err_msg("control is not a valid json object"))?;
  let objtype = get_string(obj, "type")?;

  match objtype {
    "button" => {
      let name = get_string(obj, "name")?;
      let label = match obj.get("label") {
        Some(x) => {
          let s = x.as_string().ok_or(err_msg("'label' is not a string!"))?;
          Some(String::from(s))
        }
        None => None,
      };
      Ok(Box::new(Button {
        control_id: id.clone(),
        name: String::from(name),
        label: label,
        pressed: false,
      }))
    }
    "slider" => {
      let name = get_string(obj, "name")?;
      let label = match obj.get("label") {
        Some(x) => {
          let s = x.as_string().ok_or(err_msg("'label' is not a string!"))?;
          Some(String::from(s))
        }
        None => None,
      };
      Ok(Box::new(Slider {
        control_id: id.clone(),
        name: String::from(name),
        label: label,
        pressed: false,
        location: 0.5,
      }))
    }
    "label" => {
      let name = get_string(obj, "name")?;
      let label = get_string(obj, "label")?;
      Ok(Box::new(Label {
        control_id: id.clone(),
        name: String::from(name),
        label: label.to_string(),
      }))
    }
    "sizer" => {
      let controls = obj
        .get("controls")
        .ok_or(err_msg("'controls' not found"))?
        .as_array()
        .ok_or(err_msg("'controls' is not an array"))?;

      let mut controlv = Vec::new();

      // loop through array, makin controls.
      for (i, v) in controls.into_iter().enumerate() {
        let mut id = id.clone();
        id.push(i as i32);
        let c = try!(deserialize_control(id, v));
        controlv.push(c);
      }

      Ok(Box::new(Sizer {
        control_id: id.clone(),
        controls: controlv,
      }))
    }
    _ => Err(err_msg(format!("objtype '{}' not supported!", objtype))),
  }
}*/
