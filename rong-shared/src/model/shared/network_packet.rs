use serde::{Deserialize, Serialize};

/*  Network packet wrapper for all messages */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkPacket<T> {
    sequence: u32,
    timestamp: u64,
    payload: T,
}

impl<T> NetworkPacket<T> {
    pub fn new(sequence: u32, timestamp: u64, payload: T) -> Self {
        NetworkPacket {
            sequence,
            timestamp,
            payload,
        }
    }

    pub fn get_sequence(&self) -> u32 {
        self.sequence
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_payload(&self) -> &T {
        &self.payload
    }

    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }

    pub fn set_timesamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    pub fn set_payload(&mut self, payload: T) {
        self.payload = payload;
    }
}
