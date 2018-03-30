use rocksdb;
use futures;

#[derive(Debug, Clone, PartialEq)]
pub enum StoreError {
    DbError(String),
}

impl From<rocksdb::Error> for StoreError {
    fn from(e: rocksdb::Error) -> StoreError {
        StoreError::DbError(e.into())
    }
}

impl From<futures::channel::oneshot::Canceled> for StoreError {
    fn from(e: futures::channel::oneshot::Canceled) -> StoreError {
        StoreError::DbError("canceled".to_string())
    }
}
 