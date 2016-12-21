//use serde;
// use serde::de::Deserialize;
// #![feature(custom_derive, plugin)]
// #![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;

use serde_json::Value;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::error::Error;
use stringerror;
use control_updates as cu;

// --------------------------------------------------------
// root obj.  contains controls.
// --------------------------------------------------------

// root is not itself a Control!  although maybe it should be?
#[derive(Debug)]
pub struct Root {
  pub title: String,
  pub root_control: Box<Control>,
}

// --------------------------------------------------------
// controls.
// --------------------------------------------------------

pub trait Control : Debug + Send {
  fn control_type(&self) -> &'static str; 
  fn control_id(&self) -> &Vec<i32>;
  fn clone_trol(&self) -> Box<Control>;
  fn sub_controls(&self) -> Option<&Vec<Box<Control>>>; 
  fn update(&mut self, _: &cu::UpdateMsg); 
  fn empty_update(&self) -> Option<cu::UpdateMsg>;
  // build full update message of current state.
  fn to_update(&self) -> Option<cu::UpdateMsg>;
  fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct Slider {
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: Option<String>,
  pub pressed: bool,
  pub location: f32,
}

impl Control for Slider {
  fn control_type(&self) -> &'static str { "slider" } 
  fn control_id(&self) -> &Vec<i32> { &self.control_id }
  fn clone_trol(&self) -> Box<Control> { 
    Box::new( 
      Slider { control_id: self.control_id.clone(), 
               name: self.name.clone(), 
               label: self.label.clone(), 
               pressed: self.pressed.clone(), 
               location: self.location.clone() } ) }
  fn sub_controls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &cu::UpdateMsg) {
    match um { 
      &cu::UpdateMsg::Slider { control_id: _
                         , state: ref opt_state
                         , location: ref opt_loc
                         , label: ref opt_label
                         } => {
        if let &Some(ref st) = opt_state {
          self.pressed = match st { &cu::SliderState::Pressed => true
                                  , &cu::SliderState::Unpressed => false };
          };
        if let &Some(ref loc) = opt_loc { 
          self.location = *loc as f32;
        };
        if let &Some(ref t) = opt_label {
          self.label = Some(t.clone());
        };
        ()
        }
      _ => ()
      }
    }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Slider  { control_id: self.control_id.clone()
                            , state: None
                            , location: None
                            , label: None 
                            })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    let state = if self.pressed { cu::SliderState::Pressed  } 
                else { cu::SliderState::Unpressed };
    Some(cu::UpdateMsg::Slider  { control_id: self.control_id.clone()
                            , state: Some(state)
                            , location: Some(self.location as f64) 
                            , label: self.label.clone()
                            })
  }
  fn name(&self) -> &str { &self.name[..] }
}

#[derive(Debug)]
pub struct Button { 
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: Option<String>,
  pub pressed: bool,
}

impl Control for Button { 
  fn control_type(&self) -> &'static str { "button" } 
  fn control_id(&self) -> &Vec<i32> { &self.control_id }
  fn clone_trol(&self) -> Box<Control> { 
    Box::new( 
      Button { control_id: self.control_id.clone(), 
               name: self.name.clone(), 
               label: self.label.clone(), 
               pressed: self.pressed.clone() } ) }
  fn sub_controls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &cu::UpdateMsg) {
    match um { 
      &cu::UpdateMsg::Button { control_id: _ 
                         , state: ref opt_state
                         , label: ref opt_label } => {
        if let &Some(ref st) = opt_state { 
          self.pressed = match st { &cu::ButtonState::Pressed => true
                                  , &cu::ButtonState::Unpressed => false };
          };
        if let &Some(ref t) = opt_label {
          self.label = Some(t.clone());
        };
        ()
        }
      _ => ()
      }
    }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Button { control_id: self.control_id.clone()
                           , state: None 
                           , label: None })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    let ut = if self.pressed { cu::ButtonState::Pressed  } 
                        else { cu::ButtonState::Unpressed };
    Some(cu::UpdateMsg::Button { control_id: self.control_id.clone()
                           , state: Some(ut)
                           , label: self.label.clone() })
  }
  fn name(&self) -> &str { &self.name[..] }
}

#[derive(Debug)]
pub struct Label { 
  pub control_id: Vec<i32>,
  pub name: String,
  pub label: String,
}

