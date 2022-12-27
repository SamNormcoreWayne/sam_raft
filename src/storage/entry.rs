enum EntryType {
    EntryNormal,
    EntryConfChange,
    EntryConfChangeV2,
}

pub struct Entry {
    entry_type: EntryType,
    term: u64,
    index: u64,
    meta: MetaInfo,
}

struct MetaInfo {
    data: Vec<u8>,
    context: Vec<u8>,
}