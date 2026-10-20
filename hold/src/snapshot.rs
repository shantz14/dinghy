use std::{collections::BTreeMap, ffi::OsStr, fs::{self, File, OpenOptions, read_dir}, io::{self, Write}, path::{Path, PathBuf}};

use bincode::deserialize;
use crc32fast::hash;
use serde::{Deserialize, Serialize};

use crate::kv::Store;

const SNAPSHOT_FOLDER: &str = "snap";

#[derive(Deserialize, Debug, PartialEq)]
pub struct Snapshot {
    pub revision: u64,
    pub entries: BTreeMap<Vec<u8>, Vec<u8>>,
}

// For writing
#[derive(Serialize, Debug, PartialEq)]
pub struct SnapshotRef<'a> {
    pub revision: u64,
    pub entries: &'a BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Store {
    pub fn take_snapshot(&self) -> io::Result<()> {
        let snap = SnapshotRef {
            revision: self.revision,
            entries: &self.map,
        };
        let serialized: Vec<u8> = bincode::serialize(&snap).map_err(io::Error::other)?;
        let checksum: u32 = crc32fast::hash(&serialized);

        let mut snapshot: Vec<u8> = Vec::with_capacity(serialized.len() + 4);
        snapshot.extend_from_slice(&checksum.to_le_bytes());
        snapshot.extend_from_slice(&serialized);

        // Create snapshot file
        let snap_path = self.data_dir.join(SNAPSHOT_FOLDER).join(format!("{:016}.snap.tmp", self.revision));
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true) 
            .open(&snap_path)?;
        file.write_all(&snapshot)?;
        file.sync_data()?;
        fs::rename(&snap_path, self.data_dir.join(SNAPSHOT_FOLDER).join(format!("{:016}.snap", self.revision)))?;
        let dir = File::open(self.data_dir.join(SNAPSHOT_FOLDER))?;
        dir.sync_all()?;

        Ok(())
    }

}

pub fn load_latest_snapshot(data_dir: &Path) -> io::Result<Option<Snapshot>> {
    let entries = read_dir(data_dir.join(SNAPSHOT_FOLDER))?;

    let mut snapshots: Vec<PathBuf> = entries
        .filter_map(|res| res.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension() == Some(OsStr::new("snap")))
        .collect();

    if snapshots.is_empty() {
        return Ok(None);
    }

    snapshots.sort_by(|a, b| b.file_name().expect("failed to get snapshot files").cmp(a.file_name().expect("failed to get snapshot files")));

    for snap in &snapshots {
        if let Some(valid_snap) = is_valid_snapshot(snap)? {
            return Ok(Some(valid_snap));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "all snapshots are corrupt",
    ))
}

fn is_valid_snapshot(snap_path: &PathBuf) -> io::Result<Option<Snapshot>> {
    let bytes: Vec<u8> = fs::read(snap_path)?;
    let checksum = &bytes[..4];
    let snapshot = &bytes[5..];

    if hash(checksum) != hash(snapshot) {
        return Ok(None);
    }

    let valid_snap = bincode::deserialize::<Snapshot>(snapshot);
    match valid_snap {
        Ok(res) => {
            Ok(Some(res))
        }
        Err(_e) => {
            Ok(None)
        }
    }
}


