extern crate serde;
extern crate serde_json;

use control_updates as cu;
use controls::{
  Button, Control, Label,
  Orientation::{Horizontal, Vertical},
  Root, Sizer, Slider, XY,
};
use failure::err_msg;
use failure::Error as FError;
use serde_json::value::Value;
use std::collections::BTreeMap;
use string_defaults;

pub fn deserialize_root(data: &Value) -> Result<Box<Root>, FError> {
  let obj = data.as_object().ok_or(err_msg("root is not an object"))?;
  let t_o = obj.get("title").ok_or(err_msg("'title' not found"))?;
  let title = t_o.as_string().ok_or(err_msg("'title' not a string!"))?;
  let rc = obj
    .get("rootControl")
    .ok_or(err_msg("'rootControl' not found"))?;

  let mut colors = BTreeMap::new();
  let mut insertcolor = |colorname : &str| {
          obj.get(colorname).map(
            |cs|
              cs.as_string().map(|_cstr| colors.insert(colorname.to_string(),cs.clone())))};
  insertcolor("controlsColor");
  insertcolor("labelsColor");
  insertcolor("textColor");
  insertcolor("pressedColor");
  insertcolor("unpressedColor");
  insertcolor("backgroundColor");

  println!("read in colors! {:?}", colors);

  let rootcontrol = deserialize_control(Vec::new(), rc)?;

  Ok(Box::new(Root {
    title: String::from(title),
    root_control: rootcontrol,
    colors: colors,
  }))
}

pub fn serialize_root(root: &Root) -> Value {
  let mut btv = root.colors.clone();
  btv.insert(String::from("title"), Value::String(root.title.clone()));
  btv.insert(String::from("rootControl"), root.root_control.as_json());

  Value::Object(btv)
}

fn get_string<'a>(data: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a str, FError> {
  data
    .get(name)
    .ok_or(err_msg(format!("{} not found", name)))?
    .as_string()
    .ok_or(err_msg(format!("{} not a string", name)))
}

