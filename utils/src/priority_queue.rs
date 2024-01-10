//! PriorityQueue
//! PrioritiyQueue for key, value pairs based on [BinaryHeap].
//! [BinaryHeap] levearages [Ord] to manage min vs max heap.
use std::collections::BinaryHeap;

/// Key, Value wrapper for the PriorityQueue
#[derive(Debug)]
pub struct Entry<K: Ord, V> {
    min: bool,
    key: K,
    value: V,
}

impl<K: Ord, V> Entry<K, V> {
    pub fn new(min: bool, key: K, value: V) -> Self {
        Self { min, key, value }
    }
}

impl<K: Ord, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Entry<K, V> {}

impl<K: Ord, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Entry<K, V> {
    /// To implement a max heap, do self.key.cmp(other.key)
    /// To implement a min heap, reverse it.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.min {
            true => other.key.cmp(&self.key),
            false => self.key.cmp(&other.key),
        }
    }
}

#[derive(Debug)]
pub struct PriorityQueue<K: Ord, V> {
    min: bool,
    heap: BinaryHeap<Entry<K, V>>,
}

impl<K: Ord, V> Default for PriorityQueue<K, V> {
    fn default() -> Self {
        Self {
            min: true,
            heap: BinaryHeap::new(),
        }
    }
}

impl<K: Ord, V> PriorityQueue<K, V> {
    pub fn new(min: bool) -> Self {
        Self {
            min,
            heap: BinaryHeap::<Entry<K, V>>::new(),
        }
    }
    pub fn with_capacity(min: bool, capacity: usize) -> Self {
        Self {
            min,
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Entry::<K, V>::new(self.min, key, value))
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|entry| (entry.key, entry.value))
    }

    pub fn keys(&self) -> Vec<&K> {
        self.heap.iter().map(|entry| &entry.key).collect()
    }

    pub fn values(&self) -> Vec<&V> {
        self.heap.iter().map(|entry| &entry.value).collect()
    }
    pub fn entries(&self) -> Vec<(&K, &V)> {
        self.heap
            .iter()
            .map(|entry| (&entry.key, &entry.value))
            .collect()
    }
}

/// Treat PriorityQueue like a BinaryHeap
impl<K: Ord, V> std::ops::Deref for PriorityQueue<K, V> {
    type Target = BinaryHeap<Entry<K, V>>;
    fn deref(&self) -> &Self::Target {
        &self.heap
    }
}
impl<K: Ord, V> std::ops::DerefMut for PriorityQueue<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.heap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let min = true;
        let mut min_heap = PriorityQueue::<u32, u32>::new(min);

        min_heap.push(2, 0);
        min_heap.push(1, 1);
        min_heap.push(1, 3);
        min_heap.push(1, 2);

        assert_eq!(min_heap.keys(), vec![&1u32, &1u32, &1u32, &2u32]);
        assert_eq!(min_heap.values(), vec![&1u32, &2u32, &3u32, &0u32]);
        assert_eq!(
            min_heap.entries(),
            vec![
                (&1u32, &1u32),
                (&1u32, &2u32),
                (&1u32, &3u32),
                (&2u32, &0u32)
            ]
        );

        assert_eq!(min_heap.pop(), Some((1, 1)));
        assert_eq!(min_heap.pop(), Some((1, 3)));
        assert_eq!(min_heap.pop(), Some((1, 2)));
        assert_eq!(min_heap.pop(), Some((2, 0)));

        // By implementing Deref, we can treat the PiorityQueue like a BinaryHeap
        min_heap.clear();
        assert!(min_heap.is_empty());
    }
}
