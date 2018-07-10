#![feature(plugin, decl_macro)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tera;
extern crate toml;
#[macro_use]
extern crate lazy_static;

// Logging
#[macro_use]
extern crate log;
extern crate env_logger;
// extern crate log4rs;

//Common data structure
extern crate motorsport_calendar_common;

#[macro_use]
extern crate failure;

mod client;
mod config;
mod session;
mod user;
mod web;

use std::path::Path;

fn main() {
    let config_path = Path::new("config.toml");
    let config = config::Config::from_toml_file(&config_path).unwrap();
    let wc = web::WebConfig::from(&config);
    web::start(wc);
}
