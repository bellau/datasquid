extern crate rocksdb;
extern crate byteorder;
extern crate futures_executor;
extern crate futures;
extern crate roaring;


pub type MsgId = u32;
pub type CollectionId = u32;
pub type ModId=u32;


pub struct MsgPart {
    headers: Vec<(String, String)>,
    data: Vec<u8>,
    children: Vec<MsgPart>,
}
pub struct Msg {
    headers: Vec<(String, String)>,
    body: MsgPart,
}

pub mod index;
pub mod shred;
pub mod error;
pub mod keys;
pub mod mutation;
pub mod store;
