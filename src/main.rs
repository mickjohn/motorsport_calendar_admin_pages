#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate rand;
extern crate reqwest;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tera;
extern crate toml;

// Logging
#[macro_use]
extern crate log;
extern crate log4rs;

//Common data structure
extern crate motorsport_calendar_common;

#[macro_use]
extern crate failure;

mod client;
mod config;
mod model;
mod session;
mod user;
mod web;

use std::path::Path;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("booting up");
    let config_path = Path::new("config.toml");
    let config = config::Config::from_toml_file(&config_path).unwrap();
    let wc = web::WebConfig::from(&config);
    web::start(wc);
}
