import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from src.linked_list import LinkedList


def test_new_is_empty():
    lst = LinkedList()
    assert lst.size() == 0
    assert lst.get(0) is None


def test_push_back_and_get():
    lst = LinkedList()
    lst.push_back(1)
    lst.push_back(2)
    lst.push_back(3)
    assert lst.size() == 3
    assert lst.get(0) == 1
    assert lst.get(1) == 2
    assert lst.get(2) == 3
    assert lst.get(3) is None


def test_delete_head():
    lst = LinkedList()
    lst.push_back(1)
    lst.push_back(2)
    assert lst.delete(0) is True
    assert lst.size() == 1
    assert lst.get(0) == 2


def test_traverse_collects():
    lst = LinkedList()
    lst.push_back(1)
    lst.push_back(2)
    lst.push_back(3)
    v = []
    lst.traverse(v.append)
    assert v == [1, 2, 3]
