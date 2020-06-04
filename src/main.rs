extern crate thrift;

mod replicaservice;

use std::sync::{Arc, Mutex};
use std::rc::{Rc};
use std::collections::{HashMap, VecDeque, BTreeMap};
use std::thread;

use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory};
use thrift::server::TServer;

use replicaservice::{Entry, EntryType, ID, ReplicaServiceSyncProcessor, ReplicaServiceSyncHandler, PutResponse, GetResponse, AppendEntryResponse, Ballot};

enum ReplicaState {
    FOLLOWER,
    CANDIDATE,
    LEADER
}

struct Replica {
    state : Arc<Mutex<ReplicaState>>,
    current_term : Arc<Mutex<u32>>,
    log : Arc<Mutex<Vec<Entry>>>,
    commit_index : Arc<Mutex<usize>>,
    last_applied : Arc<Mutex<usize>>,
    next_index : Arc<Mutex<Vec<usize>>>,
    match_index : Arc<Mutex<Vec<usize>>>,
    time_left : Arc<Mutex<u32>>,
    timeout : Arc<u32>,
    voted_for : Arc<Mutex<ID>>,
    leader : Arc<Mutex<ID>>,
    state_machine : Arc<Mutex<HashMap<String, String>>>,
    current_request_being_serviced : Arc<Mutex<u32>>,
    noop_index : Arc<Mutex<usize>>,
    jobs_to_retry : Arc<Mutex<VecDeque<mystorers::Job>>>,
    cluster_membership : Arc<Mutex<Vec<ID>>>,
    has_operation_started : bool,
    my_id : ID,
    timer_thr : thread::JoinHandle<()>
}

impl Replica {
    fn timer(self) {
    }

    fn new(port : u32) -> Self {

        let mut replica = Replica {
            state : Arc::new(Mutex::new(ReplicaState::FOLLOWER)),
            current_term : Arc::new(Mutex::new(0)),
            commit_index : Arc::new(Mutex::new(0)),
            last_applied : Arc::new(Mutex::new(0)),
            timeout : Arc::new(0),
            voted_for : Arc::new(Mutex::new(ID::new(None, None))),
            leader : Arc::new(Mutex::new(ID::new(None, None))),
            current_request_being_serviced : Arc::new(Mutex::new(u32::max_value())),
            has_operation_started : false,
            cluster_membership : Arc::new(Mutex::new(Vec::new())),
            noop_index : Arc::new(Mutex::new(0)),
            jobs_to_retry : Arc::new(Mutex::new(VecDeque::new())),
            match_index : Arc::new(Mutex::new(Vec::new())),
            next_index : Arc::new(Mutex::new(Vec::new())),
            log : Arc::new(Mutex::new(Vec::new())),
            state_machine : Arc::new(Mutex::new(HashMap::new())),
            time_left : Arc::new(Mutex::new(0)),
            my_id : Default::default(),
            timer_thr : thread::spawn(|| {})
        };

        //replica.timer_thr = thread::spawn(|| replica.timer());

        replica
    }
}

impl ReplicaServiceSyncHandler for Replica {
    fn handle_request_vote(&self, term: i32, candidate_i_d: ID, last_log_index: i32, last_log_term: i32) -> thrift::Result<Ballot> {
        Ok(Ballot::default())
    }

    fn handle_append_entry(&self, term: i32, leader_i_d: ID, prev_log_index: i32, prev_log_term: i32, entry: Entry, leader_commit: i32) -> 
                                                                                                     thrift::Result<AppendEntryResponse> {
        Ok(AppendEntryResponse::default())
    }

    fn handle_get(&self, key: String, client_identifier: String, request_identifier: i32) -> thrift::Result<GetResponse> {
        Ok(GetResponse::default())
    }

    fn handle_put(&self, key: String, value: String, client_identifier: String, request_identifier: i32) -> thrift::Result<PutResponse> {
        Ok(PutResponse::default())
    }

    fn handle_kill(&self) -> thrift::Result<()> {
        Ok(())
    }

    fn handle_get_information(&self) -> thrift::Result<BTreeMap<String, String>> {
        Ok(BTreeMap::new())
    }

    fn handle_start(&self) -> thrift::Result<()> {
        Ok(())
    }

    fn handle_install_snapshot(&self, leader_term: i32, leader_i_d: ID, last_included_index: i32, last_included_term: i32, offset: i32, data: Vec<u8>, done: bool) -> thrift::Result<i32> {
        Ok(0)
    }

    fn handle_add_new_configuration(&self, endpoints: Vec<ID>) -> thrift::Result<bool> {
        Ok(true)
    }
}   

fn run() -> thrift::Result<()> {

    let listen_address = format!("127.0.0.1:{}", 5000);

    println!("binding to {}", listen_address);

    let i_tran_fact = TFramedReadTransportFactory::new();
    let i_prot_fact = TCompactInputProtocolFactory::new();

    let o_tran_fact = TFramedWriteTransportFactory::new();
    let o_prot_fact = TCompactOutputProtocolFactory::new();

    let replica = Arc::new(Replica::new(5000));

    let underlying_object = match Arc::try_unwrap(replica) {
        Ok(obj) => obj,
        Err(err) => panic!("Unable to grab underlying replica from rc pointer")
    };

    let timer_thr = thread::spawn(|| underlying_object.timer());

    let processor = ReplicaServiceSyncProcessor::new(underlying_object);

    let mut server = TServer::new(
        i_tran_fact,
        i_prot_fact,
        o_tran_fact,
        o_prot_fact,
        processor,
        10,
    );

    server.listen(&listen_address)
}

fn main() {
    match run() {
        Ok(()) => println!("server ran successfully"),
        Err(e) => {
            println!("server failed with error {:?}", e);
            std::process::exit(1);
        }
    }
}
