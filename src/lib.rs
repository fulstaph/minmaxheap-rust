/*!

A min-max heap, implemented as a way to learn some
Rust. Heap data must be `PartialOrd`.

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
#[derive(Debug, Default)]
pub struct MinMaxHeap<T> {
    data: Vec<T>,
}

fn log2(value: usize) -> u32 {
    assert!(value > 0);
    8 * std::mem::size_of::<usize>() as u32 - value.leading_zeros()
}

fn parent(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child(index: usize) -> usize {
    2 * index + 1
}

fn is_on_min_level(index: usize) -> bool {
    log2(index + 1) % 2 == 1
}

impl<T: PartialOrd> std::convert::From<Vec<T>> for MinMaxHeap<T> {
    fn from(v: Vec<T>) -> Self {
        let mut heap: MinMaxHeap<T> = MinMaxHeap::with_capacity(v.len());
        for val in v {
            heap.push(val);
        }
        heap
    }
}

impl<T: PartialOrd> MinMaxHeap<T> {

    /// Make a new heap.
    pub fn new() -> Self {
        MinMaxHeap {
            data: vec![],
        }
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
    pub fn peek_min(&self) -> Option<&T> {
        if self.is_empty() { return None }
        Some(&self.data[0])
    }

    /// Show a maximum element from the heap.
    pub fn peek_max(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            1 => Some(&self.data[0]),
            2 => Some(&self.data[1]),
            _ => {
                let max = if self.data[1] > self.data[2] {
                    &self.data[1]
                } else {
                    &self.data[2]
                };
                Some(max)
            }
        }
    }

    /// Extract a minimum element from the heap.
    pub fn pop_min(&mut self) -> Option<T> {
        self.delete_element(0)
    }

    /// Extract a maximum element from the heap.
    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => self.delete_element(0),
            2 => self.delete_element(1),
            _ => {
                let max_ind = if self.data[1] > self.data[2] {
                    1
                } else {
                    2
                };
                self.delete_element(max_ind)
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
            0 => (),
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
        #![allow(clippy::collapsible_if)]
        if index == 0 { return; }
        if is_on_min_level(index) {
            if self.data[parent(index)] < self.data[index] {
                self.data.swap(parent(index), index);
                self.do_bubble_up(parent(index), true);
            } else {
                self.do_bubble_up(index, false);
            }
        } else {
            if self.data[parent(index)] > self.data[index] {
                self.data.swap(parent(index), index);
                self.do_bubble_up(parent(index), false);
            } else {
                self.do_bubble_up(index, true);
            }
        }
    }

    fn delete_element(&mut self, index: usize) -> Option<T> {
        let len = self.len();
        if index >= len {
            return None;
        }
        if index == len - 1 {
            return Some(self.data.remove(self.data.len() - 1));
        }
        self.data.swap(index, len - 1);
        let result = self.data.remove(self.data.len() - 1);
        self.trickle_down(index);
        Some(result)
    }
}
