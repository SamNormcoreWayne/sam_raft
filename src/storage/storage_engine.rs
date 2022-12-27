use super::entry::Entry;

pub trait StorageEngine {
    fn all_entries(&self) -> Entry;
}