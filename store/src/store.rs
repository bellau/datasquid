use rocksdb::DB;
use std::sync::mpsc::Sender;
use std::sync::mpsc;
use error::StoreError;
use mutation;
use futures::Future;
use futures::Async;
use std::sync::Arc;
use futures::task::Context;
use futures::channel::oneshot;
use Msg;
use shred::MsgIndexer;

pub struct Store {
    db: Arc<DB>,
    data_write_channel: Sender<(WriteAction, oneshot::Sender<Result<(), StoreError>>)>,
}

enum WriteAction {
    Put(Msg, mutation::IndexMutation, Vec<u32>),
}

impl Store {
    pub fn new(db: DB) -> Store {
        use std::thread;
        let (rx, tx) = mpsc::channel();

        let db = Arc::new(db);
        {
            let db = Arc::clone(&db);

            thread::spawn(move || loop {
                let rec: Result<
                    (WriteAction, oneshot::Sender<Result<(), StoreError>>),
                    _,
                > = tx.recv();
                if let Ok(action) = rec {
                    match action.0 {
                        WriteAction::Put(msg, msg_index, cols) => {
                            match Store::do_write(&*db, msg, msg_index, cols) {
                                Ok(_) => {
                                    action.1.send(Ok(())).unwrap();
                                }
                                Err(_) => {
                                    action
                                        .1
                                        .send(Err(StoreError::DbError("write error".to_string())))
                                        .unwrap();
                                }
                            };
                        }
                    }
                } else {
                    break;
                }
            });
        }
        let store = Store {
            db: db,
            data_write_channel: rx,
        };
        store
    }

    pub fn put(&self, msg: Msg, collections: Vec<u32>) -> Result<StoreFuture, StoreError> {
        let (p, c) = oneshot::channel::<Result<(), StoreError>>();

        let msg_index = MsgIndexer::index(&msg);

        let ret = StoreFuture { inner: c };
        self.data_write_channel
            .send((WriteAction::Put(msg, msg_index, collections), p))
            .map_err(|_| StoreError::DbError("could not send write order".to_string()))?;
        Ok(ret)
    }

    fn do_write(
        db: &DB,
        msg: Msg,
        msg_index: mutation::IndexMutation,
        collections: Vec<u32>,
    ) -> Result<(), StoreError> {
        let mut mutation = mutation::Mutation::prepare(db, &collections)?;
        let mut msg_cols = mutation.create_msg_collections();
        for col in collections {
            mutation.add_msg_to_collection(&mut msg_cols, col)?;
        }
        mutation.write_msg_collection(&msg_cols)?;
        mutation.write_msg(msg_cols.id, &msg)?;
        mutation.write_msg_index(&msg_cols, msg_index)?;
        mutation.commit(db)?;
        Ok(())
    }
}

pub struct StoreFuture {
    inner: oneshot::Receiver<Result<(), StoreError>>,
}

impl Future for StoreFuture {
    type Item = Result<(), StoreError>;
    type Error = StoreError;

    fn poll(&mut self, cx: &mut Context) -> Result<Async<Result<(), StoreError>>, StoreError> {
        match self.inner.poll(cx)? {
            Async::Ready(v) => Ok(Async::Ready(v)),
            Async::Pending => Ok(Async::Pending),
        }
    }
}
