use dbconnection::*;
use cdrs::query::QueryExecutor;
pub fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS employee WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    session.query(create_ks).expect("Keyspace creation error");
}

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE employee.emp_details (
    emp_id text,
    emp_name text,
    emp_salary float,
    emp_mobile text,
    PRIMARY KEY (emp_id))";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

