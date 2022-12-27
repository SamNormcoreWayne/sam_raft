use std::time::Duration;
use std::collections::LinkedList;
use crate::server::rpc::RaftRpc;

pub enum ServerState {
    Leader,
    Candidator,
    Follower,
}

pub struct RaftServer {
    pub term_id: u64,

    pub vote_id: u64,

    pub server_id: u64,

    pub server_state: ServerState,

    pub election_timeout_duration: Duration,

    pub heartbeat_timeout_duration: Duration,

    pub quorum: LinkedList<RaftServer>,

    snapshot: u64,
}

impl Default for RaftServer {
    fn default() -> RaftServer {
        RaftServer {
            term_id: 0u64,
            vote_id: 0u64,
            server_id: 0u64,
            server_state: ServerState::Candidator,
            election_timeout_duration: Duration::from_millis(150),
            heartbeat_timeout_duration: Duration::from_millis(150),
            quorum: LinkedList::new(),
            snapshot: 0u64,
        }
    }
}

impl RaftRpc for RaftServer {
    fn RequestVotes() {
        todo!()
    }

    fn AppendEntries() {
        todo!()
    }

    fn Snapshots(self: Self) -> u64 {
        self.snapshot
    }
}

trait RaftServerTrait {
    fn serving();

    fn take_message();

    fn send_message();

    fn execute_cmd();
}