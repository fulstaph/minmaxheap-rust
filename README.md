# Min-Max Heap in Rust

A min-max heap, implemented as a way to learn some
Rust. Heap data must be `Copy`, `Clone`, `PartialOrd`.

The implementation is based on a paper by M.D Atkinson,
J.-R. Sack and others,
"[Min-Max Heaps And Generalized Priority Queues](http://akira.ruc.dk/~keld/teaching/algoritmedesign_f03/Artikler/02/Atkinson86.pdf)"
and on the relevant
[Wikipedia](https://en.wikipedia.org/wiki/Min-max_heap)
article.

This heap can be used as a DEPQ (Double Ended Priority
Queue) implementation.

## Implementation details

- `Vec<T>` is used for underlying array representation.

- A small number of tests are written and located in
  `minmaxheap/minmaxheap_tests.rs` file;
