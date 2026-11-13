use std::{collections::BTreeMap, fs::{self, File, OpenOptions}, io, path::PathBuf};
use std::path::Path;

use crate::{server::hold::KeyValue, snapshot::load_latest_snapshot, wal::{self, WalRecord}};

const TRIGGER: u8 = 5;

#[derive(Debug)]
pub struct Store {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
    pub revision: u64,
    pub wal_fd: File,
    pub data_dir: PathBuf,
    writes_since_snap: u8,
}

impl Store {
    pub fn new(data_dir: &Path) -> io::Result<Self> {
        // Open the WAL file
        fs::create_dir_all(&data_dir)?;
        let wal_path = data_dir.join("0000000000000001.wal");
        let wal_fd = OpenOptions::new()
            .append(true)
            .create(true) 
            .open(&wal_path)?;
            // TODO segmented WAL files

        // Create the snapshot folder if it doesn't exist
        fs::create_dir_all(&data_dir.join("snap"))?;

        // Load latest snapshot if it exists
        let revision: u64;
        let map: BTreeMap<Vec<u8>, Vec<u8>>;
        match load_latest_snapshot(data_dir)? {
            Some(snapshot) => {
                revision = snapshot.revision;
                map = snapshot.entries;
            }
            None => {
                revision = 0;
                map = BTreeMap::new();
            }
        };

        let mut store = Self {
            map: map,
            revision: revision,
            wal_fd: wal_fd,
            data_dir: data_dir.to_path_buf(),
            writes_since_snap: 0,
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

        self.writes_since_snap += 1;
        self.try_snapshot()?;
        Ok(self.revision)
    }

    pub fn delete(&mut self, key: Vec<u8>) -> io::Result<u64> {
        self.wal_log(WalRecord::Delete { revision: self.revision+1, key: key.clone() })?;
        self.map.remove(&key);

        self.writes_since_snap += 1;
        self.try_snapshot()?;
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

    /// Checks if a snapshot should be triggered and triggers a snapshot if need be
    /// Resets trigger vars if one is triggered (e.g., writes_since_snap = 0)
    fn try_snapshot(&mut self) -> io::Result<()> {
        if self.writes_since_snap >= TRIGGER {
            self.take_snapshot()?;
            self.writes_since_snap = 0;

            //self.rotate_wal()?;
        }
        Ok(())
    }

}

