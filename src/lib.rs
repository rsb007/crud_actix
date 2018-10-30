#[macro_use]
extern crate actix;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate bytes;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate json;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate openssl;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


pub mod models;


pub mod dbconnection;

pub mod create_table;

pub mod insert_table;

//pub mod select_table;

pub mod update_table;

pub mod delete_table;
