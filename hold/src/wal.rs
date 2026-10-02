use std::{collections::BTreeMap, fs::{self, File}, io::{self, Write}, path::Path};

use crate::kv::Store;

use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WalRecord {
    Put {revision: u64, key: Vec<u8>, value: Vec<u8>},
    Delete {revision: u64, key: Vec<u8>}
}

impl Store {
    pub fn wal_log(&mut self, data: WalRecord) -> io::Result<()> {
        let payload: Vec<u8> = bincode::serialize(&data).map_err(io::Error::other)?;
        let len: u32 = payload.len() as u32;
        let checksum: u32 = crc32fast::hash(&payload);
        let mut frame = Vec::with_capacity(4 + 4 + payload.len());
        frame.extend_from_slice(&len.to_le_bytes());
        frame.extend_from_slice(&checksum.to_le_bytes());
        frame.extend_from_slice(&payload);

        let log_len = self.wal_fd.metadata()?.len();
        if let Err(e) = self.wal_fd.write_all(&frame) {
            self.wal_fd.set_len(log_len).expect(&format!("a write to the WAL failed and the file could not be truncated: {}", e));
        }
        if let Err(e) = self.wal_fd.sync_data() {
            self.wal_fd.set_len(log_len).expect(&format!("sync_data after a WAL write failed and the file could not be truncated: {}", e));
        }

        self.revision += 1;
        Ok(())
    }

    pub fn restore(&mut self, wal_path: &Path) -> io::Result<()> {
        let mut offset = 0;
        let bytes = fs::read(wal_path)?;

        while offset < bytes.len() {
            let Some(header) = bytes.get(offset..offset + 8) else {break}; // break on torn header, cant

            let length = u32::from_le_bytes(header[0..4].try_into().unwrap()) as usize;
            let checksum = u32::from_le_bytes(header[4..8].try_into().unwrap());

            let Some(payload) = bytes.get(offset + 8..offset + 8 + length) else {break}; // break on torn payload, cant
            if crc32fast::hash(payload) != checksum {break} // corrupt

            let Ok(record) = bincode::deserialize::<WalRecord>(payload) else {break};

            // add to map
            match record {
                WalRecord::Put { revision, key, value } => {
                    self.map.insert(key, value);
                    self.revision = revision;
                },
                WalRecord::Delete { revision, key } => {
                    self.map.remove(&key);
                    self.revision = revision;
                }
            }

            offset += 8 + length;
        }
        self.wal_fd.set_len(offset as u64)?;

        Ok(())
    }
}




