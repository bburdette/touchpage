use control_updates as cu;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Debug;

// --------------------------------------------------------
// root obj.  contains controls.
// --------------------------------------------------------

// root is not itself a Control!  although maybe it should be?
#[derive(Debug)]
pub struct Root {
  pub title: String,
  pub root_control: Box<dyn Control>,
  pub colors: BTreeMap<String, Value>,
}

// --------------------------------------------------------
// controls.
// --------------------------------------------------------

pub trait Control: Debug + Send {
  fn control_type(&self) -> &'static str;
  fn control_id(&self) -> &Vec<i32>;
  fn clone_trol(&self) -> Box<dyn Control>;
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>>;
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>>;
  fn add_control(&mut self, Box<dyn Control>);
  fn update(&mut self, _: &cu::UpdateMsg);
  fn empty_update(&self) -> Option<cu::UpdateMsg>;
  // build full update message of current state.
  fn to_update(&self) -> Option<cu::UpdateMsg>;
  fn name(&self) -> &str;
  fn as_json(&self) -> Value;
}

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
  Vertical,
  Horizontal,
}

#[derive(Debug)]
pub struct Slider {
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: Option<String>,
  pub pressed: bool,
  pub location: f32,
  pub orientation: Orientation,
}

impl Control for Slider {
  fn as_json(&self) -> Value {
    let mut btv = BTreeMap::new();
    btv.insert(String::from("type"), Value::String("slider".to_string()));
    btv.insert(
      String::from("orientation"),
      Value::String("vertical".to_string()),
    );
    btv.insert(String::from("name"), Value::String(self.name.to_string()));
    if let Some(lb) = &self.label {
      btv.insert(String::from("label"), Value::String(lb.to_string()));
    };
    match self.orientation {
      Orientation::Vertical => btv.insert(
        String::from("orientation"),
        Value::String("vertical".to_string()),
      ),
      Orientation::Horizontal => btv.insert(
        String::from("orientation"),
        Value::String("horizontal".to_string()),
      ),
    };
    Value::Object(btv)
  }
  fn control_type(&self) -> &'static str {
    "slider"
  }
  fn control_id(&self) -> &Vec<i32> {
    &self.control_id
  }
  fn clone_trol(&self) -> Box<dyn Control> {
    Box::new(Slider {
      control_id: self.control_id.clone(),
      name: self.name.clone(),
      label: self.label.clone(),
      pressed: self.pressed.clone(),
      location: self.location.clone(),
      orientation: self.orientation,
    })
  }
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>> {
    None
  }
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>> {
    None
  }
  fn add_control(&mut self, _control: Box<dyn Control>) {}

  fn update(&mut self, um: &cu::UpdateMsg) {
    match um {
      &cu::UpdateMsg::Slider {
        control_id: _,
        state: ref opt_state,
        location: ref opt_loc,
        label: ref opt_label,
      } => {
        if let &Some(ref st) = opt_state {
          self.pressed = match st {
            &cu::PressState::Pressed => true,
            &cu::PressState::Unpressed => false,
          };
        };
        if let &Some(ref loc) = opt_loc {
          self.location = *loc as f32;
        };
        if let &Some(ref t) = opt_label {
          self.label = Some(t.clone());
        };
        ()
      }
      _ => (),
    }
  }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Slider {
      control_id: self.control_id.clone(),
      state: None,
      location: None,
      label: None,
    })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    let state = if self.pressed {
      cu::PressState::Pressed
    } else {
      cu::PressState::Unpressed
    };
    Some(cu::UpdateMsg::Slider {
      control_id: self.control_id.clone(),
      state: Some(state),
      location: Some(self.location as f64),
      label: self.label.clone(),
    })
  }
  fn name(&self) -> &str {
    &self.name[..]
  }
}

#[derive(Debug)]
pub struct XY {
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: Option<String>,
  pub pressed: bool,
  pub location: (f32,f32),
}

