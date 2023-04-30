use std::{
    cmp::{Eq, Ordering},
    collections::BinaryHeap,
};

pub struct Priority<T> {
    heap: BinaryHeap<PriorityQueueItem<T>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PriorityQueueItem<T> {
    item: T,
    priority: usize,
}

impl<T: Eq> Priority<T> {
    pub fn new() -> Priority<T> {
        Priority {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T, priority: usize) {
        self.heap.push(PriorityQueueItem { item, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            Some(t) => Some(t.item),
            None => None,
        }
    }
}

impl<T: Eq> Default for Priority<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Eq> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // min queue, lower priorities are grabbed first
        other.priority.cmp(&self.priority)
    }
}
impl<T: Eq> PartialOrd for PriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Priority;

    #[test]
    fn priority_queue() {
        let mut pq: Priority<&str> = Priority::new();
        pq.push("apple", 1);
        pq.push("banana", 0);
        pq.push("melon", 100);

        assert_eq!(pq.pop().unwrap(), "banana");
        assert_eq!(pq.pop().unwrap(), "apple");
        assert_eq!(pq.pop().unwrap(), "melon");
    }
}
