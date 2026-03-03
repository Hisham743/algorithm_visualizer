#![feature(gen_blocks)]

pub trait Sort<T: Ord + Clone>: AsRef<[T]> + AsMut<[T]> {}

impl<T: Ord + Clone> Sort<T> for [T] {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! test_algorithm {
        ($algorithm:ident) => {
            #[test]
            fn $algorithm() {
                let mut numbers = Vec::new();

                numbers.$algorithm().for_each(drop);
                assert!(numbers.is_sorted(), "empty case");

                numbers.push(fastrand::i32(..));
                numbers.$algorithm().for_each(drop);
                assert!(numbers.is_sorted(), "single case");

                numbers = iter::repeat_with(|| fastrand::i32(..)).take(100).collect();
                numbers.$algorithm().for_each(drop);
                assert!(numbers.is_sorted(), "100 case");

                numbers.$algorithm().for_each(drop);
                assert!(numbers.is_sorted(), "sorted case");

                numbers.reverse();
                numbers.$algorithm().for_each(drop);
                assert!(numbers.is_sorted(), "reverse case");
            }
        };
    }

    test_algorithm!(bubble_sort);
    test_algorithm!(selection_sort);
    test_algorithm!(insertion_sort);
    test_algorithm!(merge_sort);
    test_algorithm!(quick_sort);
}
