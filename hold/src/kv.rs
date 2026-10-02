use std::{collections::BTreeMap, fs::{self, File, OpenOptions}, io};
use std::path::Path;

use crate::{server::hold::KeyValue, wal::{self, WalRecord}};

#[derive(Debug)]
pub struct Store {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
    pub revision: u64,
    pub wal_fd: File,
}

impl Store {
    pub fn new(data_dir: &Path) -> io::Result<Self> {
        // Open the WAL file
        fs::create_dir_all(&data_dir)?;
        let wal_path = data_dir.join("0000000000000001.wal");

        let file = OpenOptions::new()
            .append(true)
            .create(true) 
            .open(&wal_path)?;
            // TODO segmented WAL files
        let mut store = Self {
            map: BTreeMap::new(),
            revision: 0,
            wal_fd: file,
        };
        store.restore(&wal_path)?;

        Ok(store)
    }

    pub fn get(&self, key: Vec<u8>) -> Option<KeyValue> {
        let value = self.map.get(&key);
        match value {
            Some(value) => Some(KeyValue {
                key: key,
                value: value.clone(),
            }),
            None => None
        } 
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> io::Result<u64> {
        self.wal_log(WalRecord::Put { revision: self.revision+1, key: key.clone(), value: value.clone() })?;
        self.map.insert(key, value);
        Ok(self.revision)
    }

    pub fn delete(&mut self, key: Vec<u8>) -> io::Result<u64> {
        self.wal_log(WalRecord::Delete { revision: self.revision+1, key: key.clone() })?;
        self.map.remove(&key);
        Ok(self.revision)
    }

    pub fn range(&self, prefix: Vec<u8>) -> Vec<KeyValue> {
        self.map
            .range(prefix.clone()..)
            .take_while(|(k, _)| k.starts_with(&prefix))
            .map(|(k, v)| KeyValue {
                key: k.clone(),
                value: v.clone(),
            })
            .collect()
    }

}

