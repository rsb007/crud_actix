extern crate actix;
extern crate actix_web;
extern crate bytes;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate crudactix;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate listenfd;
extern crate maplit;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{
    App, http, HttpRequest,
    middleware, Responder, server,
};
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::rows::Row;
use listenfd::ListenFd;
use std::str;


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

fn view(emp_id: String) -> Vec<Row>
{
    let session = connection();

    let select_struct_cql = "Select * from employee.emp_details where emp_id = ?";
    let row = session
        .query_with_values(select_struct_cql, query_values!(emp_id))
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return row;
}


fn select_manual(req: &HttpRequest) -> impl Responder
{
    let rows = view(req.query().get("emp_id").unwrap().parse().unwrap());
    //println!("{}",rows.len());
    let mut my_row: Employee = Employee {
        emp_name: String::new(),
        emp_salary: 0.0,
        emp_id: String::new(),
        emp_mobile: String::new(),
    };

    /* let row=&rows[0];
     let my_row= Employee::try_from_row(row).expect("into Employee Struct");*/
    for row in rows {
        my_row = Employee::try_from_row(row).expect("into Employee Struct");
        //  println!("Struct got :{:?}", my_row);
    }

    let jsonstring = serde_json::to_string(&my_row);

    return jsonstring;
}


fn main() {
    ::std::env::set_var("RUST_Log", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            //enable logger
            .middleware(middleware::Logger::default())
            .resource("/insert", |r| r.method(http::Method::POST).f(crudactix::insert_table::insert_manual))
            .resource("/update", |r| r.method(http::Method::PUT).f(crudactix::update_table::update_manual))
            .resource("/delete", |r| r.method(http::Method::DELETE).f(crudactix::delete_table::delete_manual))
            .resource("/view", |r| r.method(http::Method::GET).f(select_manual))
        //.resource("/",|r|  r.f(|r| HttpResponse::Ok()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}
