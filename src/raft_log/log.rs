use crate::storage::storage_engine::StorageEngine;
use super::pending_log::PendingLog;
pub struct RaftLog<T: StorageEngine> {
    storage: T,

    // logs not in StorageEngine, and will be put into 
    pending_logs: PendingLog,

    // the latest log in storage
    // applied_id <= commited_id
    commited_id: u64,

    // the latest log persisted in storage
    // persisted_id < pending_logs.max && applied <= persisted_id
    persisted_id: u64,

    // the latest log applied by upper applications
    // applied_id <= min(commited_id, persisted_id)
    applied_id: u64,
}