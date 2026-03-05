pub struct DynamicArray {
    data: Vec<i32>,
}

impl DynamicArray {
    pub fn new(capacity: usize) -> Self {
        DynamicArray {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_has_zero_size() {
        let a = DynamicArray::new(4);
        assert_eq!(a.size(), 0);
        assert_eq!(a.capacity(), 4);
    }

    #[test]
    fn push_and_get() {
        let mut a = DynamicArray::new(2);
        a.push(10);
        a.push(20);
        assert_eq!(a.size(), 2);
        assert_eq!(a.get(0), Some(&10));
        assert_eq!(a.get(1), Some(&20));
        assert_eq!(a.get(2), None);
    }
}
