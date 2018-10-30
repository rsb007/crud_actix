use dbconnection;

use cdrs::query::QueryExecutor;

use actix_web::{
    AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};

use futures::{Future, Stream};
use json::JsonValue;
use serde_json;
use std;
use json;
use models::Emp;


fn delete(emp: Emp) {
    let session = dbconnection::connection();
    let delete_struct_cql = "Delete from employee.emp_details where emp_id = ?";
    session
        .query_with_values(delete_struct_cql, query_values!(emp.emp_id))
        .expect("delete");
}

pub fn delete_manual(
    req: &HttpRequest,
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object! {"err" => e.to_string() },
            };

            let emp: Emp = serde_json::from_str(&injson.to_string())?;
            delete(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))
        })
        .responder()
}