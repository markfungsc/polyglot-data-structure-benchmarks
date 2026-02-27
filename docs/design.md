# Design

Each language exposes the same logical API for each data structure so that benchmarks are comparable.

## Data structures and intended operations

- **Dynamic Array**: push (append), get by index, length/size.
- **Linked List**: push (append), get by index (or iterate), length/size.
- **HashMap**: get by key, put key-value, delete by key, size.
- **Binary Heap**: push, peek (min or max), pop, size.
- **LRU Cache**: get by key, put key-value, bounded capacity (evict least recently used when full).

Per-language signatures and implementation notes will be added as implementations are completed.
