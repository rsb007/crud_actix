
use bytes::BytesMut;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::prelude::*;
use futures::{Future, Stream};
use json::JsonValue;
use std::collections::HashMap;
use cdrs::types::rows::Row;
use std::str;
use futures::collect;


#[derive(Clone, Debug, IntoCDRSValue, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub emp_id: String,
    pub emp_name: String,
    pub emp_salary: f32,
    pub emp_mobile: String,
}

#[derive(Clone, Debug, IntoCDRSValue, PartialEq, Serialize, Deserialize)]
pub struct employee {
    pub emp_id : String,
}