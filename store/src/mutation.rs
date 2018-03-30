use rocksdb::{ColumnFamily, WriteBatch, DB};
use error::StoreError;

use byteorder::{BigEndian, ByteOrder};
use keys::*;
use index::*;
use MsgId;
use CollectionId;
use ModId;
use Msg;

struct CollectionForMutation {
    id_seq: MsgId,
    id: CollectionId,
}

pub struct Mutation {
    batch: WriteBatch,
    mod_id_seq: ModId,
    msg_id_seq: MsgId,
    collections: Vec<CollectionForMutation>,
    col_cf: ColumnFamily,
    mod_cf: ColumnFamily,
}

enum LogEntryKind {
    ADD,
    REMOVE,
    UPDATE,
}
struct LogEntry {
    mod_id: ModId,
    collection_id: CollectionId,
    msg_id: MsgId,
    collection_msg_id: MsgId,
    kind: LogEntryKind,
}

pub struct CollectionMsg {
    col_id: CollectionId,
    msg_id: MsgId,
}

pub struct MsgCollections {
    pub id: MsgId,
    pub mod_id: ModId,
    collections: Vec<CollectionMsg>,
}

impl LogEntry {
    fn remove_entry(
        mod_id: ModId,
        col_id: CollectionId,
        msg_id: MsgId,
        col_msg_id: MsgId,
    ) -> LogEntry {
        LogEntry {
            mod_id: mod_id,
            collection_id: col_id,
            msg_id: msg_id,
            collection_msg_id: col_msg_id,
            kind: LogEntryKind::REMOVE,
        }
    }

    fn add_entry(
        mod_id: ModId,
        col_id: CollectionId,
        msg_id: MsgId,
        col_msg_id: MsgId,
    ) -> LogEntry {
        LogEntry {
            mod_id: mod_id,
            collection_id: col_id,
            msg_id: msg_id,
            collection_msg_id: col_msg_id,
            kind: LogEntryKind::ADD,
        }
    }
}
impl MsgCollections {
    pub fn get_collection(&self, col: CollectionId) -> Result<&CollectionMsg, StoreError> {
        let col = self.collections
            .iter()
            .find(|col_msg| col_msg.col_id == col);
        if let Some(col) = col {
            Ok(col)
        } else {
            Err(StoreError::DbError("collection not present".to_string()))
        }
    }
}

static COLLECTIONS_CF: &'static str = "collections";
static MOD_CF: &'static str = "mod";
static SEQS_CF: &'static str = "sequences";

impl Mutation {
    pub fn prepare(db: &DB, cols: &Vec<CollectionId>) -> Result<Mutation, StoreError> {
        let col_cf = if let Some(cf) = db.cf_handle(COLLECTIONS_CF) {
            cf
        } else {
            return Err(StoreError::DbError("family not found".to_string()));
        };
        let mod_cf = if let Some(cf) = db.cf_handle(MOD_CF) {
            cf
        } else {
            return Err(StoreError::DbError("family not found".to_string()));
        };

        let seq_cf = if let Some(cf) = db.cf_handle(SEQS_CF) {
            cf
        } else {
            return Err(StoreError::DbError("family not found".to_string()));
        };

        let max_mod_id: ModId = if let Some(value) = db.get_cf(seq_cf, MAX_MOD_ID_KEY)? {
            BigEndian::read_u32(&*value)
        } else {
            0 as u32
        };

        let max_msg_id: MsgId = if let Some(value) = db.get_cf(seq_cf, MAX_MSG_ID_KEY)? {
            BigEndian::read_u32(&*value)
        } else {
            0 as u32
        };

        let mut msgseq_key = CollectionSeqKey::new();
        let mut mcols = Vec::new();
        for col in cols {
            msgseq_key.set_collection(*col);
            let max_col_msg_id = if let Some(value) = db.get_cf(col_cf, msgseq_key.get_key())? {
                Ok(BigEndian::read_u32(&*value))
            } else {
                Err(StoreError::DbError("max msg id not found".to_string()))
            }?;
            mcols.push(CollectionForMutation {
                id_seq: max_col_msg_id,
                id: *col,
            });
        }
        Ok(Mutation::new(max_msg_id, max_mod_id, col_cf, mod_cf, mcols))
    }

    pub fn commit(&self, db: &DB) -> Result<(), StoreError> {
        Ok(())
    }

    fn new(
        max_msg_id: MsgId,
        max_mod_seq: ModId,
        col_cf: ColumnFamily,
        mod_cf: ColumnFamily,
        collections: Vec<CollectionForMutation>,
    ) -> Mutation {
        Mutation {
            msg_id_seq: max_msg_id,
            mod_id_seq: max_mod_seq,
            batch: WriteBatch::default(),
            collections: collections,
            col_cf: col_cf,
            mod_cf: mod_cf,
        }
    }

