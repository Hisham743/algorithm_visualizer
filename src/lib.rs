#![feature(gen_blocks)]

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} Sort", &self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot<T: Ord + Clone> {
    pub numbers: Vec<T>,
    pub active_element: Option<usize>,
}

pub trait Sortable<T: Ord + Clone>: AsRef<[T]> + AsMut<[T]> {
    fn bubble_sort(&mut self) -> impl Iterator<Item = Snapshot<T>> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 0..(length - 1) {
                let mut swapped = false;

                for j in 0..(length - i - 1) {
                    yield Snapshot {
                        numbers: self.as_ref().to_vec(),
                        active_element: Some(j),
                    };

                    if self.as_ref()[j] > self.as_ref()[j + 1] {
                        self.as_mut().swap(j, j + 1);
                        swapped = true;
                    }
                }

                if !swapped {
                    break;
                }
            }

            yield Snapshot {
                numbers: self.as_ref().to_vec(),
                active_element: None,
            };
        }
    }

    fn selection_sort(&mut self) -> impl Iterator<Item = Snapshot<T>> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 0..(length - 1) {
                let mut min_index = i;

                for j in (i + 1)..length {
                    yield Snapshot {
                        numbers: self.as_ref().to_vec(),
                        active_element: Some(j),
                    };

                    if self.as_ref()[j] < self.as_ref()[min_index] {
                        min_index = j;
                    }
                }

                self.as_mut().swap(i, min_index);
            }

            yield Snapshot {
                numbers: self.as_ref().to_vec(),
                active_element: None,
            };
        }
    }

    fn insertion_sort(&mut self) -> impl Iterator<Item = Snapshot<T>> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            for i in 1..length {
                let mut insert_index = i;
                let current_value = self.as_ref()[i].clone();

                for j in (0..i).rev() {
                    yield Snapshot {
                        numbers: self.as_ref().to_vec(),
                        active_element: Some(j),
                    };

                    if self.as_ref()[j] > current_value {
                        self.as_mut()[j + 1] = self.as_ref()[j].clone();
                        insert_index = j;
                    } else {
                        break;
                    }
                }

                self.as_mut()[insert_index] = current_value;
            }

            yield Snapshot {
                numbers: self.as_ref().to_vec(),
                active_element: None,
            };
        }
    }

    fn merge_sort(&mut self) -> impl Iterator<Item = Snapshot<T>> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            let middle = length / 2;
            let mut buffer = Box::new(self.as_ref().to_vec());
            let (left_half, right_half) = buffer.split_at_mut(middle);

            for mut snapshot in Box::new(left_half.merge_sort()) {
                snapshot.numbers.extend_from_slice(right_half);
                yield snapshot;
            }

            for snapshot in Box::new(right_half.merge_sort()) {
                let mut full = Vec::with_capacity(middle + snapshot.numbers.len());
                full.extend_from_slice(left_half);
                full.extend_from_slice(&snapshot.numbers);

                let active_element = snapshot.active_element.map(|index| middle + index);
                yield Snapshot {
                    numbers: full,
                    active_element,
                }
            }

            let (mut i, mut j, mut k) = (0, 0, 0);

            while i < left_half.len() && j < right_half.len() {
                yield Snapshot {
                    numbers: self.as_ref().to_vec(),
                    active_element: Some(k),
                };

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
                yield Snapshot {
                    numbers: self.as_ref().to_vec(),
                    active_element: Some(k),
                };
                self.as_mut()[k] = left_half[i].clone();
                i += 1;
                k += 1;
            }

            while j < right_half.len() {
                yield Snapshot {
                    numbers: self.as_ref().to_vec(),
                    active_element: Some(k),
                };

                self.as_mut()[k] = right_half[j].clone();
                j += 1;
                k += 1;
            }

            yield Snapshot {
                numbers: self.as_ref().to_vec(),
                active_element: None,
            };
        }
    }

    fn quick_sort(&mut self) -> impl Iterator<Item = Snapshot<T>> {
        gen move {
            let length = self.as_ref().len();
            if length < 2 {
                return;
            }

            let pivot = self.as_ref()[length - 1].clone();
            let mut i = 0;

            for j in 0..(length - 1) {
                yield Snapshot {
                    numbers: self.as_ref().to_vec(),
                    active_element: Some(j),
                };

                if self.as_ref()[j] <= pivot {
                    self.as_mut().swap(i, j);
                    i += 1;
                }
            }

            self.as_mut().swap(i, length - 1);

            let (left, right) = self.as_mut().split_at_mut(i);

            for mut snapshot in Box::new(left.quick_sort()) {
                snapshot.numbers.extend_from_slice(right);
                yield snapshot;
            }

            for snapshot in Box::new(right.quick_sort()) {
                let mut full = Vec::with_capacity(i + snapshot.numbers.len() + 1);
                full.extend_from_slice(left);
                full.extend_from_slice(&snapshot.numbers);

                let active_element = snapshot.active_element.map(|index| i + index + 1);
                yield Snapshot {
                    numbers: full,
                    active_element,
                }
            }
        }
    }
}

impl<T: Ord + Clone> Sortable<T> for [T] {}

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
