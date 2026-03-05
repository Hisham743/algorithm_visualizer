#![feature(gen_blocks)]

pub trait Sort<T: Ord + Clone>: AsRef<[T]> + AsMut<[T]> {
    fn bubble_sort(&mut self) -> impl Iterator<Item = usize> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 0..(length - 1) {
                let mut swapped = false;

                for j in 0..(length - i - 1) {
                    yield j;

                    if self.as_ref()[j] > self.as_ref()[j + 1] {
                        self.as_mut().swap(j, j + 1);
                        swapped = true;
                    }
                }

                if !swapped {
                    break;
                }
            }
        }
    }

    fn selection_sort(&mut self) -> impl Iterator<Item = usize> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 0..(length - 1) {
                let mut min_index = i;

                for j in (i + 1)..length {
                    yield j;

                    if self.as_ref()[j] < self.as_ref()[min_index] {
                        min_index = j;
                    }
                }

                self.as_mut().swap(i, min_index);
            }
        }
    }

    fn insertion_sort(&mut self) -> impl Iterator<Item = usize> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 1..length {
                let mut insert_index = i;
                let current_value = self.as_ref()[i].clone();

                for j in (0..i).rev() {
                    yield j;

                    if self.as_ref()[j] > current_value {
                        self.as_mut()[j + 1] = self.as_ref()[j].clone();
                        insert_index = j;
                    } else {
                        break;
                    }
                }

                self.as_mut()[insert_index] = current_value;
            }
        }
    }

    fn merge_sort(&mut self) -> impl Iterator<Item = usize> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            let middle = length / 2;
            let mut buffer = Box::new(self.as_ref().to_vec());
            let (left_half, right_half) = buffer.split_at_mut(middle);

            for index in Box::new(left_half.merge_sort()) {
                yield index
            }

            for index in Box::new(right_half.merge_sort()) {
                yield index
            }

            let (mut i, mut j, mut k) = (0, 0, 0);

            while i < left_half.len() && j < right_half.len() {
                yield k;

                if left_half[i] < right_half[j] {
                    self.as_mut()[k] = left_half[i].clone();
                    i += 1;
                } else {
                    self.as_mut()[k] = right_half[j].clone();
                    j += 1;
                }

                k += 1;
            }

            while i < left_half.len() {
                yield k;

                self.as_mut()[k] = left_half[i].clone();
                i += 1;
                k += 1;
            }

            while j < right_half.len() {
                yield k;

                self.as_mut()[k] = right_half[j].clone();
                j += 1;
                k += 1;
            }
        }
    }
}

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
}
