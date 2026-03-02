// src/node.rs

use raft::eraftpb::{EntryType, Message};
use raft::{Config, RawNode};
use slog::Logger;

use crate::storage::MemStorage;

pub struct Node {
    pub id: u64,
    pub raft: RawNode<MemStorage>,
}

impl Node {
    pub fn new(id: u64, peers: Vec<u64>) -> Self {
        let storage = MemStorage::new(peers.clone());

        // Silence raft internal logs; we print only high-signal events.
        let logger = Logger::root(slog::Discard, slog::o!());

        let cfg = Config {
            id,
            election_tick: 50,
            heartbeat_tick: 5,
            max_size_per_msg: 1024 * 1024,
            max_inflight_msgs: 256,
            ..Default::default()
        };

        let raft = RawNode::new(&cfg, storage, &logger).unwrap();

        Node { id, raft }
    }

    pub fn tick(&mut self) {
        self.raft.tick();
    }

    pub fn step(&mut self, msg: Message) {
        self.raft.step(msg).unwrap();
    }

    pub fn propose(&mut self, command: String) {
        println!("📨 Node {} proposing: {}", self.id, command);
        self.raft.propose(vec![], command.into_bytes()).unwrap();
    }

    pub fn on_ready(&mut self) {
        if !self.raft.has_ready() {
            return;
        }

        let mut ready = self.raft.ready();

        // Append new entries
        let entries = ready.entries().to_vec();
        if !entries.is_empty() {
            self.raft.mut_store().append(&entries);
        }

        // Apply committed entries
        let committed = ready.committed_entries();
        for entry in committed {
            if entry.get_entry_type() == EntryType::EntryNormal && !entry.get_data().is_empty() {
                self.raft.mut_store().apply_entry(entry);
            }
        }

        // Send outbound messages
        let messages = ready.take_messages();
        for msg in messages {
            println!("Send message to {}", msg.get_to());
        }

        self.raft.advance(ready);
    }
}