    // cols
    fn find_collection(
        &mut self,
        collection_id: u32,
    ) -> Result<&mut CollectionForMutation, StoreError> {
        let cols = &mut self.collections;
        let col = cols.iter_mut().find(|col| col.id == collection_id);
        if let Some(col) = col {
            Ok(col)
        } else {
            Err(StoreError::DbError("collection not present".to_string()))
        }
    }

    pub fn remove_msg_from_collection(
        &mut self,
        msg: &mut MsgCollections,
        col_id: CollectionId,
    ) -> Result<(), StoreError> {
        let mod_id = self.next_mod_id();
        let entry = {
            let col_msg = msg.get_collection(col_id)?;
            LogEntry::remove_entry(mod_id, col_id, msg.id, col_msg.msg_id)
        };

        msg.mod_id = mod_id;
        msg.collections.retain(|c| c.col_id != col_id);
        self.write_log_entry(entry)?;
        self.write_collection_unindex(col_id, msg.id)?;
        Ok(())
    }

    pub fn add_msg_to_collection(
        &mut self,
        msg: &mut MsgCollections,
        col_id: CollectionId,
    ) -> Result<(), StoreError> {
        let mod_id = self.next_mod_id();

        let entry = {
            let col = self.find_collection(col_id)?;
            let col_msg_id = col.next_id_seq();

            LogEntry::add_entry(mod_id, col_id, msg.id, col_msg_id)
        };

        msg.mod_id = mod_id;
        msg.collections.push(CollectionMsg {
            col_id: col_id,
            msg_id: entry.collection_msg_id,
        });
        self.write_log_entry(entry)?;
        self.write_collection_index(col_id, msg.id)?;
        Ok(())
    }

    pub fn create_msg_collections(&mut self) -> MsgCollections {
        let msg_id = self.next_msg_id();
        MsgCollections {
            id: msg_id,
            mod_id: 0,
            collections: vec![],
        }
    }

    pub fn write_msg_index(
        &mut self,
        msg: &MsgCollections,
        msg_index: IndexMutation,
    ) -> Result<(), StoreError> {
        Ok(())
    }

    pub fn write_msg(&mut self, msg_id: MsgId, msg: &Msg) -> Result<(), StoreError> {
        Ok(())
    }

    pub fn write_msg_collection(&mut self, msg: &MsgCollections) -> Result<(), StoreError> {
        Ok(())
    }

    fn write_collection_unindex(
        &mut self,
        col_id: CollectionId,
        msg_id: MsgId,
    ) -> Result<(), StoreError> {
        let mut key = CollectionIndexKey::new();
        key.set_collection(col_id);
        self.batch.merge_cf(
            self.col_cf,
            key.get_key(),
            &OrderedMsgIds::MinusOne(msg_id).get_value()[..],
        )?;
        Ok(())
    }

    fn write_collection_index(
        &mut self,
        col_id: CollectionId,
        msg_id: MsgId,
    ) -> Result<(), StoreError> {
        let mut key = CollectionIndexKey::new();
        key.set_collection(col_id);
        self.batch.merge_cf(
            self.col_cf,
            key.get_key(),
            &OrderedMsgIds::PlusOne(msg_id).get_value()[..],
        )?;
        Ok(())
    }

    fn write_log_entry(&mut self, entry: LogEntry) -> Result<(), StoreError> {
        let mut key = ModLogEntryKey::collection();
        key.set_values(
            entry.mod_id,
            entry.collection_id,
            entry.msg_id,
            match entry.kind {
                LogEntryKind::ADD => 1,
                LogEntryKind::REMOVE => 2,
                LogEntryKind::UPDATE => 3,
            },
        );

        self.batch.put_cf(self.mod_cf, key.get_value(), &[][..])?;
        key.set_global(false);
        key.set_msg_id(entry.collection_msg_id);
        self.batch.put_cf(self.col_cf, key.get_value(), &[][..])?;

        Ok(())
    }

    fn next_mod_id(&mut self) -> u32 {
        self.mod_id_seq += 1;
        self.mod_id_seq
    }

    fn next_msg_id(&mut self) -> u32 {
        self.msg_id_seq += 1;
        self.msg_id_seq
    }
}

impl CollectionForMutation {
    fn next_id_seq(&mut self) -> u32 {
        self.id_seq += 1;
        self.id_seq
    }
}

pub struct IndexMutation {}

impl IndexMutation {
    pub fn new() -> IndexMutation {
        IndexMutation {}
    }
}
