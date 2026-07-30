"""5. Rate Limiter (class-level, pluggable algorithms)
RateLimiter is an interface; Fixed Window, Sliding Window Log, and Token Bucket are
interchangeable implementations selected at construction time.
"""
from __future__ import annotations

import time
from abc import ABC, abstractmethod
from collections import deque


class RateLimiter(ABC):
    @abstractmethod
    def allow_request(self, client_id: str, now: float | None = None) -> bool:
        """Return True if the request is allowed, False if it should be rejected."""


class FixedWindowRateLimiter(RateLimiter):
    """Simple and cheap, but bursts can double at window boundaries."""

    def __init__(self, max_requests: int, window_seconds: float):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self._counts: dict[str, tuple[int, int]] = {}  # client_id -> (window_index, count)

    def allow_request(self, client_id: str, now: float | None = None) -> bool:
        now = now if now is not None else time.time()
        window_index = int(now // self.window_seconds)
        stored_window, count = self._counts.get(client_id, (window_index, 0))

        if stored_window != window_index:
            count = 0
            stored_window = window_index

        if count >= self.max_requests:
            self._counts[client_id] = (stored_window, count)
            return False

        self._counts[client_id] = (stored_window, count + 1)
        return True


class SlidingWindowLogRateLimiter(RateLimiter):
    """Exact — no boundary burst — at the cost of O(max_requests) memory per client."""

    def __init__(self, max_requests: int, window_seconds: float):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self._logs: dict[str, deque[float]] = {}

    def allow_request(self, client_id: str, now: float | None = None) -> bool:
        now = now if now is not None else time.time()
        log = self._logs.setdefault(client_id, deque())

        cutoff = now - self.window_seconds
        while log and log[0] <= cutoff:
            log.popleft()

        if len(log) >= self.max_requests:
            return False

        log.append(now)
        return True


class TokenBucketRateLimiter(RateLimiter):
    """Allows controlled bursts up to bucket capacity; smooths sustained rate via refill."""

    def __init__(self, capacity: int, refill_rate_per_second: float):
        self.capacity = capacity
        self.refill_rate = refill_rate_per_second
        self._buckets: dict[str, tuple[float, float]] = {}  # client_id -> (tokens, last_refill)

    def allow_request(self, client_id: str, now: float | None = None) -> bool:
        now = now if now is not None else time.time()
        tokens, last_refill = self._buckets.get(client_id, (float(self.capacity), now))

        elapsed = max(0.0, now - last_refill)
        tokens = min(self.capacity, tokens + elapsed * self.refill_rate)

        if tokens < 1:
            self._buckets[client_id] = (tokens, now)
            return False

        self._buckets[client_id] = (tokens - 1, now)
        return True


if __name__ == "__main__":
    fw = FixedWindowRateLimiter(max_requests=2, window_seconds=10)
    assert fw.allow_request("u1", now=0) is True
    assert fw.allow_request("u1", now=1) is True
    assert fw.allow_request("u1", now=2) is False  # window exhausted
    assert fw.allow_request("u1", now=11) is True  # new window

    sw = SlidingWindowLogRateLimiter(max_requests=2, window_seconds=10)
    assert sw.allow_request("u1", now=0) is True
    assert sw.allow_request("u1", now=5) is True
    assert sw.allow_request("u1", now=9) is False  # both prior requests still in window
    assert sw.allow_request("u1", now=11) is True  # now=0 request has aged out

    tb = TokenBucketRateLimiter(capacity=2, refill_rate_per_second=1)
    assert tb.allow_request("u1", now=0) is True
    assert tb.allow_request("u1", now=0) is True
    assert tb.allow_request("u1", now=0) is False  # bucket empty
    assert tb.allow_request("u1", now=1) is True  # refilled one token after 1s

    # Per-client isolation: exhausting u1 doesn't affect u2.
    assert fw.allow_request("u2", now=2) is True

    print("All tests passed.")
