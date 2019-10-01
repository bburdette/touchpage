use controls::{Button, Control, Label, Root, Sizer, Slider};
use failure::err_msg;
use failure::Error as FError;

pub struct Gui {
  pub title: String,
  pub root_control: Option<Box<dyn Control>>,
  sizerstack: Vec<Vec<i32>>,
}

fn get_control<'a>(id: &Vec<i32>, control: &'a mut dyn Control) -> Option<&'a mut dyn Control> {
  // iterate through all controls, looking for our id.
  if *id == *control.control_id() {
    Some(control)
  } else {
    match control.mut_sub_controls() {
      Some(cs) => {
        let mut reet = None;
        for c in cs.iter_mut() {
          match get_control(id, &mut **c) {
            Some(cr) => reet = Some(cr),
            None => (),
          };
        }
        reet
      }
      None => None,
    }
  }
}

impl Gui {
  fn add_control(&mut self, control: Box<dyn Control>) -> Result<&Gui, FError> {
    match &mut self.root_control {
      None => {
        self.root_control = Some(control);
        Ok(self)
      }

      Some(rootcontrol) => match self.sizerstack.last() {
        None => Err(err_msg("no active sizer, can't add new element!")),
        Some(id) => {
          match get_control(id, &mut **rootcontrol) {
            Some(c) => c.add_control(control),
            None => (),
          }
          Ok(self)
        }
      },
    }
  }

  fn add_button(&mut self, name: String, label: Option<String>) -> Result<&Gui, FError> {
    let newbutton = Box::new(Button {
      control_id: Vec::new(),
      name: String::from(name),
      label: label,
      pressed: false,
    });
    self.add_control(newbutton)
  }

  fn add_slider(&mut self, name: String, label: Option<String>) -> Result<&Gui, FError> {
    let newslider = Box::new(Slider {
      control_id: Vec::new(),
      name: String::from(name),
      label: label,
      location: 0.5,
      pressed: false,
    });
    self.add_control(newslider)
  }

  fn add_label(&mut self, name: String, label: String) -> Result<&Gui, FError> {
    let newlabel = Box::new(Label {
      control_id: Vec::new(),
      name: name,
      label: label,
    });
    self.add_control(newlabel)
  }

  fn add_sizer(&mut self) -> Result<&Gui, FError> {
    let newsizer = Box::new(Sizer {
      control_id: Vec::new(),
      controls: Vec::new(),
    });
    self.add_control(newsizer)
  }
}
