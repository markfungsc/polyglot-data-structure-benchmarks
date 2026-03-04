package com.polyglot;

import java.util.function.Consumer;

public class LinkedList {
    private static class Node {
        int value;
        Node next;

        Node(int value) {
            this.value = value;
            this.next = null;
        }
    }

    private Node head;
    private Node tail;
    private int size;

    public LinkedList() {
        this.head = null;
        this.tail = null;
        this.size = 0;
    }

    public void pushBack(int value) {
        Node newNode = new Node(value);
        if (head == null) {
            head = newNode;
            tail = newNode;
        } else {
            tail.next = newNode;
            tail = newNode;
        }
        size++;
    }

    public int get(int i) {
        if (i < 0 || i >= size) {
            throw new IndexOutOfBoundsException("LinkedList: Index out of bounds: " + i);
        }
        Node current = head;
        for (int j = 0; j < i; j++) {
            current = current.next;
        }
        return current.value;
    }

    public void delete(int i) {
        if (i < 0 || i >= size) {
            throw new IndexOutOfBoundsException("LinkedList: Index out of bounds: " + i);
        }
        if (i == 0) {
            head = head.next;
            if (head == null) {
                tail = null;
            }
        } else {
            Node current = head;
            for (int j = 0; j < i - 1; j++) {
                current = current.next;
            }
            current.next = current.next.next;
            if (current.next == null) {
                tail = current;
            }
        }
        size--;
    }

    public int size() { return size; }

    public void traverse(Consumer<Integer> f) {
        Node current = head;
        while (current != null) {
            f.accept(current.value);
            current = current.next;
        }
    }
}
