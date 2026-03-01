package com.polyglot;

import java.util.Arrays;

public class DynamicArray {
    private int[] data;
    private int size;

    public DynamicArray(int capacity) {
        this.data = new int[Math.max(1, capacity)];
        this.size = 0;
    }

    public void push(int value) {
        if (size == data.length) {
            resize();
        }
        data[size++] = value;
    }

    public int get(int index) {
        if (index < 0 || index >= size) {
            throw new IndexOutOfBoundsException("DynamicArray: Index out of bounds: " + index);
        }
        return data[index];
    }

    public int size() { return size; }
    public int capacity() { return data.length; }

    private void resize() {
        int newCapacity = data.length * 2;
        data = Arrays.copyOf(data, newCapacity);
    }
}
