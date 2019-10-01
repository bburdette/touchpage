extern crate websocket;

#[macro_use]
extern crate log;
extern crate serde_json;
extern crate failure;
extern crate actix_web;
extern crate toml;

#[macro_use]
pub mod controls;
mod broadcaster;
pub mod json;
pub mod control_nexus;
pub mod control_updates;
pub mod guibuilder;
pub mod string_defaults;
pub mod webserver;
pub mod websocketserver;



