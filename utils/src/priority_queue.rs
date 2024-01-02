//! PriorityQueue
//! Generic MinHeap based PrioritiyQueue
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Wrapper<K: Ord, V> {
    min: bool,
    key: K,
    value: V,
}

impl<K: Ord, V> Wrapper<K, V> {
    pub fn new(min: bool, key: K, value: V) -> Self {
        Self { min, key, value }
    }
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    /// To implement a max heap, use self.key.cmp(other.key)
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
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> Default for PriorityQueue<K, V> {
    fn default() -> Self {
        Self {
            min: false,
            heap: BinaryHeap::new(),
        }
    }
}

impl<K: Ord, V> PriorityQueue<K, V> {
    pub fn new(min: bool) -> Self {
        Self {
            min,
            heap: BinaryHeap::<Wrapper<K, V>>::new(),
        }
    }
    pub fn with_capacity(min: bool, capacity: usize) -> Self {
        Self {
            min,
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        let wrapper = Wrapper::<K, V>::new(self.min, key, value);
        self.heap.push(wrapper);
    }
    pub fn pop(&mut self) -> Option<(K, V)> {
        let Some(wrapper) = self.heap.pop() else {
            return None;
        };
        Some((wrapper.key, wrapper.value))
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
        assert_eq!(min_heap.pop(), Some((1, 1)));
        assert_eq!(min_heap.pop(), Some((1, 3)));
        assert_eq!(min_heap.pop(), Some((1, 2)));
        assert_eq!(min_heap.pop(), Some((2, 0)));
    }
}
