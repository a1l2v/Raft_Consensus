// src/transport.rs

use crate::node::Node;
use raft::eraftpb::Message;
use std::collections::HashMap;

pub struct Transport {
    pub nodes: HashMap<u64, Node>,
}

impl Transport {
    pub fn new(nodes: HashMap<u64, Node>) -> Self {
        Transport { nodes }
    }

    pub fn send(&mut self, msg: Message) {
        let to = msg.get_to();
        if let Some(node) = self.nodes.get_mut(&to) {
            node.step(msg);
        }
    }

    pub fn tick_all(&mut self) {
        for node in self.nodes.values_mut() {
            node.tick();
        }
    }

    pub fn process_ready(&mut self) {
        // Drain the full ready/message cascade before returning so vote responses
        // are processed immediately.
        loop {
            let mut made_progress = false;
            let mut outbound = vec![];

            let mut ids: Vec<u64> = self.nodes.keys().copied().collect();
            ids.sort_unstable();

            for id in ids {
                let node = self
                    .nodes
                    .get_mut(&id)
                    .expect("node id must exist during ready processing");

                if !node.raft.has_ready() {
                    continue;
                }

                made_progress = true;
                let mut ready = node.raft.ready();

                if !ready.messages().is_empty() {
                    outbound.extend(ready.take_messages());
                }

                // Persist hard state before advancing.
                if let Some(hs) = ready.hs() {
                    node.raft.mut_store().set_hard_state(hs.clone());
                }

                // Persist newly appended entries.
                let entries = ready.entries().to_vec();
                if !entries.is_empty() {
                    node.raft.mut_store().append(&entries);
                }

                // Apply committed entries to the in-memory state machine.
                for entry in ready.take_committed_entries() {
                    if !entry.get_data().is_empty() {
                        println!("📝 Node {} committed index {}", node.id, entry.get_index());
                        let command = String::from_utf8_lossy(entry.get_data());
                        println!("💾 Node {} applied: {}", node.id, command);
                        node.raft.mut_store().apply_entry(&entry);
                    }
                }

                if !ready.persisted_messages().is_empty() {
                    outbound.extend(ready.take_persisted_messages());
                }

                let mut light_rd = node.raft.advance(ready);
                outbound.extend(light_rd.take_messages());

                for entry in light_rd.take_committed_entries() {
                    if !entry.get_data().is_empty() {
                        println!("📝 Node {} committed index {}", node.id, entry.get_index());
                        let command = String::from_utf8_lossy(entry.get_data());
                        println!("💾 Node {} applied: {}", node.id, command);
                        node.raft.mut_store().apply_entry(&entry);
                    }
                }

                node.raft.advance_apply();
            }

            for msg in outbound {
                self.send(msg);
            }

            if !made_progress {
                break;
            }
        }
    }
}
