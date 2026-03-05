pub mod bench_util;
pub mod dynamic_array;
pub mod hashmap;
pub mod heap;
pub mod linked_list;
pub mod lru_cache;

#[cfg(test)]
mod tests {
    use crate::dynamic_array::DynamicArray;
    #[test]
    fn stub() {
        let a = DynamicArray::new(1);
        assert_eq!(a.size(), 0);
    }
}
