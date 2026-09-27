use std::collections::BTreeMap;

use crate::server::hold::KeyValue;

#[derive(Debug)]
pub struct Store {
    map: BTreeMap<Vec<u8>, Vec<u8>>
}

impl Default for Store {
    fn default() -> Self {
        Self { 
            map: BTreeMap::new() 
        }
    }
}

impl Store {
    pub fn get(&self, key: Vec<u8>) -> Option<KeyValue> {
        let value = self.map.get(&key);
        match value {
            Some(value) => return Some(KeyValue {
                key: key,
                value: value.clone(),
            }),
            None => return None
        } 
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: Vec<u8>) {
        self.map.remove(&key);
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

