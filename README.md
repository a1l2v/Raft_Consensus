# 🧠 Minimal In-Memory Raft Key-Value Store (Rust)

A fully functional, in-memory distributed Key-Value store built using the Raft consensus algorithm in Rust.

This project demonstrates:

- Leader election
- Log replication
- State machine application
- Commit handling
- Multi-node cluster simulation (single process)
- Interactive CLI-based command input

The system simulates a 3-node Raft cluster running inside a single process and ensures strong consistency across all nodes.

---

## 🚀 Features

✅ Automatic leader election  
✅ Log replication across 3 nodes  
✅ Majority-based commit  
✅ Deterministic state machine application  
✅ In-memory storage engine  
✅ Interactive CLI command input  
✅ Clean log output (important events only)

---

## 🏗 Architecture Overview

The system simulates a distributed cluster with:

- 3 Raft nodes
- Message passing via an in-memory transport layer
- A replicated log
- A key-value state machine

Even though all nodes run in one process, the Raft protocol remains fully enforced.

---

## 📂 Project Structure

```

src/
│
├── main.rs
├── node.rs
├── storage.rs
├── keys.rs
├── transport.rs
│
└── (http.rs - planned future enhancement)

```

---

## 📄 File-by-File Explanation

---

### 1️⃣ main.rs

This is the entry point of the program.

Responsibilities:

- Create 3 Raft nodes
- Initialize transport layer
- Run election loop until leader is elected
- Start interactive CLI
- Forward commands to leader
- Drive replication via ticks
- Print final cluster state

Execution Flow:

1. Initialize nodes
2. Run election ticks
3. Detect leader
4. Enter CLI loop
5. Propose commands to leader
6. Tick cluster to replicate
7. Print state

---

### 2️⃣ node.rs

This is the core Raft wrapper.

Responsibilities:

- Wrap `RawNode` from the `raft` crate
- Handle `tick()` calls
- Handle `on_ready()` processing
- Apply committed log entries
- Maintain in-memory KV state machine
- Propose new commands

Key Components:

- `tick()` → Advances logical clock
- `propose()` → Sends new client command to Raft
- `on_ready()` → Processes:
  - Outbound messages
  - Committed entries
  - State machine application
  - Log persistence (in-memory)

State Machine:

Commands follow format:
```

SET key value

```

Example:
```

SET x 10

````

Applied state stored in:

```rust
kv_store: HashMap<String, String>
````

---

### 3️⃣ storage.rs

Implements a minimal in-memory storage engine compatible with Raft.

Responsibilities:

* Store raft log entries
* Maintain hard state
* Track commit index
* Provide log read/write interface

This replaces disk-based storage (like RocksDB) with pure memory for simplicity.

---

### 4️⃣ keys.rs

Defines internal key structures for log and metadata organization.

Used for:

* Log indexing
* State tracking
* Future extensibility

Currently minimal but designed for extension to persistent storage.

---

### 5️⃣ transport.rs

Implements in-memory message passing between nodes.

Responsibilities:

* Deliver Raft messages between nodes
* Simulate network communication
* Process outgoing messages
* Route them to appropriate node

Important:

Even though nodes are in the same process, transport enforces real Raft message flow.

No shortcuts.

---

## 🧠 How Raft Works Here

### 🗳 Leader Election

* Each node starts as follower.
* After random election timeout:

  * Node becomes candidate.
  * Sends `RequestVote` RPCs.
* Majority vote → becomes leader.

---

### 📜 Log Replication

When client proposes:

1. Leader appends entry to its log.
2. Sends `AppendEntries` to followers.
3. Followers replicate log.
4. Once majority replicated → commit.
5. Apply to state machine.

---

### 💾 State Machine Application

Committed entries are applied:

```
SET key value
```

All nodes eventually reach identical KV state.

---

## 🖥 Running the Project

### Step 1: Clone

```bash
git clone <repo>
cd <repo>
```

### Step 2: Run

```bash
cargo run
```

Expected output:

```
Starting stable in-memory Raft cluster...
✅ Leader elected: 1
>
```

---

## 🧪 Example Usage

```
> SET x 10
Node1 KV: {"x": "10"}
Node2 KV: {"x": "10"}
Node3 KV: {"x": "10"}

> SET y 50
Node1 KV: {"x": "10", "y": "50"}
Node2 KV: {"x": "10", "y": "50"}
Node3 KV: {"x": "10", "y": "50"}

> exit
```

All nodes remain consistent.

---

## ⚙ Design Decisions

### Why In-Memory?

To focus on Raft correctness before persistence complexity.

### Why Single Process?

To simplify debugging and understanding of Raft behavior.

### Why Manual Tick Driving?

To control election timing deterministically.

---

## 📊 Guarantees

✔ Strong consistency
✔ Leader-based replication
✔ Majority commit
✔ Deterministic state machine

---

## 🔮 Future Enhancements

### 1️⃣ HTTP API Layer (Planned)

Add REST endpoints:

* `POST /set`
* `GET /get`
* `POST /raft` (internal)

This will allow:

* External client access
* Real distributed deployment
* Multi-process cluster support

---

### 2️⃣ Multi-Process Cluster

Run each node as separate binary:

```
node --id 1
node --id 2
node --id 3
```

Communication via HTTP transport.

---

### 3️⃣ Persistent Storage

Replace in-memory storage with:

* RocksDB
* Sled
* File-backed WAL

To survive crashes.

---

### 4️⃣ Failure Simulation

* Kill leader
* Network partitions
* Snapshot handling

---

### 5️⃣ Snapshot Support

Implement snapshotting for:

* Log compaction
* Memory efficiency
* Faster recovery

---

## 📚 Learning Outcomes

This project demonstrates deep understanding of:

* Raft internals
* Distributed consensus
* Leader election mechanics
* Log replication
* State machine application
* Rust ownership + async messaging

---

