// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
// 
// use generated_module::TopLevel;
// 
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: TopLevel = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "rootControl")]
    root_control: RootControl,
}

#[derive(Serialize, Deserialize)]
pub struct RootControl {
    #[serde(rename = "type")]
    root_control_type: String,

    #[serde(rename = "orientation")]
    orientation: String,

    #[serde(rename = "controls")]
    controls: Vec<RootControlControl>,
}

#[derive(Serialize, Deserialize)]
pub struct RootControlControl {
    #[serde(rename = "type")]
    control_type: String,

    #[serde(rename = "orientation")]
    orientation: String,

    #[serde(rename = "controls")]
    controls: Vec<PurpleControl>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleControl {
    #[serde(rename = "type")]
    control_type: Type,

    #[serde(rename = "name")]
    name: Option<String>,

    #[serde(rename = "orientation")]
    orientation: Option<Orientation>,

    #[serde(rename = "controls")]
    controls: Option<Vec<FluffyControl>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyControl {
    #[serde(rename = "type")]
    control_type: String,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "button")]
    Button,

    #[serde(rename = "sizer")]
    Sizer,

    #[serde(rename = "slider")]
    Slider,
}

#[derive(Serialize, Deserialize)]
pub enum Orientation {
    #[serde(rename = "horizontal")]
    Horizontal,

    #[serde(rename = "vertical")]
    Vertical,
}
