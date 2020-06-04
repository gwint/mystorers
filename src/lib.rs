use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Entry {
}

pub struct ID {
    pub host : String,
    pub port : u32
}

pub struct Job {
    entry_position : u32,
    target_host : String,
    target_port : u32
}

struct LockHandler {
}

