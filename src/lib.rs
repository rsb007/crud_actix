extern crate actix;
extern crate actix_web;
extern crate bytes;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate json;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


pub mod models;


pub mod dbconnection;

pub mod create_table;

pub mod insert_table;

//pub mod select_table;

pub mod update_table;

pub mod delete_table;
