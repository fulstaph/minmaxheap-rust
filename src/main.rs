mod minmaxheap;
use minmaxheap::minmaxheap::MinMaxHeap;
use std::ops::Range;

fn main() {
    let mut heap: MinMaxHeap<i32> = MinMaxHeap::new();


    for val in 0..8 {
        heap.push(val);
    }

    let _ = heap.pop_max();
    let _ = heap.pop_min();
    // and other stuff

    println!("{:?}", heap);
}
