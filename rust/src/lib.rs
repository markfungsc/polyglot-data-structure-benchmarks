pub mod dynamic_array;
pub mod linked_list;
pub mod hashmap;
pub mod heap;
pub mod lru_cache;

#[cfg(test)]
mod tests {
    use crate::dynamic_array::DynamicArray;
    #[test]
    fn stub() {
        let a = DynamicArray::new();
        assert_eq!(a.length(), 0);
    }
}
