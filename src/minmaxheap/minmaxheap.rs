pub struct MinMaxHeap<T> {
    data: Vec<T>,

}

impl<T: PartialOrd> MinMaxHeap<T> {

    pub fn new() -> Self {
        MinMaxHeap {
            data: vec![],
        }
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
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
