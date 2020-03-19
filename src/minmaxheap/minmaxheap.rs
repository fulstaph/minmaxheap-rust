use crate::minmaxheap::minmaxheap::utils_funcs::{left_child, parent};
use conditional::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct MinMaxHeap<T> {
    data: Vec<T>,
}

impl<T: std::clone::Clone + PartialOrd + Copy> MinMaxHeap<T> {
    // public API
    pub fn new() -> Self {
        MinMaxHeap {
            data: vec![],
        }
    }

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

    pub fn with_capacity(len: usize) -> Self {
        MinMaxHeap {
            data: Vec::with_capacity(len)
        }
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.size() - 1);
    }

    pub fn peek_min(&self) -> Option<T> {
        if self.empty() { return None }
        return Some(self.data[0].clone())
    }

    pub fn peek_max(&self) -> Option<T> {
        match self.size() {
            0 => None,
            1 => Some(self.data[0].clone()),
            2 => Some(self.data[1].clone()),
            _ => {
                let max = conditional!(self.data[1] > self.data[2]?
                    self.data[1].clone() : self.data[2].clone()
                );
                Some(max)
            }
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        match self.size() {
            0 => None,
            _ => {
                let min = self.data[0].clone();
                self.delete_element(0);
                Some(min)
            }
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        match self.size() {
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
                let max_ind = conditional!(self.data[1] > self.data[2]? 1 : 2);
                let max = self.data[max_ind].clone();
                self.delete_element(max_ind);
                Some(max)
            }
        }
    }

    pub fn erase(&mut self) {
        self.data.clear()
    }

    // private methods
    fn _trickle_down(&mut self, index: usize, max_level: bool) {
        if index >= self.size() { return; }
        let mut min_node = index;
        let left = left_child(index);
        if left < self.size() && ((self.data[left] < self.data[min_node]) ^ max_level) { min_node = left; }
        if left + 1 < self.size() && ((self.data[left + 1] < self.data[min_node]) ^ max_level) { min_node = left + 1; }
        let left_gchild = left_child(left);
        let mut i = 0;
        while (i < 4) && (left_gchild + i < self.size()) {
            if (self.data[left_gchild + i] < self.data[min_node]) ^ max_level {
                min_node = left_gchild + i;
            }
            i += 1;
        }
        if index == min_node { return; }
        self.data.swap(index, min_node);
        if min_node - left > 1 {
            if (self.data[utils_funcs::parent(min_node)] < self.data[min_node]) ^ max_level {
                self.data.swap(utils_funcs::parent(min_node), min_node);
            }
            self._trickle_down(min_node, max_level);
        }
    }

    fn trickle_down(&mut self, index: usize) {
            if utils_funcs::is_on_min_level(index) {
                self._trickle_down(index, false);
            } else {
                self._trickle_down(index, true);
            }
    }

    fn _bubble_up(&mut self, index: usize, max_level: bool) {
        if index == 0 { return; }
        let mut grandparent = parent(index);
        match grandparent {
            0 => { return; }
            _ => {
                grandparent = parent(grandparent);
                if (self.data[index] < self.data[grandparent]) ^ max_level {
                    self.data.swap(grandparent, index);
                    self._bubble_up(grandparent, max_level);
                }
            }
        }
    }

    fn bubble_up(&mut self, index: usize) {
        if index == 0 { return; }
        let condition = utils_funcs::is_on_min_level(index);
        match condition {
             true => {
                if self.data[parent(index)] < self.data[index] {
                    self.data.swap(parent(index), index);
                    self._bubble_up(parent(index), true);
                } else {
                    self._bubble_up(index, false);
                }
            }
            false => {
                if self.data[parent(index)] > self.data[index] {
                    self.data.swap(parent(index), index);
                    self._bubble_up(parent(index), false);
                } else {
                    self._bubble_up(index, true);
                }
            }
        }
    }

    fn delete_element(&mut self, index: usize) {
        let len = self.size();
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

// helper functions

mod utils_funcs {

    pub fn log2(mut value: usize) -> Option<usize> {
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

    pub fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    pub fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    pub fn is_on_min_level(index: usize) -> bool {
        (log2(index + 1).unwrap_or_else(|| {
            println!("can't calc log2 of 0");
            0
        }) % 2) == 1
    }

    pub fn is_on_max_level(index: usize) -> bool {
        !is_on_min_level(index)
    }
}
