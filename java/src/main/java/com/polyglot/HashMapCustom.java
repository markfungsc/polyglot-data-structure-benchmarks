package com.polyglot;

import java.util.ArrayList;
import java.util.List;

class Entry<K, V> {
    K key;
    V value;

    Entry(K key, V value) {
        this.key = key;
        this.value = value;
    }
}

public class HashMapCustom<K, V> {
    private static final int DEFAULT_CAPACITY = 16;
    private static final double LOAD_FACTOR = 0.75;

    private List<List<Entry<K, V>>> buckets;
    private int capacity;
    private int size;

    public HashMapCustom(int capacity) {
        this.capacity = Math.max(1, capacity);
        this.buckets = new ArrayList<>(this.capacity);
        for (int i = 0; i < this.capacity; i++) {
            buckets.add(new ArrayList<>());
        }
        this.size = 0;
    }

    private int hash(K key) {
        return Math.abs(key.hashCode()) % capacity;
    }

    public void insert(K key, V value) {
        int idx = hash(key);
        List<Entry<K,V>> bucket = buckets.get(idx);
        for (Entry<K,V> e : bucket) {
            if (e.key.equals(key)) {
                e.value = value;
                return;
            }
        }
        bucket.add(new Entry<>(key, value));
        size++;

        if (size > LOAD_FACTOR * capacity) {
            resize();
        }
    }

    public V get(K key) {
        int idx = hash(key);
        List<Entry<K,V>> bucket = buckets.get(idx);
        for (Entry<K,V> e : bucket) {
            if (e.key.equals(key)) {
                return e.value;
            }
        }
        return null;
    }

    private void resize() {
        int newCapacity = capacity * 2;
        List<List<Entry<K,V>>> newBuckets = new ArrayList<>();
        for (int i = 0; i < newCapacity; i++) newBuckets.add(new ArrayList<Entry<K, V>>());

        for (List<Entry<K,V>> bucket : buckets) {
            for (Entry<K,V> e : bucket) {
                int idx = Math.abs(e.key.hashCode()) % newCapacity;
                newBuckets.get(idx).add(e);
            }
        }

        capacity = newCapacity;
        buckets = newBuckets;
    }
}
