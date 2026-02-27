import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "src"))
from dynamic_array import DynamicArray

def test_exists():
    assert DynamicArray is not None
    assert DynamicArray().length() == 0
