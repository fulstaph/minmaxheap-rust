use crate::minmaxheap::minmaxheap::MinMaxHeap;
use rand::Rng;
use crate::minmaxheap::minmaxheap_tests::Operation::{Insert, Insert2, PeekMin, PeekMax, PopMax, PopMin};

#[test]
fn peek_min_test() {
    let v = generate_random_data_set(256);
    let heap = MinMaxHeap::from(&v);
    assert_eq!(heap.peek_min().unwrap(), *v.iter().min().unwrap())
}


#[test]
fn peek_max_test() {
    let v = generate_random_data_set(256);
    let heap = MinMaxHeap::from(&v);
    assert_eq!(heap.peek_max().unwrap(), *v.iter().max().unwrap())
}

#[test]
fn pop_last_test() {
    let mut heap = MinMaxHeap::from(&vec![1]);
    let _ = heap.pop_min();
    assert_eq!(heap.pop_min(), None)
}

#[test]
fn pop_min_test() {
    let mut v = generate_random_data_set(256);
    let mut heap = MinMaxHeap::from(&v);
    assert_eq!(heap.pop_min().unwrap(), *v.iter().min().unwrap());
    let _ = v.remove(v.iter()
                .enumerate()
                .map(|(x, y)| (y, x))
                .min()
                .unwrap().1
    );

    assert_eq!(heap.size(), v.len())
}

#[test]
fn pop_max_test() {
    let mut v = generate_random_data_set(256);
    let mut heap = MinMaxHeap::from(&v);
    assert_eq!(heap.pop_max().unwrap(), *v.iter().max().unwrap());
    let _ = v.remove(v.iter()
        .enumerate()
        .map(|(x, y)| (y, x))
        .max()
        .unwrap().1
    );

    assert_eq!(heap.size(), v.len())
}

#[test]
fn weird_test() {
    let mut cor_vec = generate_random_data_set(8);
    let mut heap = MinMaxHeap::from(&cor_vec);
    let v = generate_random_data_set(1_024);
    for val in &v {
        let operation = Operation::from(
                    rand::thread_rng()
                                .gen_range(0, 5) as i32)
                        .unwrap();
        match operation {
            Operation::Insert => {
                heap.push(*val);
                cor_vec.push(*val);
                assert_eq!(heap.size(), cor_vec.len());
            }
            Operation::Insert2 => {
                heap.push(*val);
                cor_vec.push(*val);
                assert_eq!(heap.size(), cor_vec.len());
            }
            Operation::PopMax => {
                assert_eq!(heap.pop_max().unwrap(), *cor_vec.iter().max().unwrap());
                let _ = cor_vec.remove(cor_vec.iter()
                    .enumerate()
                    .map(|(x, y)| (y, x))
                    .max()
                    .unwrap().1
                );

                assert_eq!(heap.size(), cor_vec.len())
            }
            Operation::PopMin => {
                assert_eq!(heap.pop_min().unwrap(), *cor_vec.iter().min().unwrap());
                let _ = cor_vec.remove(cor_vec.iter()
                    .enumerate()
                    .map(|(x, y)| (y, x))
                    .min()
                    .unwrap().1
                );

                assert_eq!(heap.size(), cor_vec.len())
            }
            Operation::PeekMax => {
                assert_eq!(heap.peek_max().unwrap(), *cor_vec.iter().max().unwrap())
            }
            Operation::PeekMin => {
                assert_eq!(heap.peek_min().unwrap(), *cor_vec.iter().min().unwrap())
            }
        }
    }
}

// helpers funcs and structs

enum Operation {
    Insert,
    Insert2,
    PeekMin,
    PeekMax,
    PopMin,
    PopMax,
}

impl Operation {
    pub fn from(num: i32) -> Option<Operation> {
        match num {
            0 => Some(Insert),
            1 => Some(Insert2),
            2 => Some(PeekMin),
            3 => Some(PeekMax),
            4 => Some(PopMin),
            5 => Some(PopMax),
            _ => None
        }
    }
}

fn generate_random_data_set(size: usize) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::random())
    }
    v
}