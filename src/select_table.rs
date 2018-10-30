use actix_web::HttpRequest;
use actix_web::Responder;
use models::Employee;
use cdrs::types::prelude::Row;
use dbconnection;
use serde_json;

fn view(emp_id :String) -> Vec<Row>
{

    let session = dbconnection::connection();

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

pub fn select_manual(req:&HttpRequest) -> impl Responder
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
