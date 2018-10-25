
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::types::prelude::*;
use std::collections::HashMap;
use cdrs::frame::TryFromRow;
use dbconnection;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
pub struct Employee {
    pub emp_id: String,
    pub emp_name: String,
    pub emp_salary: f32,
    pub emp_mobile: String,
}

/*

pub fn view_of_employee(rows: &Vec<Row>){

   *//* let con = dbconnection::connection();
    let rows: Vec<Row> = crudactix::select_table::select_struct(&con);*//*

    for row in rows {
        let my_row: Employee = Employee::try_from_row(row).expect("into Employee Struct");
        format!("Struct got :{}", my_row.emp_name);
    }
}*/
