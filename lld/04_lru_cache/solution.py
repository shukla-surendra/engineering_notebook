"""4. LRU Cache (designed as a class, not just an algorithm)
Composition of a hash map (O(1) lookup) and a doubly linked list (O(1) reorder/evict),
with the eviction policy pulled behind a Strategy interface for extensibility.
"""
from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, Optional, TypeVar

K = TypeVar("K")
V = TypeVar("V")


class Node(Generic[K, V]):
    __slots__ = ("key", "value", "prev", "next")

    def __init__(self, key: K, value: V):
        self.key = key
        self.value = value
        self.prev: Optional["Node[K, V]"] = None
        self.next: Optional["Node[K, V]"] = None


class DoublyLinkedList(Generic[K, V]):
    """Sentinel-headed list: head.next is most-recently-used, tail.prev is least."""

    def __init__(self):
        self.head: Node[K, V] = Node(None, None)  # type: ignore[arg-type]
        self.tail: Node[K, V] = Node(None, None)  # type: ignore[arg-type]
        self.head.next = self.tail
        self.tail.prev = self.head

    def remove(self, node: Node[K, V]) -> None:
        node.prev.next = node.next
        node.next.prev = node.prev

    def add_front(self, node: Node[K, V]) -> None:
        node.next = self.head.next
        node.prev = self.head
        self.head.next.prev = node
        self.head.next = node

    def pop_back(self) -> Optional[Node[K, V]]:
        if self.tail.prev is self.head:
            return None
        lru = self.tail.prev
        self.remove(lru)
        return lru


class EvictionPolicy(ABC, Generic[K, V]):
    """Strategy seam: LRU today, LFU or TTL-based tomorrow, without touching Cache."""

    @abstractmethod
    def on_access(self, cache: "Cache[K, V]", node: Node[K, V]) -> None:
        ...

    @abstractmethod
    def on_insert(self, cache: "Cache[K, V]", node: Node[K, V]) -> None:
        ...

    @abstractmethod
    def evict(self, cache: "Cache[K, V]") -> Optional[K]:
        ...


class LRUEvictionPolicy(EvictionPolicy[K, V]):
    def on_access(self, cache: "Cache[K, V]", node: Node[K, V]) -> None:
        cache.order.remove(node)
        cache.order.add_front(node)

    def on_insert(self, cache: "Cache[K, V]", node: Node[K, V]) -> None:
        cache.order.add_front(node)

    def evict(self, cache: "Cache[K, V]") -> Optional[K]:
        lru = cache.order.pop_back()
        return lru.key if lru else None


class Cache(Generic[K, V]):
    def __init__(self, capacity: int, policy: Optional[EvictionPolicy[K, V]] = None):
        if capacity <= 0:
            raise ValueError("capacity must be positive")
        self.capacity = capacity
        self.policy = policy or LRUEvictionPolicy()
        self._map: dict[K, Node[K, V]] = {}
        self.order: DoublyLinkedList[K, V] = DoublyLinkedList()

    def get(self, key: K) -> Optional[V]:
        node = self._map.get(key)
        if node is None:
            return None
        self.policy.on_access(self, node)
        return node.value

    def put(self, key: K, value: V) -> None:
        existing = self._map.get(key)
        if existing is not None:
            existing.value = value
            self.policy.on_access(self, existing)
            return

        if len(self._map) >= self.capacity:
            evicted_key = self.policy.evict(self)
            if evicted_key is not None:
                del self._map[evicted_key]

        node = Node(key, value)
        self._map[key] = node
        self.policy.on_insert(self, node)

    def __len__(self) -> int:
        return len(self._map)


if __name__ == "__main__":
    cache: Cache[str, int] = Cache(capacity=2)
    cache.put("a", 1)
    cache.put("b", 2)
    assert cache.get("a") == 1  # "a" now most-recently-used

    cache.put("c", 3)  # evicts "b" (least-recently-used), not "a"
    assert cache.get("b") is None
    assert cache.get("a") == 1
    assert cache.get("c") == 3

    cache.put("a", 100)  # update existing key refreshes recency, doesn't evict
    assert cache.get("a") == 100
    assert len(cache) == 2

    small: Cache[int, str] = Cache(capacity=1)
    small.put(1, "x")
    small.put(2, "y")  # evicts 1
    assert small.get(1) is None
    assert small.get(2) == "y"

    print("All tests passed.")
