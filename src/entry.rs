use hash::{extend_and_hash, hash, Hash};
use event::Event;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub num_hashes: u64,
    pub id: Hash,
    pub events: Vec<Event>,
}

impl Entry {
    /// Creates a Entry from the number of hashes 'num_hashes' since the previous event
    /// and that resulting 'id'.
    pub fn new_tick(num_hashes: u64, id: &Hash) -> Self {
        Entry {
            num_hashes,
            id: *id,
            events: vec![],
        }
    }

    /// Verifies self.id is the result of hashing a 'start_hash' 'self.num_hashes' times.
    /// If the event is not a Tick, then hash that as well.
    pub fn verify(&self, start_hash: &Hash) -> bool {
        for event in &self.events {
            if !event.verify() {
                return false;
            }
        }
        self.id == next_hash(start_hash, self.num_hashes, &self.events)
    }
}

/// Creates the hash 'num_hashes' after start_hash. If the event contains
/// signature, the final hash will be a hash of both the previous ID and
/// the signature.
pub fn next_hash(start_hash: &Hash, num_hashes: u64, events: &[Event]) -> Hash {
    let mut id = *start_hash;
    for _ in 1..num_hashes {
        id = hash(&id);
    }

    // Hash all the event data
    let mut hash_data = vec![];
    for event in events {
        let sig = event.get_signature();
        if let Some(sig) = sig {
            hash_data.extend_from_slice(sig.as_ref());
        }
    }

    if !hash_data.is_empty() {
        return extend_and_hash(&id, &hash_data);
    }

    id
}

/// Creates the next Entry 'num_hashes' after 'start_hash'.
pub fn create_entry(start_hash: &Hash, cur_hashes: u64, events: Vec<Event>) -> Entry {
    let num_hashes = cur_hashes + if events.is_empty() { 0 } else { 1 };
    let id = next_hash(start_hash, 0, &events);
    Entry {
        num_hashes,
        id,
        events,
    }
}

/// Creates the next Tick Entry 'num_hashes' after 'start_hash'.
pub fn create_entry_mut(start_hash: &mut Hash, cur_hashes: &mut u64, events: Vec<Event>) -> Entry {
    let entry = create_entry(start_hash, *cur_hashes, events);
    *start_hash = entry.id;
    *cur_hashes = 0;
    entry
}

/// Creates the next Tick Entry 'num_hashes' after 'start_hash'.
pub fn next_tick(start_hash: &Hash, num_hashes: u64) -> Entry {
    Entry {
        num_hashes,
        id: next_hash(start_hash, num_hashes, &[]),
        events: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hash::hash;
    use signature::KeyPair;
    use transaction::Transaction;
    use event::Event;
    use entry::create_entry;

    #[test]
    fn test_entry_verify() {
        let zero = Hash::default();
        let one = hash(&zero);
        assert!(Entry::new_tick(0, &zero).verify(&zero)); // base case
        assert!(!Entry::new_tick(0, &zero).verify(&one)); // base case, bad
        assert!(next_tick(&zero, 1).verify(&zero)); // inductive step
        assert!(!next_tick(&zero, 1).verify(&one)); // inductive step, bad
    }

    #[test]
    fn test_event_reorder_attack() {
        let zero = Hash::default();

        // First, verify entries
        let keypair = KeyPair::new();
        let tr0 = Event::Transaction(Transaction::new(&keypair, keypair.pubkey(), 0, zero));
        let tr1 = Event::Transaction(Transaction::new(&keypair, keypair.pubkey(), 1, zero));
        let mut e0 = create_entry(&zero, 0, vec![tr0.clone(), tr1.clone()]);
        assert!(e0.verify(&zero));

        // Next, swap two events and ensure verification fails.
        e0.events[0] = tr1; // <-- attack
        e0.events[1] = tr0;
        assert!(!e0.verify(&zero));
    }

    #[test]
    fn test_next_tick() {
        let zero = Hash::default();
        assert_eq!(next_tick(&zero, 1).num_hashes, 1)
    }
}
