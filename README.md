# Min-Max Heap Data Structure implementation in Rustlang
My attempt at implementing min-max heap as a part of learning Rust.
Implementation is based on paper by M.D Atkinson, J.-R. Sack and others, 
["Min-Max Heaps And Generalized Priority Queues"](http://akira.ruc.dk/~keld/teaching/algoritmedesign_f03/Artikler/02/Atkinson86.pdf)
and [Wikipedia](https://en.wikipedia.org/wiki/Min-max_heap) article. 
A structure of this type can be used to implement DEPQ (Double Ended Priority Queue).
## Implementation details
- Min-max heap is generic, works for every type that implements `Copy` and `Clone` traits;
- `Vec<T>` is used for underlying array representation.
- Helper functions are defined and implemented in `utils_funcs` module;
- A small number of tests are written and located in `minmaxheap/minmaxheap_tests.rs` file;
## Methods
- `push` is used to insert an element into the heap;
- `pop_min` takes minimum out of the heap and returns it;
- `pop_max` takes maximum out of the heap and returns it;
- `peek_min` and `peek_max` return heap's minimum and maximum  