import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from src.lru_cache import LRUCache


def test_new_accepts_capacity():
    cache = LRUCache(4)
    assert cache is not None
    assert cache.capacity == 4
    assert len(cache.cache) == 0


def test_get():
    cache = LRUCache(2)
    assert cache.get(1) is None
    cache.put(1, 10)
    cache.put(2, 20)
    assert cache.get(1) == 10
    assert cache.get(2) == 20
    assert cache.get(3) is None


def test_put():
    cache = LRUCache(2)
    cache.put(1, 10)
    assert cache.get(1) == 10
    assert cache.get(2) is None
    cache.put(2, 20)
    assert cache.get(1) == 10
    assert cache.get(2) == 20
    assert cache.get(3) is None
    cache.put(3, 30)
    assert cache.get(1) is None
    assert cache.get(2) == 20
    assert cache.get(3) == 30
