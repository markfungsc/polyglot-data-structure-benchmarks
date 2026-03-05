pub struct MinHeap {
    data: Vec<i32>,
}

impl Default for MinHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    /// Pop the minimum value from the heap
    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&i32> {
        self.data.first()
    }

    fn sift_up(&mut self, mut index: usize) {
        let value = self.data[index];
        while index > 0 {
            // get the parent index
            let parent = (index - 1) / 2;

            // if the current value is greater than the parent, break
            if value >= self.data[parent] {
                break;
            }
            // else, move the parent down
            self.data[index] = self.data[parent];
            // update the index to the parent
            index = parent;
        }
        // update the value at the correct index
        self.data[index] = value;
    }

    fn sift_down(&mut self, mut index: usize) {
        let n = self.data.len();
        let value = self.data[index];
        loop {
            // get the left child index
            let left = 2 * index + 1;
            if left >= n {
                // if the left child is out of bounds, break
                break;
            }
            let right = left + 1;
            let smallest = if right < n && self.data[right] < self.data[left] {
                // if the right child is in bounds and is smaller than the left child, update the smallest index
                right
            } else {
                // else, update the smallest index to the left child
                left
            };
            if self.data[smallest] >= value {
                // if the smallest value is greater than the value, break
                break;
            }
            self.data[index] = self.data[smallest]; // else, move the smallest value up
            index = smallest; // update the index to the smallest index
        }
        self.data[index] = value; // update the value at the correct index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let mut h = MinHeap::new();
        assert_eq!(h.size(), 0);
        assert_eq!(h.peek(), None);
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn insert_and_peek() {
        let mut h = MinHeap::new();
        h.insert(3);
        h.insert(1);
        h.insert(2);
        assert_eq!(h.size(), 3);
        assert_eq!(h.peek(), Some(&1));
    }

    #[test]
    fn pop_returns_min() {
        let mut h = MinHeap::new();
        h.insert(3);
        h.insert(1);
        h.insert(2);
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), None);
    }
}
