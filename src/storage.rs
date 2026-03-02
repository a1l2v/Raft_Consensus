// src/storage.rs

use std::collections::HashMap;

use raft::eraftpb::{ConfState, Entry, HardState};
use raft::prelude::Snapshot;
use raft::storage::GetEntriesContext;
use raft::{Error as RaftError, RaftState, Result as RaftResult, Storage, StorageError};

pub struct MemStorage {
    pub logs: Vec<Entry>, // logs[0] is dummy
    pub hard_state: HardState,
    pub conf_state: ConfState,
    pub apply_index: u64,
    pub kv_store: HashMap<String, String>,
}

impl MemStorage {
    pub fn new(peers: Vec<u64>) -> Self {
        let mut conf_state = ConfState::default();
        conf_state.set_voters(peers);

        // IMPORTANT: Insert dummy entry at index 0
        let mut dummy = Entry::default();
        dummy.set_index(0);
        dummy.set_term(0);

        MemStorage {
            logs: vec![dummy], // index 0 reserved
            hard_state: HardState::default(),
            conf_state,
            apply_index: 0,
            kv_store: HashMap::new(),
        }
    }

    pub fn append(&mut self, entries: &[Entry]) {
        for e in entries {
            let index = e.get_index() as usize;

            // Overwrite if index already exists (conflict resolution)
            if index < self.logs.len() {
                self.logs.truncate(index);
            }

            self.logs.push(e.clone());
        }
    }

    pub fn apply_entry(&mut self, entry: &Entry) {
        self.apply_index = entry.get_index();

        if entry.get_data().is_empty() {
            return;
        }

        let cmd = String::from_utf8(entry.get_data().to_vec()).unwrap();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.len() == 3 && parts[0] == "SET" {
            self.kv_store
                .insert(parts[1].to_string(), parts[2].to_string());
        }
    }

    pub fn set_hard_state(&mut self, hs: HardState) {
        self.hard_state = hs;
    }
}

impl Storage for MemStorage {
    fn initial_state(&self) -> RaftResult<RaftState> {
        Ok(RaftState {
            hard_state: self.hard_state.clone(),
            conf_state: self.conf_state.clone(),
        })
    }

    fn entries(
        &self,
        low: u64,
        high: u64,
        _max_size: impl Into<Option<u64>>,
        _context: GetEntriesContext,
    ) -> RaftResult<Vec<Entry>> {
        if low == 0 || high as usize > self.logs.len() {
            return Err(RaftError::Store(StorageError::Unavailable));
        }

        Ok(self.logs[low as usize..high as usize].to_vec())
    }

    fn term(&self, idx: u64) -> RaftResult<u64> {
        if idx == 0 {
            return Ok(0);
        }

        if idx as usize >= self.logs.len() {
            return Err(RaftError::Store(StorageError::Unavailable));
        }

        Ok(self.logs[idx as usize].get_term())
    }

    fn first_index(&self) -> RaftResult<u64> {
        // First real entry index
        Ok(1)
    }

    fn last_index(&self) -> RaftResult<u64> {
        // logs[0] is dummy
        Ok((self.logs.len() - 1) as u64)
    }

    fn snapshot(&self, _request_index: u64, _to: u64) -> RaftResult<Snapshot> {
        Err(RaftError::Store(
            StorageError::SnapshotTemporarilyUnavailable,
        ))
    }
}
