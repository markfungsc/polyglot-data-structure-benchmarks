import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from src.hashmap import HashMap


def test_new_is_empty():
    m = HashMap(8)
    assert m.get(1) is None


def test_insert_and_get():
    m = HashMap(8)
    m.insert(1, 10)
    m.insert(2, 20)
    assert m.get(1) == 10
    assert m.get(2) == 20
    assert m.get(3) is None


def test_insert_overwrites():
    m = HashMap(8)
    m.insert(1, 10)
    m.insert(1, 20)
    assert m.get(1) == 20
