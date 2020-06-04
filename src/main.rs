extern crate thrift;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::collections::VecDeque;

use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory};
use thrift::server::TServer;

enum ReplicaState {
    FOLLOWER,
    CANDIDATE,
    LEADER
}

struct Replica {
    state : Arc<Mutex<ReplicaState>>,
    current_term : Arc<Mutex<u32>>,
    log : Arc<Mutex<Vec<mystorers::Entry>>>,
    commit_index : Arc<Mutex<usize>>,
    last_applied : Arc<Mutex<usize>>,
    next_index : Arc<Mutex<usize>>,
    match_index : Arc<Mutex<usize>>,
    time_left : Arc<Mutex<u32>>,
    timeout : Arc<u32>,
    voted_for : Arc<Mutex<mystorers::ID>>,
    leader : Arc<Mutex<mystorers::ID>>,
    state_machine : Arc<Mutex<HashMap<String, String>>>,
    current_request_being_serviced : Arc<Mutex<u32>>,
    noop_index : Arc<Mutex<usize>>,
    jobs_to_retry : Arc<Mutex<VecDeque<mystorers::Job>>>,
    cluster_membership : Arc<Mutex<Vec<mystorers::ID>>>,
    has_operation_started : bool,
    my_id : mystorers::ID
}

impl Replica {
    fn new(&mut self, port : u32) {
        self.state = Arc::new(Mutex::new(ReplicaState::FOLLOWER));
        self.current_term = Arc::new(Mutex::new(0));
        self.commit_index = Arc::new(Mutex::new(0));
        self.last_applied = Arc::new(Mutex::new(0));
        self.my_id.host = String::new();
        self.my_id.port = 0;
    }
}

fn main() {
    println!("Hello, world!");
}
