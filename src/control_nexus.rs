use control_updates as cu;
use controls;
use util::{load_string, write_string};
use serde_json::Value;
use broadcaster;
use failure::Error as FError;
use json;
use websocket::message::Message;
use std::sync::{Arc, Mutex};

// Implement a ControlUpdateProcessor if you want the controls to actually do something
// on the server.
pub trait ControlUpdateProcessor: Send {
  fn on_update_received(&mut self, &cu::UpdateMsg, ci: &ControlInfo) -> ();
}


// A basic ControlUpdateProcessor that just prints out the update messages.
pub struct PrintUpdateMsg {
}

impl ControlUpdateProcessor for PrintUpdateMsg { 
  fn on_update_received(&mut self, update: &cu::UpdateMsg, ci: &ControlInfo) -> ()
  {
    println!("update callback called! {:?}", update);
  }
}

// Info about all the controls.
pub struct ControlInfo {
  cm: controls::ControlMap,
  cnm: controls::ControlNameMap,
  guijson: String,
}

impl ControlInfo {
  pub fn get_name(&self, id: &Vec<i32>) -> Option<String> {
    match self.cm.get(id) {
      Some(ctrl) => Some(String::from(ctrl.name())),
      _ => None,
    }
  }
}

// The control nexus contains all the controls and the broadcaster.
pub struct ControlNexus {
  ci: Arc<Mutex<ControlInfo>>,
  bc: broadcaster::Broadcaster,
}

impl ControlNexus {
  fn get_cid_by_name(&self, name: &str) -> Option<Vec<i32>> {
    let guard = match self.ci.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };

    match guard.cnm.get(name) {
      Some(cid) => Some(cid.clone()),
      _ => None,
    }
  }

  pub fn get_name(&self, id: &Vec<i32>) -> Option<String> {
    let ci = match self.ci.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };

    match ci.cm.get(id) {
      Some(ctrl) => Some(String::from(ctrl.name())),
      _ => None,
    }
  }

  pub fn make_update_msg(&self, name: &str) -> Option<cu::UpdateMsg> {
    let guard = match self.ci.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };

    match guard.cnm.get(name) {
      Some(cid) => match guard.cm.get(cid) {
        Some(ctrl) => ctrl.empty_update(),
        None => None,
      },
      _ => None,
    }
  }
  pub fn update(&self, updmsg: &cu::UpdateMsg) {
    let mut ci = match self.ci.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };

    match ci.cm.get_mut(controls::get_um_id(&updmsg)) {
      Some(ctl) => {
        (*ctl).update(&updmsg);
        let val = json::encode_update_message(&updmsg);
        match serde_json::ser::to_string(&val) {
          Ok(s) => self.bc.broadcast(Message::text(s)),
          Err(_) => (),
        }
      }
      None => (),
    }
  }

  pub fn update_label(&self, name: &str, label: &str) {
    match self.get_cid_by_name(name) {
      Some(cid) => self.update(&cu::UpdateMsg::Label {
        control_id: cid,
        label: String::from(label),
      }),
      None => (),
    }
  }
  pub fn update_button(&self, name: &str, state: Option<cu::ButtonState>, label: Option<String>) {
    match self.get_cid_by_name(name) {
      Some(cid) => self.update(&cu::UpdateMsg::Button {
        control_id: cid,
        state: state,
        label: label,
      }),
      None => (),
    }
  }
  pub fn update_slider(
    &self,
    name: &str,
    state: Option<cu::SliderState>,
    location: Option<f64>,
    label: Option<String>,
  ) {
    match self.get_cid_by_name(name) {
      Some(cid) => self.update(&cu::UpdateMsg::Slider {
        control_id: cid,
        state: state,
        location: location,
        label: label,
      }),
      None => (),
    }
  }
  pub fn load_gui_string(&self, guistring: &str) -> Result<(), FError> {
    let guival = serde_json::from_str(guistring)?;

    let controltree = json::deserialize_root(&guival)?;
    println!("new control layout recieved!");

    println!(
      "title: {} count: {} ",
      controltree.title,
      controltree.root_control.control_type()
    );
    println!("controls: {:?}", controltree.root_control);

    let mut guard = match self.ci.lock() {
      Ok(guard) => guard,
      Err(poisoned) => poisoned.into_inner(),
    };

    (*guard).cm = controls::make_control_map(&*controltree.root_control);
    (*guard).cnm = controls::control_map_to_name_map(&(*guard).cm);
    (*guard).guijson = guistring.to_string();

    // send the updated gui string to all clients.
    self.bc.broadcast(Message::text(guistring.to_string()));

    Ok(())
  }
}
