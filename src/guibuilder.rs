use controls::{Button, Control, Label, Root, Sizer, Slider};
use failure::err_msg;
use failure::Error as FError;
use std::convert::TryInto;

pub struct Gui {
  pub title: String,
  pub root_control: Option<Box<dyn Control>>,
  sizerstack: Vec<Vec<i32>>,
}

impl Gui {
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

  pub fn add_control(&mut self, control: Box<dyn Control>) -> Result<&Gui, FError> {
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

  pub fn add_button(&mut self, name: String, label: Option<String>) -> Result<&Gui, FError> {
    let newbutton = Box::new(Button {
      control_id: self.next_id()?,
      name: String::from(name),
      label: label,
      pressed: false,
    });
    self.add_control(newbutton)
  }

  pub fn add_slider(&mut self, name: String, label: Option<String>) -> Result<&Gui, FError> {
    let newslider = Box::new(Slider {
      control_id: self.next_id()?,
      name: String::from(name),
      label: label,
      location: 0.5,
      pressed: false,
    });
    self.add_control(newslider)
  }

  pub fn add_label(&mut self, name: String, label: String) -> Result<&Gui, FError> {
    let newlabel = Box::new(Label {
      control_id: self.next_id()?,
      name: name,
      label: label,
    });
    self.add_control(newlabel)
  }

  pub fn add_sizer(&mut self) -> Result<&Gui, FError> {
    let newsizer = Box::new(Sizer {
      control_id: self.next_id()?,
      controls: Vec::new(),
    });
    self.add_control(newsizer)
  }

  pub fn end_sizer(&mut self) -> Result<&Gui, FError> {
    if self.sizerstack.is_empty() {
      Err(err_msg(
        "mismatched end_sizer() call; sizer stack is empty!",
      ))
    } else {
      self.sizerstack.pop();
      Ok(self)
    }
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
