use std::{
    cmp::{Eq, Ordering},
    collections::BinaryHeap,
};

// minimum priority queue

pub struct MinPriority<T> {
    heap: BinaryHeap<MinPriorityQueueItem<T>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinPriorityQueueItem<T> {
    item: T,
    priority: usize,
}

impl<T: Eq> Default for MinPriority<T> {
    fn default() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
}

impl<T: Eq> MinPriority<T> {
    pub fn push(&mut self, item: T, priority: usize) {
        self.heap.push(MinPriorityQueueItem { item, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            Some(t) => Some(t.item),
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Eq> Ord for MinPriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // min queue, lower priorities are grabbed first
        other.priority.cmp(&self.priority)
    }
}

impl<T: Eq> PartialOrd for MinPriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.priority.cmp(&self.priority))
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::MinPriority;

    #[test]
    fn min_priority_queue() {
        let mut pq: MinPriority<&str> = MinPriority::default();
        pq.push("apple", 1);
        pq.push("banana", 0);
        pq.push("melon", 100);

        assert_eq!(pq.pop().unwrap(), "banana");
        assert_eq!(pq.pop().unwrap(), "apple");
        assert_eq!(pq.pop().unwrap(), "melon");
    }
}
