use models::Employee;
use dbconnection::*;

use cdrs::query::QueryExecutor;

pub fn insert_struct(session:&CurrentSession, row: Employee) {

    let insert_struct_cql = "INSERT INTO employee.emp_details \
                           (emp_id, emp_name, emp_salary, emp_mobile) VALUES (?, ?, ?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.emp_id, row.emp_name, row.emp_salary, row.emp_mobile))
        .expect("insert error ");
}