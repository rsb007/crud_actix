use models::Employee;
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


fn update(row: Employee) {
    let session = dbconnection::connection();
    let update_struct_cql = "Update employee.emp_details SET emp_name = ? , emp_salary = ? ,emp_mobile = ? where emp_id = ?";

    session
        .query_with_values(update_struct_cql, query_values!(row.emp_name, row.emp_salary, row.emp_mobile, row.emp_id))
        .expect("update takes place");
    //let x=view(HttpRequest);
}


pub fn update_manual(
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

            let emp: Employee = serde_json::from_str(&injson.to_string())?;
            update(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))
        })
        .responder()
}