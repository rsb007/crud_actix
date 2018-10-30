use cdrs::frame::IntoBytes;
use cdrs::types::prelude::*;
use std::str;


#[derive(Clone, Debug, IntoCDRSValue, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub emp_id: String,
    pub emp_name: String,
    pub emp_salary: f32,
    pub emp_mobile: String,
}

#[derive(Clone, Debug, IntoCDRSValue, PartialEq, Serialize, Deserialize)]
pub struct Emp {
    pub emp_id: String,
}