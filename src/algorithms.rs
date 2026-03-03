use crate::SortingEngine;

impl<T: Ord> SortingEngine<T> {
    pub fn bubble_sort(&mut self) -> impl Iterator<Item = usize> {
        gen {
            let length = self.0.len();
            if length < 2 {
                return;
            }

            for i in 0..(length - 1) {
                for j in 0..(length - i - 1) {
                    yield j;

                    if self.0[j] > self.0[j + 1] {
                        self.0.swap(j, j + 1);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! test_algorithm {
        ($algorithm:ident) => {
            #[test]
            fn $algorithm() {
                let mut engine = SortingEngine::<i32>::new();

                engine.$algorithm().for_each(drop);
                assert!(engine.0.is_sorted(), "empty case");

                engine.0.push(fastrand::i32(..));
                engine.$algorithm().for_each(drop);
                assert!(engine.0.is_sorted(), "single case");

                engine.set_elements(iter::repeat_with(|| fastrand::i32(..)).take(100));
                engine.$algorithm().for_each(drop);
                assert!(engine.0.is_sorted(), "100 case");

                engine.$algorithm().for_each(drop);
                assert!(engine.0.is_sorted(), "sorted case");

                engine.0.reverse();
                engine.$algorithm().for_each(drop);
                assert!(engine.0.is_sorted(), "reverse case");
            }
        };
    }

    test_algorithm!(bubble_sort);
}
