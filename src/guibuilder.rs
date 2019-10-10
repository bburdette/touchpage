use controls::{Button, Control, Label, Orientation, Root, Sizer, Slider, XY};
use serde_json::value::Value;
use failure::err_msg;
use failure::Error as FError;
use std::collections::BTreeMap;
use std::convert::TryInto;

pub enum Color {
  Fill,
  Text,
  Pressed,
  Unpressed,
}

pub struct Gui {
  pub title: String,
  pub root_control: Option<Box<dyn Control>>,
  sizerstack: Vec<Vec<i32>>,
  colors: BTreeMap<String, String>,
}

impl Gui {
  pub fn new_gui(title: String) -> Gui {
    Gui {
      title: title,
      root_control: None,
      sizerstack: Vec::new(),
      colors: BTreeMap::new(),
    }
  }
  // one way function!
  pub fn to_root(self) -> Result<Root, FError> {
    match self.root_control {
      Some(rc) => {
        if self.sizerstack.is_empty() {
          let mut colors = BTreeMap::new();
          for (key,value) in self.colors {
            colors.insert(key, Value::String(value)); 
          };
          
          Ok(Root {
            title: self.title.clone(),
            root_control: rc,
            colors: colors,
          })
        } else {
          Err(err_msg("there are still incomplete sizers!"))
        }
      }
      None => Err(err_msg("no controls in the gui yet!")),
    }
  }

  pub fn next_id(&self) -> Result<Vec<i32>, FError> {
    match &self.root_control {
      Some(rc) => match self.sizerstack.last() {
        Some(id) => match get_control(id, &**rc) {
          Some(c) => match c.sub_controls() {
            Some(sc) => {
              let mut nid = id.clone();
              nid.push(sc.len().try_into().unwrap());
              Ok(nid)
            }
            None => Err(err_msg(format!("element is not a sizer: {:?}", id))),
          },
          None => Err(err_msg(format!("element not found: {:?}", id))),
        },
        None => Err(err_msg(
          "no active sizer and gui is not empty; can't assign a control id!",
        )),
      },
      None => Ok(Vec::new()),
    }
  }

  pub fn add_control(&mut self, control: Box<dyn Control>) -> Result<&mut Gui, FError> {
    match &mut self.root_control {
      None => {
        // root level controls have an empty id - since there's only one control.
        self.root_control = Some(control);
        Ok(self)
      }

      Some(rootcontrol) =>
      // there's already a root control.  is there a sizer in the stack?
      {
        match self.sizerstack.last() {
          None => Err(err_msg("no active sizer, can't add new element!")),
          Some(id) => {
            match get_control_mut(id, &mut **rootcontrol) {
              Some(c) => c.add_control(control),
              None => (),
            }
            Ok(self)
          }
        }
      }
    }
  }

  pub fn add_button(&mut self, name: String, label: Option<String>) -> Result<&mut Gui, FError> {
    let newbutton = Box::new(Button {
      control_id: self.next_id()?,
      name: String::from(name),
      label: label,
      pressed: false,
    });
    self.add_control(newbutton)
  }

  pub fn add_slider(
    &mut self,
    name: String,
    label: Option<String>,
    orientation: Orientation,
  ) -> Result<&mut Gui, FError> {
    let newslider = Box::new(Slider {
      control_id: self.next_id()?,
      name: String::from(name),
      label: label,
      location: 0.5,
      pressed: false,
      orientation: orientation,
    });
    self.add_control(newslider)
  }
  pub fn add_xy(&mut self, name: String, label: Option<String>) -> Result<&mut Gui, FError> {
    let newxy = Box::new(XY {
      control_id: self.next_id()?,
      name: String::from(name),
      label: label,
      location: (0.5, 0.5),
      pressed: false,
    });
    self.add_control(newxy)
  }

  pub fn add_label(&mut self, name: String, label: String) -> Result<&mut Gui, FError> {
    let newlabel = Box::new(Label {
      control_id: self.next_id()?,
      name: name,
      label: label,
    });
    self.add_control(newlabel)
  }

  pub fn add_sizer(&mut self, orientation: Orientation) -> Result<&mut Gui, FError> {
    let id = self.next_id()?;
    let newsizer = Box::new(Sizer {
      control_id: id.clone(),
      controls: Vec::new(),
      orientation: orientation,
    });
    self.add_control(newsizer);
    self.sizerstack.push(id);
    Ok(self)
  }

  pub fn end_sizer(&mut self) -> Result<&mut Gui, FError> {
    if self.sizerstack.is_empty() {
      Err(err_msg(
        "mismatched end_sizer() call; sizer stack is empty!",
      ))
    } else {
      self.sizerstack.pop();
      Ok(self)
    }
  }

  pub fn set_color(&mut self, color: Color, hexstring: &str) -> &mut Gui {
    let cs = match color {
      Color::Fill => "fillColor",
      Color::Text => "textColor",
      Color::Pressed => "pressedColor",
      Color::Unpressed => "unpressedColor",
    };

    self.colors.insert(cs.to_string(), hexstring.to_string());
    self
  }
}

fn get_control<'a>(id: &Vec<i32>, control: &'a dyn Control) -> Option<&'a dyn Control> {
  // iterate through all controls, looking for our id.
  if *id == *control.control_id() {
    Some(control)
  } else {
    match control.sub_controls() {
      Some(cs) => {
        let mut ret = None;
        for c in cs.iter() {
          match get_control(id, &**c) {
            Some(cr) => ret = Some(cr),
            None => (),
          };
        }
        ret
      }
      None => None,
    }
  }
}

fn get_control_mut<'a>(id: &Vec<i32>, control: &'a mut dyn Control) -> Option<&'a mut dyn Control> {
  // iterate through all controls, looking for our id.
  if *id == *control.control_id() {
    Some(control)
  } else {
    match control.mut_sub_controls() {
      Some(cs) => {
        let mut ret = None;
        for c in cs.iter_mut() {
          match get_control_mut(id, &mut **c) {
            Some(cr) => ret = Some(cr),
            None => (),
          };
        }
        ret
      }
      None => None,
    }
  }
}
