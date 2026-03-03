use crate::SortingEngine;

impl<T: Ord> SortingEngine<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! test_algorithm {
        ($algorithm:ident) => {
            #[test]
            fn $algorithm() {
                let engine = SortingEngine::<i32>::new();

                engine.$algorithm().collect();
                assert!(engine.0.is_sorted(), "empty case");

                engine.0.push(fastrand::i32(..));
                engine.$algorithm().collect();
                assert!(engine.0.is_sorted(), "single case");

                engine.set_elements(iter::repeat_with(|| fastrand::i32(..)).take(100).collect());
                engine.$algorithm().collect();
                assert!(engine.0.is_sorted(), "100 case");

                engine.$algorithm().collect();
                assert!(engine.0.is_sorted(), "sorted case");

                engine.0.reverse();
                engine.$algorithm().collect();
                assert!(engine.0.is_sorted(), "reverse case");
            }
        };
    }

    test_algorithm!(bubble_sort);
}
