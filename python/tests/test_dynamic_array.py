import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from src.dynamic_array import DynamicArray


def test_new_has_zero_size():
    a = DynamicArray(4)
    assert len(a) == 0
    assert a.capacity == 4


def test_push_and_get():
    a = DynamicArray(2)
    a.push(10)
    a.push(20)
    assert len(a) == 2
    assert a.get(0) == 10
    assert a.get(1) == 20


def test_get_out_of_bounds_raises():
    a = DynamicArray(2)
    a.push(10)
    try:
        a.get(1)
        assert False, "expected IndexError"
    except IndexError:
        pass
