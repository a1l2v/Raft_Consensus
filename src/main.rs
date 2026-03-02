mod keys;
mod node;
mod storage;
mod transport;

use node::Node;
use std::collections::HashMap;
use transport::Transport;

fn main() {
    println!("Starting stable in-memory Raft cluster...");

    // Create nodes
    let mut nodes = HashMap::new();
    nodes.insert(1, Node::new(1, vec![1, 2, 3]));
    nodes.insert(2, Node::new(2, vec![1, 2, 3]));
    nodes.insert(3, Node::new(3, vec![1, 2, 3]));

    let mut transport = Transport::new(nodes);

    // ---- Run election until leader found ----
    let mut leader_id = 0;

    for _ in 0..500 {
        // Tick each node independently and process immediately
        for id in 1..=3 {
            transport.nodes.get_mut(&id).unwrap().tick();
            transport.process_ready();
        }

        // Check leader
        if let Some((_, node)) = transport
            .nodes
            .iter()
            .find(|(_, n)| n.raft.raft.leader_id != 0)
        {
            leader_id = node.raft.raft.leader_id;
            break;
        }
    }

    if leader_id == 0 {
        println!("❌ No leader elected!");
        return;
    }

    println!("✅ Leader elected: {}", leader_id);

    // ---- Propose command from leader ----
    transport
        .nodes
        .get_mut(&leader_id)
        .unwrap()
        .propose("SET x 10".to_string());

    // ---- Replication phase ----
    for _ in 0..300 {
        for id in 1..=3 {
            transport.nodes.get_mut(&id).unwrap().tick();
            transport.process_ready();
        }
    }

    // ---- Print KV state ----
    println!("Final state:");
    println!(
        "Node1 KV: {:?}",
        transport
            .nodes
            .get_mut(&1)
            .unwrap()
            .raft
            .mut_store()
            .kv_store
    );
    println!(
        "Node2 KV: {:?}",
        transport
            .nodes
            .get_mut(&2)
            .unwrap()
            .raft
            .mut_store()
            .kv_store
    );
    println!(
        "Node3 KV: {:?}",
        transport
            .nodes
            .get_mut(&3)
            .unwrap()
            .raft
            .mut_store()
            .kv_store
    );
}
