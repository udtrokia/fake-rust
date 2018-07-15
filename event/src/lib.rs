use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct EventHash {
    pub hash: Vec<u8>,
}

#[derive(Debug)]
pub struct Message {}

#[derive(Debug)]
pub struct Event {
    pub event_hash: EventHash,
}

#[derive(Debug)]
pub struct Block {}

#[derive(Debug)]
pub struct BlockHeader {}

#[derive(Debug)]
pub struct BlockBody {}

impl Event {
    pub fn from_bytes(bytes: Vec<u8>) -> Event {
        Event {
            event_hash: EventHash { hash: bytes },
        }
    }
}

impl Default for Event {
    fn default() -> Event {
        Event {
            event_hash: EventHash { hash: vec![0, 0] },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
