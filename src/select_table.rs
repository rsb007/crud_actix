use models::Employee;
use dbconnection::*;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::Row;

pub fn select_struct(session: &CurrentSession) -> Vec<Row>
{
    let select_struct_cql = "Select * from employee.emp_details";
    let rows = session
        .query(select_struct_cql)
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return rows;
}

