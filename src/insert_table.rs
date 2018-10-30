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

fn insert(row: Employee)
{
    let session = dbconnection::connection();

    let insert_struct_cql = "INSERT INTO employee.emp_details \
                           (emp_id, emp_name, emp_salary, emp_mobile) VALUES (?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.emp_id, row.emp_name, row.emp_salary, row.emp_mobile))
        .expect("insert error ");
}

pub fn insert_manual(
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
            insert(emp);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))
        })
        .responder()
}