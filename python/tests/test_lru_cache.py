import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from lru_cache import LRUCache


def test_new_accepts_capacity():
    cache = LRUCache(4)
    assert cache is not None


def test_get_returns_none_stub():
    cache = LRUCache(2)
    assert cache.get(1) is None


def test_put_does_not_raise_stub():
    cache = LRUCache(2)
    cache.put(1, 10)
