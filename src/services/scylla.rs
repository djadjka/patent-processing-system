use cdrs::authenticators::NoneAuthenticator;
use cdrs::query::QueryExecutor;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobinSync;
use std::sync::{Arc};
// use cdrs::query::*;

pub type CurrentSession = Arc<Session<RoundRobinSync<TcpConnectionPool<NoneAuthenticator>>>>;

pub fn create_session() -> CurrentSession {
    let node1 = NodeTcpConfigBuilder::new("127.0.0.1:9001", NoneAuthenticator {}).build();
    let node2 = NodeTcpConfigBuilder::new("127.0.0.1:9002", NoneAuthenticator {}).build();
    let node3 = NodeTcpConfigBuilder::new("127.0.0.1:9003", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node1,node2,node3]);
    let no_compression = new_session(&cluster_config, RoundRobinSync::new()).expect("session should be created");
    let session = Arc::new(no_compression);
    migration(session.clone());
    session
}

pub fn migration(session: CurrentSession) {
    let query = "CREATE KEYSPACE IF NOT EXISTS test WITH REPLICATION = { \
                                   'class' : 'SimpleStrategy', 'replication_factor' : 2 };";
    session.query(query).expect("Keyspace creation error");

    let create_table_cql =
    "CREATE TABLE IF NOT EXISTS test.patents ( \
        serial_number text, \
        registration_date bigint, \
        expire_date bigint, \
        company text, \
        img text, \
        info text, \
        PRIMARY KEY (serial_number) \
    )";
    session.query(create_table_cql).expect("Table creation error");
}