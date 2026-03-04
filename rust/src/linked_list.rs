type Link = Option<Box<Node>>;

struct Node {
    value: i32,
    next: Link,
}

pub struct LinkedList {
    head: Link,
    tail: *mut Node, // raw pointer to the last node
    size: usize,
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
            // boxed_node goes out of scope here, deallocating node
        }
        // tail pointer is raw, doesn't need drop
    }
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None, tail: std::ptr::null_mut(), size: 0 }
    }

    // Push a new node to the back of the list
    pub fn push_back(&mut self, value: i32) {
        let mut new_node = Box::new(Node { value, next: None });
        
        let raw_tail: *mut Node = &mut *new_node;

        if self.head.is_none() {
            // if the list is empty, set the head and tail to the new node
            self.head = Some(new_node);
            self.tail = raw_tail;
        } else {
            // if the list is not empty, set the next of the tail to the new node
            // update tail raw pointer to the new node
            unsafe {
                (*self.tail).next = Some(new_node);
            }
            self.tail = raw_tail;
        }

        self.size += 1;
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.size {
            return None;
        }
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current.unwrap().next.as_ref();
        }
        current.map(|node| node.value)
    }

    pub fn delete(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        if index == 0 {
            // take ownership of the head node and set the head to the next node
            let mut old_head = self.head.take().unwrap();
            // points the head to the next node
            self.head = old_head.next.take();

            // reset the tail to null if the list is empty
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            self.size -= 1;
            return true;
        }

        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            current = current.unwrap().next.as_mut();
        }

        if let Some(node) = current {
            let mut removed = node.next.take().unwrap();
            node.next = removed.next.take();

            // deleted the last node, so reset the tail to null
            if node.next.is_none() {
                self.tail = std::ptr::null_mut();
            }
        }

        self.size -= 1;
        true
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            f(node.value);
            current = node.next.as_ref();
        }
    }
}
