//! A backend to store the signer state in.

pub use gl_client::persist::State;
use log::debug;
use thiserror::Error;
use tonic::async_trait;

#[derive(Debug, Error)]
pub enum Error {
    /// underlying database error
    #[error("database error: {0}")]
    Sled(#[from] ::sled::Error),
    #[error("state corruption: {0}")]
    CorruptState(#[from] serde_json::Error),
    #[error("unhandled error: {0}")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait]
pub trait StateStore: Send + Sync {
    async fn write(&self, state: State) -> Result<(), Error>;
    async fn read(&self) -> Result<State, Error>;
}

/// A StateStore that uses `sled` as its storage backend
pub struct SledStateStore {
    db: sled::Db,
}

impl SledStateStore {
    pub fn new(path: std::path::PathBuf) -> Result<SledStateStore, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }
}

use sled::transaction::TransactionError;
impl From<TransactionError<Error>> for Error {
    fn from(e: TransactionError<Error>) -> Self {
        match e {
            TransactionError::Abort(e) => e,
            TransactionError::Storage(e) => Error::Sled(e),
        }
    }
}

const SLED_KEY: &str = "signer_state";

#[async_trait]
impl StateStore for SledStateStore {
    //RANDY_COMMENTED
    async fn read(&self) -> Result<State, Error> {
        //Match the return value of the db query using SLED_KEY('signer_state')
        match self.db.get(SLED_KEY)? {
            //If there's nothing
            None => {
                //log that we're initializing a new signer state
                debug!("Initializing a new signer state");
                //return a new signer state
                Ok(State::new())
            }
            //If there was a byte vec found, then read it form the slice
            Some(v) => Ok(serde_json::from_slice(&v)?),
        }
    }

    //RANDY_COMMENTED
    //Write the raw json of the signer state to the sled db
    async fn write(&self, state: State) -> Result<(), Error> {
        //Convert the state to a byte arr using serde_json
        let raw = serde_json::to_vec(&state)?;

        //Insert the json signer state store into the sled db under the key 'signer_state'
        self.db
            .insert(SLED_KEY, raw)
            .map(|_v| ())
            .map_err(|e| e.into())
    }
}
