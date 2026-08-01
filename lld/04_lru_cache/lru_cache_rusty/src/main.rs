//! 4. LRU Cache (designed as a class, not just an algorithm)
//! Composition of a hash map (O(1) lookup) and a doubly linked list (O(1) reorder/evict),
//! with the eviction policy pulled behind a Strategy interface for extensibility.
//!
//! solution.py's linked list freely aliases mutable `prev`/`next` references between
//! nodes — exactly what Rust's ownership model forbids by default (only one `&mut`
//! reference to a value at a time). Production Rust LRU implementations (e.g. the `lru`
//! crate) reach for `unsafe` raw pointers to get this back. The safe-Rust alternative
//! used here is `Rc<RefCell<Node>>` for forward links plus `Weak<RefCell<Node>>` for
//! backward links: `Rc` allows multiple owners, `RefCell` moves the "only one mutable
//! borrow at a time" rule from compile time to run time, and `Weak` avoids the
//! reference-cycle memory leak that plain `Rc` for both directions would create (a
//! `prev`/`next` cycle of strong references would never hit a refcount of zero). The
//! trade-off: every access goes through a `.borrow()`/`.borrow_mut()` runtime check
//! instead of being free at compile time — the classic safety-vs-zero-cost tension this
//! specific data structure surfaces in Rust.
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    prev: Option<Weak<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

type NodeRef<K, V> = Rc<RefCell<Node<K, V>>>;

impl<K, V> Node<K, V> {
    fn sentinel() -> NodeRef<K, V> {
        Rc::new(RefCell::new(Node {
            key: None,
            value: None,
            prev: None,
            next: None,
        }))
    }
}

/// Sentinel-headed list: head.next is most-recently-used, tail.prev is least.
struct DoublyLinkedList<K, V> {
    head: NodeRef<K, V>,
    tail: NodeRef<K, V>,
}

impl<K, V> DoublyLinkedList<K, V> {
    fn new() -> Self {
        let head = Node::sentinel();
        let tail = Node::sentinel();
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        DoublyLinkedList { head, tail }
    }

    // &self, not &mut self: mutation happens through RefCell's interior mutability, so
    // the eviction policy (below) only ever needs a shared reference to the list —
    // sidestepping the exact double-mutable-borrow problem that forced the elevator
    // and vending machine states into an enum instead of a trait object.
    fn remove(&self, node: &NodeRef<K, V>) {
        let prev = node.borrow().prev.clone().and_then(|w| w.upgrade());
        let next = node.borrow().next.clone();
        if let (Some(p), Some(n)) = (prev, next) {
            p.borrow_mut().next = Some(n.clone());
            n.borrow_mut().prev = Some(Rc::downgrade(&p));
        }
    }

    fn add_front(&self, node: &NodeRef<K, V>) {
        let first = self.head.borrow().next.clone().unwrap();
        node.borrow_mut().next = Some(first.clone());
        node.borrow_mut().prev = Some(Rc::downgrade(&self.head));
        first.borrow_mut().prev = Some(Rc::downgrade(node));
        self.head.borrow_mut().next = Some(node.clone());
    }

    fn pop_back(&self) -> Option<NodeRef<K, V>> {
        let last = self.tail.borrow().prev.clone().and_then(|w| w.upgrade())?;
        if Rc::ptr_eq(&last, &self.head) {
            return None; // empty: tail.prev points straight back to head
        }
        self.remove(&last);
        Some(last)
    }
}

/// Strategy seam: LRU today, LFU or TTL-based tomorrow, without touching Cache.
trait EvictionPolicy<K, V> {
    fn on_access(&self, order: &DoublyLinkedList<K, V>, node: &NodeRef<K, V>);
    fn on_insert(&self, order: &DoublyLinkedList<K, V>, node: &NodeRef<K, V>);
    fn evict(&self, order: &DoublyLinkedList<K, V>) -> Option<K>;
}

struct LruEvictionPolicy;

impl<K: Clone, V> EvictionPolicy<K, V> for LruEvictionPolicy {
    fn on_access(&self, order: &DoublyLinkedList<K, V>, node: &NodeRef<K, V>) {
        order.remove(node);
        order.add_front(node);
    }

    fn on_insert(&self, order: &DoublyLinkedList<K, V>, node: &NodeRef<K, V>) {
        order.add_front(node);
    }

    fn evict(&self, order: &DoublyLinkedList<K, V>) -> Option<K> {
        order.pop_back().map(|n| n.borrow().key.clone().unwrap())
    }
}

struct Cache<K: Eq + Hash + Clone, V: Clone> {
    capacity: usize,
    policy: Box<dyn EvictionPolicy<K, V>>,
    map: HashMap<K, NodeRef<K, V>>,
    order: DoublyLinkedList<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    fn new(capacity: usize) -> Self {
        Self::with_policy(capacity, Box::new(LruEvictionPolicy))
    }

    fn with_policy(capacity: usize, policy: Box<dyn EvictionPolicy<K, V>>) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        Cache {
            capacity,
            policy,
            map: HashMap::new(),
            order: DoublyLinkedList::new(),
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        let node = self.map.get(key)?;
        self.policy.on_access(&self.order, node);
        node.borrow().value.clone()
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(existing) = self.map.get(&key) {
            existing.borrow_mut().value = Some(value);
            self.policy.on_access(&self.order, existing);
            return;
        }

        if self.map.len() >= self.capacity {
            if let Some(evicted_key) = self.policy.evict(&self.order) {
                self.map.remove(&evicted_key);
            }
        }

        let node = Rc::new(RefCell::new(Node {
            key: Some(key.clone()),
            value: Some(value),
            prev: None,
            next: None,
        }));
        self.policy.on_insert(&self.order, &node);
        self.map.insert(key, node);
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

fn main() {
    let mut cache: Cache<String, i32> = Cache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    println!("{:?}", cache.get(&"a".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_python_reference_behavior() {
        let mut cache: Cache<String, i32> = Cache::new(2);
        cache.put("a".to_string(), 1);
        cache.put("b".to_string(), 2);
        assert_eq!(cache.get(&"a".to_string()), Some(1)); // "a" now most-recently-used

        cache.put("c".to_string(), 3); // evicts "b" (least-recently-used), not "a"
        assert_eq!(cache.get(&"b".to_string()), None);
        assert_eq!(cache.get(&"a".to_string()), Some(1));
        assert_eq!(cache.get(&"c".to_string()), Some(3));

        cache.put("a".to_string(), 100); // update existing key refreshes recency, doesn't evict
        assert_eq!(cache.get(&"a".to_string()), Some(100));
        assert_eq!(cache.len(), 2);

        let mut small: Cache<i32, String> = Cache::new(1);
        small.put(1, "x".to_string());
        small.put(2, "y".to_string()); // evicts 1
        assert_eq!(small.get(&1), None);
        assert_eq!(small.get(&2), Some("y".to_string()));
    }
}
