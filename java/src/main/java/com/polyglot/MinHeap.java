package com.polyglot;

public class MinHeap {
    private int[] data;
    private int size;

    public MinHeap(int capacity) {
        this.data = new int[capacity];
        this.size = 0;
    }

    private void ensureCapacity() {
        if (size >= data.length) {
            int[] newData = new int[data.length * 2];
            System.arraycopy(data, 0, newData, 0, data.length);
            data = newData;
        }
    }

    public void insert(int value) {
        ensureCapacity();
        data[size++] = value;
        siftUp(size - 1);
    }

    public int pop() {
        if (size == 0) {
            throw new IllegalStateException("MinHeap is empty");
        }
        int min = data[0];
        data[0] = data[--size];
        if (size > 0) {
            siftDown(0);
        }
        return min;
    }

    public int peek() {
        if (size == 0) {
            throw new IllegalStateException("MinHeap is empty");
        }
        return data[0];
    }

    public int size() { return size; }

    private void siftUp(int index) {
        // get the value at the index
        int value = data[index];
        while (index > 0) {
            int parent = (index - 1) / 2;
            // if the current value is less than the parent, swap them
            if (value >= data[parent]) break; // if the current value is greater than the parent, break
            // else, swap the current value with the parent
            data[index] = data[parent];
            index = parent;
        }
        // update the value at the index
        data[index] = value;
    }

    private void siftDown(int index) {
        int n = size;
        // get the value at the index
        int value = data[index];
        while (true) {
            int left = 2 * index + 1;
            if (left >= n) break; // if the left child is out of bounds, break
            int right = left + 1;
            int smallest = left;
            if (right < n && data[right] < data[left]) smallest = right;
            if (data[smallest] >= value) break; // if the current value is less than the smallest value, break
            data[index] = data[smallest];
            index = smallest;
        }
        data[index] = value; // update the value at the index
    }
}
