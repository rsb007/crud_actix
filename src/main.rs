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
#[macro_use]
extern crate listenfd;
#[macro_use]
extern crate crudactix;

use listenfd::ListenFd;


use actix_web::{
    App, AsyncResponder, error, Error, http, HttpMessage, HttpRequest, HttpResponse,
    Json, middleware, Responder, server,
};
use actix_web::http::Method;
use bytes::BytesMut;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::prelude::*;
use futures::{Future, Stream};
use json::JsonValue;
use std::collections::HashMap;
use cdrs::types::rows::Row;
use actix_web::Path;
use actix_web::Result;
use std::str;
use futures::collect;


pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub emp_id: String,
    pub emp_name: String,
    pub emp_salary: f32,
    pub emp_mobile: String,
}

fn connection() -> CurrentSession {
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    return no_compression;
}
/*
fn insert(row : Employee)
{
    let session = connection();

    let insert_struct_cql = "INSERT INTO employee.emp_details \
                           (emp_id, emp_name, emp_salary, emp_mobile) VALUES (?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.emp_id, row.emp_name, row.emp_salary, row.emp_mobile))
        .expect("insert error ");


}*/

fn view(emp_id :String) -> Vec<Row>
{

    let session = connection();

    let select_struct_cql = "Select * from employee.emp_details where emp_id = ?";
    let row  = session
        .query_with_values(select_struct_cql, query_values!(emp_id))
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return row;
}
/*

fn update(row : Employee) {
    let session = connection();
    let update_struct_cql = "Update employee.emp_details SET emp_name = ? , emp_salary = ? ,emp_mobile = ? where emp_id = ?";

    session
        .query_with_values(update_struct_cql, query_values!(row.emp_name, row.emp_salary, row.emp_mobile, row.emp_id))
        .expect("update takes place");
    //let x=view(HttpRequest);

}

fn delete(emp : employee)  {
    let session = connection();
    let delete_struct_cql = "Delete from employee.emp_details where emp_id = ?";
    session
        .query_with_values(delete_struct_cql, query_values!(emp.emp_id))
        .expect("delete");
}
*/

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/*


fn insert_manual(
    req: &HttpRequest,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object!{"err" => e.to_string() },
            };

            let emp :Employee = serde_json::from_str(&injson.to_string())?;
            insert(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))

        })
        .responder()
}


fn update_manual(
    req: &HttpRequest,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object!{"err" => e.to_string() },
            };

            let emp :Employee = serde_json::from_str(&injson.to_string())?;
            update(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))

        })
        .responder()

}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct employee {
    emp_id : String,
}



fn delete_manual(
    req: &HttpRequest,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object!{"err" => e.to_string() },
            };

            let emp : employee = serde_json::from_str(&injson.to_string())?;
            delete(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))

        })
        .responder()
}*/

fn select_manual(req:&HttpRequest) -> impl Responder
{
    let rows = view(req.query().get("emp_id").unwrap().parse().unwrap());
    //println!("{}",rows.len());
    let mut my_row: Employee =Employee{
        emp_name:String::new(),
        emp_salary: 0.0,
        emp_id: String::new(),
        emp_mobile: String::new()
    };

   /* let row=&rows[0];
    let my_row= Employee::try_from_row(row).expect("into Employee Struct");*/
    for row in rows {
        my_row= Employee::try_from_row(row).expect("into Employee Struct");
      //  println!("Struct got :{:?}", my_row);
    }

    let jsonstring = serde_json::to_string(&my_row);

   return jsonstring;
}


fn main() {

    ::std::env::set_var("RUST_Log", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
  let mut server= server::new(|| {
        App::new()
            //enable logger
            .middleware(middleware::Logger::default())
            .resource("/insert", |r| r.method(http::Method::POST).f( crudactix::insert_table::insert_manual))
            .resource("/update", |r| r.method(http::Method::PUT).f(crudactix::update_table::update_manual))
            .resource("/delete", |r| r.method(http::Method::DELETE).f(crudactix::delete_table::delete_manual))
            .resource("/view",   |r| r.method(http::Method::GET).f(select_manual))
            .resource("/",|r|  r.f(|r| HttpResponse::Ok()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}
