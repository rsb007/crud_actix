pub fn delete_struct(session: &CurrentSession)
{
    let delete_struct_cql = "Delete from employee.emp_details where emp_id = ?";
    let user_key = "John cena";
    session
        .query_with_values(delete_struct_cql, query_values!(user_key))
        .expect("delete");
}