impl Control for XY {
  fn as_json(&self) -> Value {
    let mut btv = BTreeMap::new();
    btv.insert(String::from("type"), Value::String("xy".to_string()));
    btv.insert(String::from("name"), Value::String(self.name.to_string()));
    if let Some(lb) = &self.label {
      btv.insert(String::from("label"), Value::String(lb.to_string()));
    };
    Value::Object(btv)
  }
  fn control_type(&self) -> &'static str {
    "slider"
  }
  fn control_id(&self) -> &Vec<i32> {
    &self.control_id
  }
  fn clone_trol(&self) -> Box<dyn Control> {
    Box::new(XY {
      control_id: self.control_id.clone(),
      name: self.name.clone(),
      label: self.label.clone(),
      pressed: self.pressed.clone(),
      location: self.location.clone(),
    })
  }
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>> {
    None
  }
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>> {
    None
  }
  fn add_control(&mut self, _control: Box<dyn Control>) {}

  fn update(&mut self, um: &cu::UpdateMsg) {
    match um {
      &cu::UpdateMsg::XY {
        control_id: _,
        state: ref opt_state,
        location: ref opt_loc,
        label: ref opt_label,
      } => {
        if let &Some(ref st) = opt_state {
          self.pressed = match st {
            &cu::PressState::Pressed => true,
            &cu::PressState::Unpressed => false,
          };
        };
        if let &Some(ref loc) = opt_loc {
          self.location = *loc;
        };
        if let &Some(ref t) = opt_label {
          self.label = Some(t.clone());
        };
        ()
      }
      _ => (),
    }
  }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::XY {
      control_id: self.control_id.clone(),
      state: None,
      location: None,
      label: None,
    })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    let state = if self.pressed {
      cu::PressState::Pressed
    } else {
      cu::PressState::Unpressed
    };
    Some(cu::UpdateMsg::XY {
      control_id: self.control_id.clone(),
      state: Some(state),
      location: Some(self.location ),
      label: self.label.clone(),
    })
  }
  fn name(&self) -> &str {
    &self.name[..]
  }
}

#[derive(Debug)]
pub struct Button {
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: Option<String>,
  pub pressed: bool,
}

impl Control for Button {
  fn as_json(&self) -> Value {
    let mut btv = BTreeMap::new();
    btv.insert(
      String::from("type"),
      Value::String(self.control_type().to_string()),
    );
    btv.insert(String::from("name"), Value::String(self.name.to_string()));
    if let Some(lb) = &self.label {
      btv.insert(String::from("label"), Value::String(lb.to_string()));
    };
    Value::Object(btv)
  }
  fn control_type(&self) -> &'static str {
    "button"
  }
  fn control_id(&self) -> &Vec<i32> {
    &self.control_id
  }
  fn clone_trol(&self) -> Box<dyn Control> {
    Box::new(Button {
      control_id: self.control_id.clone(),
      name: self.name.clone(),
      label: self.label.clone(),
      pressed: self.pressed.clone(),
    })
  }
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>> {
    None
  }
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>> {
    None
  }
  fn add_control(&mut self, _control: Box<dyn Control>) {}
  fn update(&mut self, um: &cu::UpdateMsg) {
    match um {
      &cu::UpdateMsg::Button {
        control_id: _,
        state: ref opt_state,
        label: ref opt_label,
      } => {
        if let &Some(ref st) = opt_state {
          self.pressed = match st {
            &cu::PressState::Pressed => true,
            &cu::PressState::Unpressed => false,
          };
        };
        if let &Some(ref t) = opt_label {
          self.label = Some(t.clone());
        };
        ()
      }
      _ => (),
    }
  }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Button {
      control_id: self.control_id.clone(),
      state: None,
      label: None,
    })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    let ut = if self.pressed {
      cu::PressState::Pressed
    } else {
      cu::PressState::Unpressed
    };
    Some(cu::UpdateMsg::Button {
      control_id: self.control_id.clone(),
      state: Some(ut),
      label: self.label.clone(),
    })
  }
  fn name(&self) -> &str {
    &self.name[..]
  }
}

#[derive(Debug)]
pub struct Label {
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: String,
}

impl Control for Label {
  fn as_json(&self) -> Value {
    let mut btv = BTreeMap::new();
    btv.insert(
      String::from("type"),
      Value::String(self.control_type().to_string()),
    );
    btv.insert(String::from("name"), Value::String(self.name.to_string()));
    btv.insert(String::from("label"), Value::String(self.label.to_string()));
    Value::Object(btv)
  }
  fn control_type(&self) -> &'static str {
    "label"
  }
  fn control_id(&self) -> &Vec<i32> {
    &self.control_id
  }
  fn clone_trol(&self) -> Box<dyn Control> {
    Box::new(Label {
      control_id: self.control_id.clone(),
      name: self.name.clone(),
      label: self.label.clone(),
    })
  }
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>> {
    None
  }
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>> {
    None
  }
  fn add_control(&mut self, _control: Box<dyn Control>) {}
  fn update(&mut self, um: &cu::UpdateMsg) {
    match um {
      &cu::UpdateMsg::Label {
        control_id: _,
        label: ref l,
      } => {
        self.label = l.clone();
        ()
      }
      _ => (),
    }
  }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Label {
      control_id: self.control_id.clone(),
      label: String::from(""),
    })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Label {
      control_id: self.control_id.clone(),
      label: self.label.clone(),
    })
  }
  fn name(&self) -> &str {
    &self.name[..]
  }
}

