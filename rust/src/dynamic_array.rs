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