impl Control for Label { 
  fn control_type(&self) -> &'static str { "label" } 
  fn control_id(&self) -> &Vec<i32> { &self.control_id }
  fn clone_trol(&self) -> Box<Control> { 
    Box::new( 
      Label { control_id: self.control_id.clone(), 
              name: self.name.clone(), 
              label: self.label.clone() } ) }
  fn sub_controls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &cu::UpdateMsg) {
    match um { 
      &cu::UpdateMsg::Label { control_id: _, label: ref l } => {
        self.label = l.clone();
        ()
        }
      _ => ()
      }
    }
  fn empty_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Label { control_id: self.control_id.clone(), label: String::from("") })
  }
  fn to_update(&self) -> Option<cu::UpdateMsg> {
    Some(cu::UpdateMsg::Label { control_id: self.control_id.clone(), label: self.label.clone() })
  }
  fn name(&self) -> &str { &self.name[..] }
}

//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Sizer { 
  pub control_id: Vec<i32>,
  pub controls: Vec<Box<Control>>,
}

impl Control for Sizer { 
  fn control_type(&self) -> &'static str { "sizer" } 
  fn control_id(&self) -> &Vec<i32> { &self.control_id }
  fn clone_trol(&self) -> Box<Control> { 
    Box::new( 
      Sizer { control_id: self.control_id.clone(), 
              controls: Vec::new() } ) } 
  fn sub_controls(&self) -> Option<&Vec<Box<Control>>> { Some(&self.controls) } 
  fn update(&mut self, _: &cu::UpdateMsg) {}
  fn empty_update(&self) -> Option<cu::UpdateMsg> { None }
  fn to_update(&self) -> Option<cu::UpdateMsg> { None }
  fn name(&self) -> &str { "" }
}



// -------------------------------------------------------------------------

pub fn get_um_id(um: &cu::UpdateMsg) -> &Vec<i32> {
  match um { 
    &cu::UpdateMsg::Button { control_id: ref cid, state: _, label: _ } => &cid,
    &cu::UpdateMsg::Slider { control_id: ref cid, state: _, label: _, location: _ } => &cid, 
    &cu::UpdateMsg::Label { control_id: ref cid, label: _ } => &cid, 
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
            (match st { &cu::ButtonState::Pressed => "Press", 
                        &cu::ButtonState::Unpressed => "Unpress", }))));
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
            (match st { &cu::SliderState::Pressed => "Press",
                        &cu::SliderState::Unpressed => "Unpress" }))));
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
    &cu::UpdateMsg::Label { control_id: ref cid, label: ref labtext } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("label")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("label"), Value::String(labtext.clone()));
      Value::Object(btv)
    }, 
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

// --------------------------------------------------------
// control state map.  copies all the controls.
// --------------------------------------------------------

pub type ControlMap = BTreeMap<Vec<i32>,Box<Control>>;

pub fn make_control_map (control: &Control) -> ControlMap {
  let btm = BTreeMap::new();

  make_control_map_impl(control, btm)
}

fn make_control_map_impl (control: &Control, mut map: ControlMap) 
  -> ControlMap 
{ 
  map.insert(control.control_id().clone(), control.clone_trol());

  match control.sub_controls() {
    Some(trols) => {
      let mut item = trols.into_iter();

      loop {
          match item.next() {
            Some(x) => {
              map = make_control_map_impl(&**x, map)
              },
            None => { break },
          }
        }
      }
    None => {} 
    }

  map
}

pub type ControlNameMap = BTreeMap<String, Vec<i32>>;

pub fn control_map_to_name_map(cmap: &ControlMap) -> ControlNameMap 
{
  let mut iter = cmap.iter();
  let mut cnm = BTreeMap::new();

  loop {
    match iter.next() {
      Some((key,val)) => { 
        let s = String::from(&*val.name()); 
        cnm.insert(s, key.clone());
        ()
      }, 
      None => break
    }
  }

  cnm
}

pub fn cm_to_update_array(cm: &ControlMap) -> Vec<cu::UpdateMsg>
{
  let mut iter = cm.iter();
  let mut result = Vec::new();
  
  loop {
    match iter.next() {
      Some((_,val)) => { 
        match val.to_update() { 
          Some(updmsg) => result.push(updmsg),
          None => (),
        }
      },
      None => return result,
    }
  }
}
