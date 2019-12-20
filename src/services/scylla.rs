use cdrs::authenticators::NoneAuthenticator;
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
    Arc::new(no_compression)
}