fn deserialize_control(id: Vec<i32>, data: &Value) -> Result<Box<dyn Control>, FError> {
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
      let orientation = match obj.get("orientation") {
        Some(val) => match val.to_string().as_str() {
          "vertical" => Vertical,
          "horizontal" => Horizontal,
          _ => Vertical,
        },
        _ => Vertical,
      };
      Ok(Box::new(Slider {
        control_id: id.clone(),
        name: String::from(name),
        label: label,
        pressed: false,
        location: 0.5,
        orientation: orientation,
      }))
    }
    "xy" => {
      let name = get_string(obj, "name")?;
      let label = match obj.get("label") {
        Some(x) => {
          let s = x.as_string().ok_or(err_msg("'label' is not a string!"))?;
          Some(String::from(s))
        }
        None => None,
      };
      Ok(Box::new(XY {
        control_id: id.clone(),
        name: String::from(name),
        label: label,
        pressed: false,
        location: (0.5, 0.5),
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

      let orientation = match obj.get("orientation") {
        Some(val) => match val.to_string().as_str() {
          "vertical" => Vertical,
          "horizontal" => Horizontal,
          _ => Vertical,
        },
        _ => Vertical,
      };

      let mut proportions = Vec::new();
      obj.get("proportions").and_then(|val| {
        val.as_array().map(|vals| {
          for v in vals {
            serde_json::from_value(v.clone())
              .map(|f| proportions.push(f))
              .unwrap();
          }
        })
      });

      Ok(Box::new(Sizer {
        control_id: id.clone(),
        controls: controlv,
        orientation: orientation,
        proportions: if proportions.is_empty() {
          None
        } else {
          Some(proportions)
        },
      }))
    }
    _ => Err(err_msg(format!("objtype '{}' not supported!", objtype))),
  }
}

pub fn decode_update_message(data: &Value) -> Option<cu::UpdateMsg> {
  let obj = data.as_object()?;
  let contype = obj.get("controlType")?.as_string()?;
  let conid = convarrayi32(obj.get("controlId")?.as_array()?);
  let mbst = obj.get("state").map(|wut| wut.as_string());

  match contype {
    "slider" => {
      let location = obj.get("location").and_then(|l| l.as_f64());
      let optst = match mbst {
        Some(Some("Press")) => Some(cu::PressState::Pressed),
        Some(Some("Unpress")) => Some(cu::PressState::Unpressed),
        _ => None,
      };
      let lab = obj
        .get("label")
        .and_then(|s| s.as_string())
        .map(|s| String::from(s));
      Some(cu::UpdateMsg::Slider {
        control_id: conid,
        state: optst,
        location: location,
        label: lab,
      })
    }
    "xy" => {
      let location: Option<(f32, f32)> =
        obj
          .get("location")
          .and_then(|l| l.as_array())
          .and_then(|a| {
            a.get(0).and_then(|e1| {
              a.get(1).and_then(|e2| {
                serde_json::from_value(e1.clone()).ok().and_then(|e1i| {
                  serde_json::from_value(e2.clone())
                    .ok()
                    .map(|e2i| (e1i, e2i))
                })
              })
            })
          });

      let optst = match mbst {
        Some(Some("Press")) => Some(cu::PressState::Pressed),
        Some(Some("Unpress")) => Some(cu::PressState::Unpressed),
        _ => None,
      };
      let lab = obj
        .get("label")
        .and_then(|s| s.as_string())
        .map(|s| String::from(s));
      Some(cu::UpdateMsg::XY {
        control_id: conid,
        state: optst,
        location: location,
        label: lab,
      })
    }
    "button" => {
      let optst = match mbst {
        Some(Some("Press")) => Some(cu::PressState::Pressed),
        Some(Some("Unpress")) => Some(cu::PressState::Unpressed),
        _ => None,
      };
      let lab = obj
        .get("label")
        .and_then(|s| s.as_string())
        .map(|s| String::from(s));

      Some(cu::UpdateMsg::Button {
        control_id: conid,
        state: optst,
        label: lab,
      })
    }
    _ => None,
  }
}

fn convi32array(inp: &Vec<i32>) -> Vec<Value> {
  inp.into_iter().map(|x| Value::I64(*x as i64)).collect()
}

fn convarrayi32(inp: &Vec<Value>) -> Vec<i32> {
  inp
    .into_iter()
    .map(|x| x.as_i64().unwrap() as i32)
    .collect()
}

pub fn encode_update_message(um: &cu::UpdateMsg) -> Value {
  match um {
    &cu::UpdateMsg::Button {
      control_id: ref cid,
      state: ref opt_state,
      label: ref opt_label,
    } => {
      let mut btv = BTreeMap::new();
      btv.insert(
        String::from("controlType"),
        Value::String(String::from("button")),
      );
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      if let &Some(ref st) = opt_state {
        btv.insert(
          String::from("state"),
          Value::String(String::from(match st {
            &cu::PressState::Pressed => "Press",
            &cu::PressState::Unpressed => "Unpress",
          })),
        );
      };
      if let &Some(ref lb) = opt_label {
        btv.insert(String::from("label"), Value::String(lb.clone()));
      };

      Value::Object(btv)
    }
    &cu::UpdateMsg::Slider {
      control_id: ref cid,
      state: ref opt_state,
      label: ref opt_label,
      location: ref opt_loc,
    } => {
      let mut btv = BTreeMap::new();
      btv.insert(
        String::from("controlType"),
        Value::String(String::from("slider")),
      );
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      if let &Some(ref st) = opt_state {
        btv.insert(
          String::from("state"),
          Value::String(String::from(match st {
            &cu::PressState::Pressed => "Press",
            &cu::PressState::Unpressed => "Unpress",
          })),
        );
      };
      if let &Some(loc) = opt_loc {
        btv.insert(String::from("location"), Value::F64(loc));
      };
      if let &Some(ref lb) = opt_label {
        btv.insert(String::from("label"), Value::String(lb.clone()));
      };

      Value::Object(btv)
    }
    &cu::UpdateMsg::XY {
      control_id: ref cid,
      state: ref opt_state,
      label: ref opt_label,
      location: ref opt_loc,
    } => {
      let mut btv = BTreeMap::new();
      btv.insert(
        String::from("controlType"),
        Value::String(String::from("xy")),
      );
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      if let &Some(ref st) = opt_state {
        btv.insert(
          String::from("state"),
          Value::String(String::from(match st {
            &cu::PressState::Pressed => "Press",
            &cu::PressState::Unpressed => "Unpress",
          })),
        );
      };
      if let &Some((locx, locy)) = opt_loc {
        btv.insert(
          String::from("location"),
          Value::Array(vec![
            serde_json::to_value(&locx),
            serde_json::to_value(&locy),
          ]),
        );
      };
      if let &Some(ref lb) = opt_label {
        btv.insert(String::from("label"), Value::String(lb.clone()));
      };

      Value::Object(btv)
    }
    &cu::UpdateMsg::Label {
      control_id: ref cid,
      label: ref labtext,
    } => {
      let mut btv = BTreeMap::new();
      btv.insert(
        String::from("controlType"),
        Value::String(String::from("label")),
      );
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("label"), Value::String(labtext.clone()));
      Value::Object(btv)
    }
  }
}

pub fn sample_gui_config() -> &'static str {
  string_defaults::SAMPLE_GUI_CONFIG
}
