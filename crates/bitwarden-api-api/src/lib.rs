#![allow(warnings)]
#![allow(clippy::all)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_repr;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;
