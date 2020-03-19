/*!

A min-max heap, implemented as a way to learn some
Rust. Heap data must be `Copy`, `Clone`, `PartialOrd`.

The implementation is based on a paper by M.D Atkinson,
J.-R. Sack and others, "[Min-Max Heaps And Generalized
Priority
Queues](http://akira.ruc.dk/~keld/teaching/algoritmedesign_f03/Artikler/02/Atkinson86.pdf)"
and on the relevant
[Wikipedia](https://en.wikipedia.org/wiki/Min-max_heap)
article.

This heap can be used as a DEPQ (Double Ended Priority
Queue) implementation.

*/

/// Min-max heap struct.
#[derive(Debug)]
pub struct MinMaxHeap<T> {
    data: Vec<T>,
}

fn log2(mut value: usize) -> Option<usize> {
    match value {
        0 => None,
        _ => {
            let mut res = 0 as usize;
            while value != 0 {
                value >>= 1;
                res += 1;
            }
            Some(res)
        }
    }
}

fn parent(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child(index: usize) -> usize {
    2 * index + 1
}

fn is_on_min_level(index: usize) -> bool {
    (log2(index + 1).unwrap_or_else(|| {
        println!("can't calc log2 of 0");
        0
    }) % 2) == 1
}

impl<T: std::clone::Clone + PartialOrd + Copy> MinMaxHeap<T> {

    /// Make a new heap.
    pub fn new() -> Self {
        MinMaxHeap {
            data: vec![],
        }
    }

    /// Make a `Vec` of elements into a heap.
    pub fn from(v: &Vec<T>) -> Self {
        let mut heap: MinMaxHeap<T> = MinMaxHeap::new();

        // v.map(|val| {
        //     heap.push(val)
        // });

        for val in v.iter() {
            heap.push(*val);
        }

        heap
    }

    /// Make a new heap with initial capacity `len`.
    pub fn with_capacity(len: usize) -> Self {
        MinMaxHeap {
            data: Vec::with_capacity(len)
        }
    }

    /// Test whether the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Number of elements in the heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Insert an element into the heap.
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.len() - 1);
    }

    /// Show a minimum element from the heap.
    pub fn peek_min(&self) -> Option<T> {
        if self.is_empty() { return None }
        return Some(self.data[0].clone())
    }

    /// Show a maximum element from the heap.
    pub fn peek_max(&self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => Some(self.data[0].clone()),
            2 => Some(self.data[1].clone()),
            _ => {
                let max = if self.data[1] > self.data[2] {
                    self.data[1].clone()
                } else {
                    self.data[2].clone()
                };
                Some(max)
            }
        }
    }

    /// Extract a minimum element from the heap.
    pub fn pop_min(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            _ => {
                let min = self.data[0].clone();
                self.delete_element(0);
                Some(min)
            }
        }
    }

    /// Extract a maximum element from the heap.
    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => {
                let max = self.data[0].clone();
                self.delete_element(0);
                Some(max)
            }
            2 => {
                let max = self.data[1].clone();
                self.delete_element(1);
                Some(max)
            }
            _ => {
                let max_ind = if self.data[1] > self.data[2] {
                    1
                } else {
                    2
                };
                let max = self.data[max_ind].clone();
                self.delete_element(max_ind);
                Some(max)
            }
        }
    }
    
    /// Empty the heap.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    fn do_trickle_down(&mut self, index: usize, max_level: bool) {
        if index >= self.len() { return; }
        let mut min_node = index;
        let left = left_child(index);
        if left < self.len() && ((self.data[left] < self.data[min_node]) ^ max_level) { min_node = left; }
        if left + 1 < self.len() && ((self.data[left + 1] < self.data[min_node]) ^ max_level) { min_node = left + 1; }
        let left_gchild = left_child(left);
        let mut i = 0;
        while (i < 4) && (left_gchild + i < self.len()) {
            if (self.data[left_gchild + i] < self.data[min_node]) ^ max_level {
                min_node = left_gchild + i;
            }
            i += 1;
        }
        if index == min_node { return; }
        self.data.swap(index, min_node);
        if min_node - left > 1 {
            if (self.data[parent(min_node)] < self.data[min_node]) ^ max_level {
                self.data.swap(parent(min_node), min_node);
            }
            self.do_trickle_down(min_node, max_level);
        }
    }


    fn trickle_down(&mut self, index: usize) {
            if is_on_min_level(index) {
                self.do_trickle_down(index, false);
            } else {
                self.do_trickle_down(index, true);
            }
    }

    fn do_bubble_up(&mut self, index: usize, max_level: bool) {
        if index == 0 { return; }
        let mut grandparent = parent(index);
        match grandparent {
            0 => { return; }
            _ => {
                grandparent = parent(grandparent);
                if (self.data[index] < self.data[grandparent]) ^ max_level {
                    self.data.swap(grandparent, index);
                    self.do_bubble_up(grandparent, max_level);
                }
            }
        }
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 { return; }
        let condition = is_on_min_level(index);
        match condition {
             true => {
                if self.data[parent(index)] < self.data[index] {
                    self.data.swap(parent(index), index);
                    self.do_bubble_up(parent(index), true);
                } else {
                    self.do_bubble_up(index, false);
                }
            }
            false => {
                if self.data[parent(index)] > self.data[index] {
                    self.data.swap(parent(index), index);
                    self.do_bubble_up(parent(index), false);
                } else {
                    self.do_bubble_up(index, true);
                }
            }
        }
    }

    fn delete_element(&mut self, index: usize) {
        let len = self.len();
        if index == len - 1 {
            let _ = self.data.remove(self.data.len() - 1);
            return;
        }
        if index >= len { return; }

        self.data.swap(index, len - 1);
        let _ = self.data.remove(self.data.len() - 1);
        self.trickle_down(index);
    }
}
