pub trait RaftRpc {
    fn RequestVotes();
    fn AppendEntries();
    fn Snapshots(self: Self) -> u64;
}