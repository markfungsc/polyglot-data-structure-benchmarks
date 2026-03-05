package com.polyglot;

import org.junit.jupiter.api.Test;
import java.util.ArrayList;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;

class LinkedListTest {
    @Test
    void newIsEmpty() {
        LinkedList list = new LinkedList();
        assertEquals(0, list.size());
        assertThrows(IndexOutOfBoundsException.class, () -> list.get(0));
    }

    @Test
    void pushBackAndGet() {
        LinkedList list = new LinkedList();
        list.pushBack(1);
        list.pushBack(2);
        list.pushBack(3);
        assertEquals(3, list.size());
        assertEquals(1, list.get(0));
        assertEquals(2, list.get(1));
        assertEquals(3, list.get(2));
        assertThrows(IndexOutOfBoundsException.class, () -> list.get(3));
    }

    @Test
    void deleteHead() {
        LinkedList list = new LinkedList();
        list.pushBack(1);
        list.pushBack(2);
        list.delete(0);
        assertEquals(1, list.size());
        assertEquals(2, list.get(0));
    }

    @Test
    void traverseCollects() {
        LinkedList list = new LinkedList();
        list.pushBack(1);
        list.pushBack(2);
        list.pushBack(3);
        List<Integer> v = new ArrayList<>();
        list.traverse(v::add);
        assertEquals(List.of(1, 2, 3), v);
    }
}
