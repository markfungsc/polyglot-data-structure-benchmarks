import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from src.heap import MinHeap


def test_new_is_empty():
    h = MinHeap()
    assert h.size() == 0
    assert h.peek() is None
    assert h.pop() is None


def test_insert_and_peek():
    h = MinHeap()
    h.insert(3)
    h.insert(1)
    h.insert(2)
    assert h.size() == 3
    assert h.peek() == 1


def test_pop_returns_min():
    h = MinHeap()
    h.insert(3)
    h.insert(1)
    h.insert(2)
    assert h.pop() == 1
    assert h.pop() == 2
    assert h.pop() == 3
    assert h.pop() is None
