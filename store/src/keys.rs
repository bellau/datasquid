use byteorder::{BigEndian, ByteOrder};

pub static MAX_MSG_ID_KEY: &'static [u8] = b"Smax_msg_id";
pub static MAX_MOD_ID_KEY: &'static [u8] = b"Smax_mod_id";

pub static COLLECTION_SEQ_KEY_PREFIX: u8 = 'c' as u8;
pub static COLLECTION_INDEX_KEY_PREFIX: u8 = 'C' as u8;

pub static MOD_LOG_ENTRY_KEY_PREFIX: u8 = 'm' as u8;
pub static MOD_COLLECTION_LOG_ENTRY_KEY_PREFIX: u8 = 'n' as u8;
pub static INDEX_KEY_PREFIX: u8 = 'i' as u8;

pub struct ModLogEntryKey {
    key_value: [u8; 1 + 4 + 4 + 4 + 1],
}

impl ModLogEntryKey {
    pub fn global() -> ModLogEntryKey {
        let mut key_value = [0; 1 + 4 + 4 + 4 + 1];
        key_value[0] = MOD_LOG_ENTRY_KEY_PREFIX;

        ModLogEntryKey {
            key_value: key_value,
        }
    }
    pub fn collection() -> ModLogEntryKey {
        let mut key_value = [0; 1 + 4 + 4 + 4 + 1];
        key_value[0] = MOD_COLLECTION_LOG_ENTRY_KEY_PREFIX;

        ModLogEntryKey {
            key_value: key_value,
        }
    }

    pub fn set_global(&mut self, is_global: bool) {
        self.key_value[0] = if is_global {
            MOD_LOG_ENTRY_KEY_PREFIX
        } else {
            MOD_COLLECTION_LOG_ENTRY_KEY_PREFIX
        };
    }

    pub fn set_msg_id(&mut self, msg_id: u32) {
        let mut index = 1;
        index += 4;
        index += 4;
        BigEndian::write_u32(&mut self.key_value[index..index + 4], msg_id);
    }

    pub fn set_values(&mut self, mod_id: u32, collection_id: u32, msg_id: u32, kind: u8) {
        let mut index = 1;
        BigEndian::write_u32(&mut self.key_value[index..index + 4], mod_id);
        index += 4;
        BigEndian::write_u32(&mut self.key_value[index..index + 4], collection_id);
        index += 4;
        BigEndian::write_u32(&mut self.key_value[index..index + 4], msg_id);
        index += 4;
        self.key_value[index] = kind;
    }

    // m : mod_id : collection_id : msg_id : kind ?
    pub fn get_value(&self) -> &[u8] {
        &self.key_value
    }
}

pub struct CollectionSeqKey {
    key_value: [u8; 6],
}

impl CollectionSeqKey {
    pub fn new() -> CollectionSeqKey {
        let mut key_value = [0; 6];
        key_value[0] = 'S' as u8;
        key_value[1] = COLLECTION_SEQ_KEY_PREFIX;
        CollectionSeqKey {
            key_value: key_value,
        }
    }

    pub fn set_collection(&mut self, id: u32) {
        BigEndian::write_u32(&mut self.key_value[2..], id);
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key_value
    }
}

pub struct CollectionIndexKey {
    key_value: [u8; 6],
}

impl CollectionIndexKey {
    pub fn new() -> CollectionIndexKey {
        let mut key_value = [0; 6];
        key_value[0] = INDEX_KEY_PREFIX;
        key_value[1] = 'r' as u8;
        CollectionIndexKey {
            key_value: key_value,
        }
    }

    pub fn set_collection(&mut self, id: u32) {
        BigEndian::write_u32(&mut self.key_value[2..], id);
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key_value
    }
}

pub enum IndexKind {
    UNIQUE,
    ROARMAP,
}

pub struct IndexedWordKey {
    key_value: Vec<u8>,
}

fn create_base_index_key(
    kind: IndexKind,
    msg_kind: u32,
    property: u32,
    pre_allocate: usize,
) -> Vec<u8> {
    let mut key_value = Vec::with_capacity(1 + 1 + 4 + 4 + pre_allocate);
    key_value.push(INDEX_KEY_PREFIX);
    match kind {
        IndexKind::UNIQUE => {
            key_value.push(b'u');
        }
        IndexKind::ROARMAP => {
            key_value.push(b'r');
        }
    }

    key_value.extend(&[0; 8]);
    BigEndian::write_u32(&mut key_value[2..2 + 4], msg_kind);
    BigEndian::write_u32(&mut key_value[2 + 4..2 + 4 + 4], property);

    key_value
}

fn base_index_key_set_suffix(key_value: &mut Vec<u8>, suffix: &[u8]) {
    key_value.truncate(1 + 1 + 4 + 4);
    key_value.extend(suffix);
}

impl IndexedWordKey {
    pub fn new(msg_kind: u32, property: u32) -> IndexedWordKey {
        IndexedWordKey {
            key_value: create_base_index_key(IndexKind::ROARMAP, msg_kind, property, 20),
        }
    }

    pub fn set_word(&mut self, value: &str) {
        base_index_key_set_suffix(&mut self.key_value, value.as_bytes());
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key_value[..]
    }
}