//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Sizer {
  pub control_id: Vec<i32>,
  pub controls: Vec<Box<dyn Control>>,
  pub orientation: Orientation,
}

impl Control for Sizer {
  fn as_json(&self) -> Value {
    let ctrlvals = self.controls.iter().map(|c| c.as_json()).collect();
    let mut btv = BTreeMap::new();
    btv.insert(
      String::from("type"),
      Value::String(self.control_type().to_string()),
    );
    match self.orientation {
      Orientation::Vertical => btv.insert(
        String::from("orientation"),
        Value::String("vertical".to_string()),
      ),
      Orientation::Horizontal => btv.insert(
        String::from("orientation"),
        Value::String("horizontal".to_string()),
      ),
    };
    btv.insert(String::from("controls"), Value::Array(ctrlvals));
    Value::Object(btv)
  }
  fn control_type(&self) -> &'static str {
    "sizer"
  }
  fn control_id(&self) -> &Vec<i32> {
    &self.control_id
  }
  fn clone_trol(&self) -> Box<dyn Control> {
    Box::new(Sizer {
      control_id: self.control_id.clone(),
      controls: Vec::new(),
      orientation: self.orientation,
    })
  }
  fn sub_controls(&self) -> Option<&Vec<Box<dyn Control>>> {
    Some(&self.controls)
  }
  fn mut_sub_controls(&mut self) -> Option<&mut Vec<Box<dyn Control>>> {
    Some(&mut self.controls)
  }
  fn add_control(&mut self, control: Box<dyn Control>) {
    self.controls.push(control);
  }
  fn update(&mut self, _: &cu::UpdateMsg) {}
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    None
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    None
  }
  fn name(&self) -> &str {
    ""
  }
}

// -------------------------------------------------------------------------

pub fn get_um_id(um: &cu::UpdateMsg) -> &Vec<i32> {
  match um {
    &cu::UpdateMsg::Button {
      control_id: ref cid,
      state: _,
      label: _,
    } => &cid,
    &cu::UpdateMsg::Slider {
      control_id: ref cid,
      state: _,
      label: _,
      location: _,
    } => &cid,
     &cu::UpdateMsg::XY {
      control_id: ref cid,
      state: _,
      label: _,
      location: _,
    } => &cid,
    &cu::UpdateMsg::Label {
      control_id: ref cid,
      label: _,
    } => &cid,
  }
}

// --------------------------------------------------------
// control state map.  copies all the controls.
// --------------------------------------------------------

pub type ControlMap = BTreeMap<Vec<i32>, Box<dyn Control>>;

pub fn make_control_map(control: &dyn Control) -> ControlMap {
  let btm = BTreeMap::new();

  make_control_map_impl(control, btm)
}

fn make_control_map_impl(control: &dyn Control, mut map: ControlMap) -> ControlMap {
  map.insert(control.control_id().clone(), control.clone_trol());

  match control.sub_controls() {
    Some(trols) => {
      let mut item = trols.into_iter();

      loop {
        match item.next() {
          Some(x) => map = make_control_map_impl(&**x, map),
          None => break,
        }
      }
    }
    None => {}
  }

  map
}

pub type ControlNameMap = BTreeMap<String, Vec<i32>>;

pub fn control_map_to_name_map(cmap: &ControlMap) -> ControlNameMap {
  let mut iter = cmap.iter();
  let mut cnm = BTreeMap::new();

  loop {
    match iter.next() {
      Some((key, val)) => {
        let s = String::from(&*val.name());
        cnm.insert(s, key.clone());
        ()
      }
      None => break,
    }
  }

  cnm
}

pub fn cm_to_update_array(cm: &ControlMap) -> Vec<cu::UpdateMsg> {
  let mut iter = cm.iter();
  let mut result = Vec::new();

  loop {
    match iter.next() {
      Some((_, val)) => match val.to_update() {
        Some(updmsg) => result.push(updmsg),
        None => (),
      },
      None => return result,
    }
  }
}
