use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Job {
    entry_position : u32,
    target_host : String,
    target_port : u32
}

struct LockHandler {
}

