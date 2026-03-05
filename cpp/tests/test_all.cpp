// Simple unit tests for all data structures (no external test framework).
#include "../src/dynamic_array.hpp"
#include "../src/linked_list.hpp"
#include "../src/heap.hpp"
#include "../src/hashmap.hpp"
#include "../src/lru_cache.hpp"
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <functional>
#include <stdexcept>
#include <vector>

static int tests_run = 0;
static int tests_failed = 0;
static const char* current_test = "";

#define ASSERT(cond) do { \
    ++tests_run; \
    if (!(cond)) { \
        fprintf(stderr, "%s:%d: [%s] ASSERT failed: %s\n", __FILE__, __LINE__, current_test, #cond); \
        ++tests_failed; \
        return; \
    } \
} while (0)
#define ASSERT_THROW(expr, exc) do { \
    ++tests_run; \
    try { (expr); \
        fprintf(stderr, "%s:%d: [%s] ASSERT_THROW failed: expected exception, none thrown\n", __FILE__, __LINE__, current_test); \
        ++tests_failed; \
        return; \
    } catch (const exc&) {} \
} while (0)

void test_dynamic_array() {
    current_test = "test_dynamic_array";
    dynamic_array::DynamicArray<std::int32_t> a;
    ASSERT(a.size() == 0);
    ASSERT(a.capacity() >= 4);

    a.push(10);
    a.push(20);
    ASSERT(a.size() == 2);
    ASSERT(a.get(0) == 10);
    ASSERT(a.get(1) == 20);

    ASSERT_THROW((void)a.get(2), std::out_of_range);
}

void test_linked_list() {
    current_test = "test_linked_list";
    linked_list::LinkedList list;
    ASSERT(list.size() == 0);
    ASSERT_THROW((void)list.get(0), std::out_of_range);

    list.pushBack(1);
    list.pushBack(2);
    list.pushBack(3);
    ASSERT(list.size() == 3);
    ASSERT(list.get(0) == 1);
    ASSERT(list.get(1) == 2);
    ASSERT(list.get(2) == 3);
    ASSERT_THROW((void)list.get(3), std::out_of_range);

    list.deleteNode(0);
    ASSERT(list.size() == 2);
    ASSERT(list.get(0) == 2);

    std::vector<int> v;
    list.traverse([&v](int x) { v.push_back(x); });
    ASSERT(v.size() == 2 && v[0] == 2 && v[1] == 3);
}

void test_heap() {
    current_test = "test_heap";
    heap::MinHeap<int> h(16);
    ASSERT(h.size() == 0);
    ASSERT(h.empty());
    ASSERT_THROW((void)h.peek(), std::out_of_range);
    ASSERT_THROW((void)h.pop(), std::out_of_range);

    h.insert(3);
    h.insert(1);
    h.insert(2);
    ASSERT(h.size() == 3);
    ASSERT(h.peek() == 1);

    ASSERT(h.pop() == 1);
    ASSERT(h.pop() == 2);
    ASSERT(h.pop() == 3);
    ASSERT(h.empty());
    ASSERT_THROW((void)h.pop(), std::out_of_range);
}

void test_hashmap() {
    current_test = "test_hashmap";
    hashmap::HashMap m(8);
    ASSERT(m.get(1) == 0);  // not found returns 0 in this impl

    m.insert(1, 10);
    m.insert(2, 20);
    ASSERT(m.get(1) == 10);
    ASSERT(m.get(2) == 20);
    ASSERT(m.get(3) == 0);

    m.insert(1, 20);
    ASSERT(m.get(1) == 20);
}

void test_lru_cache() {
    current_test = "test_lru_cache";
    lru_cache::LRUCache cache(4);
    ASSERT(cache.get(1) == -1);  // stub returns -1
    cache.put(1, 10);  // no throw
}

int main() {
    test_dynamic_array();
    test_linked_list();
    test_heap();
    test_hashmap();
    test_lru_cache();

    if (tests_failed != 0) {
        fprintf(stderr, "FAILED: %d / %d tests\n", tests_failed, tests_run);
        return 1;
    }
    printf("OK: %d tests passed\n", tests_run);
    return 0;
}
