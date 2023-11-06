use std::{
    cmp::{Eq, Ordering},
    collections::BinaryHeap,
};

// maximum priority queue

pub struct MinPriority<T> {
    heap: BinaryHeap<MinPriorityQueueItem<T>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinPriorityQueueItem<T> {
    item: T,
    priority: usize,
}

impl<T: Eq> MinPriority<T> {
    pub fn new() -> MinPriority<T> {
        MinPriority {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T, priority: usize) {
        self.heap.push(MinPriorityQueueItem { item, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            Some(t) => Some(t.item),
            None => None,
        }
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

// maximum priority queue

pub struct MaxPriority<T> {
    heap: BinaryHeap<MaxPriorityQueueItem<T>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MaxPriorityQueueItem<T> {
    item: T,
    priority: usize,
}

impl<T: Eq> MaxPriority<T> {
    pub fn new() -> MaxPriority<T> {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T, priority: usize) {
        self.heap.push(MaxPriorityQueueItem { item, priority });
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.heap.pop() {
            Some(t) => Some(t.item),
            None => None,
        }
    }
}

impl<T: Eq> Ord for MaxPriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // max queue, higher priorities are grabbed first
        self.priority.cmp(&other.priority)
    }
}
impl<T: Eq> PartialOrd for MaxPriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::{MaxPriority, MinPriority};

    #[test]
    fn min_priority_queue() {
        let mut pq: MinPriority<&str> = MinPriority::new();
        pq.push("apple", 1);
        pq.push("banana", 0);
        pq.push("melon", 100);

        assert_eq!(pq.pop().unwrap(), "banana");
        assert_eq!(pq.pop().unwrap(), "apple");
        assert_eq!(pq.pop().unwrap(), "melon");
    }

    #[test]
    fn max_priority_queue() {
        let mut pq: MaxPriority<&str> = MaxPriority::new();
        pq.push("apple", 1);
        pq.push("banana", 0);
        pq.push("melon", 100);

        assert_eq!(pq.pop().unwrap(), "melon");
        assert_eq!(pq.pop().unwrap(), "apple");
        assert_eq!(pq.pop().unwrap(), "banana");
    }
}
