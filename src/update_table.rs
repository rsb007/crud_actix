
fn update_struct(session: &CurrentSession) {
    let update_struct_cql = "Update employee.emp_details SET emp_name = ? where emp_id = ?";
    let emp_name = "John Cena";
    let emp_id = "John";
    session
        .query_with_values(update_struct_cql, query_values!(emp_name, emp_id))
        .expect("update takes place");
}
