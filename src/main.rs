#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
#[macro_use]
extern crate crudactix;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate openssl;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, HttpRequest, HttpResponse, Responder, server};
use actix_web::http::Method;
use cdrs::types::prelude::*;
/*
use crudactix::dbconnection::CurrentSession;
use crudactix::models::Employee;
pub mod models;
pub mod dbconnection;
*/

use std::collections::HashMap;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;


pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]

pub struct Employee {
    pub emp_id: String,
    pub emp_name: String,
    pub emp_salary: f32,
    pub emp_mobile: String,
}



fn connection() -> CurrentSession{
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let no_compression: CurrentSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    return no_compression;
}

fn insert(_req: &HttpRequest) -> impl Responder
{
    let session = connection();
    let row: Employee = Employee {
        emp_id: "John_vxcvxcvcena".to_string(),
        emp_name: "John vxcDoe".to_string(),
        emp_salary: 100000.00,
        emp_mobile: "123456789".to_string(),
    };

    let insert_struct_cql = "INSERT INTO employee.emp_details \
                           (emp_id, emp_name, emp_salary, emp_mobile) VALUES (?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.emp_id, row.emp_name, row.emp_salary, row.emp_mobile))
        .expect("insert error ");

    "Hello world"
}

fn view(_req: &HttpRequest) -> impl Responder
{
   // println!("sdfgdsvghdscvv");
    let session = connection();

    let select_struct_cql = "Select * from employee.emp_details";
    let rows = session
        .query(select_struct_cql)
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

   /* let rows: Vec<Row> = crudactix::select_table::select_struct(&con);*/

  //  println!("tgregregreg");
    for row in rows {
        let my_row: Employee = Employee::try_from_row(row).expect("into Employee Struct");
        println!("Struct got :{:?}", my_row);
    }

    "display on console"
}

fn update(_req: &HttpRequest) -> impl Responder {
    let session = connection();
    let update_struct_cql = "Update employee.emp_details SET emp_name = ? where emp_id = ?";
    let emp_name = "John knol ";
    let emp_id = "John_vxcvxcvcena";
    session
        .query_with_values(update_struct_cql, query_values!(emp_name, emp_id))
        .expect("update takes place");
    //let x=view(HttpRequest);

    "it is updated"
}

fn delete(_req: &HttpRequest) -> impl Responder {
    let session = connection();
    let delete_struct_cql = "Delete from employee.emp_details where emp_id = ?";
    let user_key = "John_vxcvxcvcena";
    session
        .query_with_values(delete_struct_cql, query_values!(user_key))
        .expect("delete");

    "It is deleted"
}

fn main() {
    let con =connection();

    /*crudactix::create_table::create_keyspace(&con);
        crudactix::create_table::create_table(&con);
        */

    server::new(|| {
        vec![
            App::new()
                .prefix("/insert")
                .resource("/", |r| r.f(insert)),
            App::new()
                .prefix("/view")
                .resource("/", |r| r.f(view)),
            App::new()
                .prefix("/update")
                .resource("/", |r| r.f(update)),
            App::new()
                .prefix("/delete")
                .resource("/", |r| r.f(delete)),
            App::new().resource("/", |r| r.f(|r| HttpResponse::Ok())),
        ]
    })
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}
