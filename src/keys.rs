pub const HARD_STATE_KEY: &str = "raft_hard_state";
pub const APPLY_INDEX_KEY: &str = "raft_apply_index";
pub const NODE_ID_KEY: &str = "node_id";

pub fn raft_log_key(index: u64) -> String {
    format!("raft_log_{}", index)
}

pub fn decode_raft_log_key(key: &str) -> Option<u64> {
    key.strip_prefix("raft_log_")?.parse::<u64>().ok()
}

pub fn data_key(key: &str) -> String {
    format!("user:{}", key)
}
