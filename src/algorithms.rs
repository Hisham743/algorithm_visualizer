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

impl Algorithm {
    pub fn operations<T>(&self) -> fn(Vec<T>) -> Box<dyn Iterator<Item = Operation<T>>>
    where
        T: Ord + Clone + 'static,
    {
        match self {
            Algorithm::Bubble => |numbers| Box::new(bubble_sort(numbers)),
            Algorithm::Selection => |numbers| Box::new(selection_sort(numbers)),
            Algorithm::Insertion => |numbers| Box::new(insertion_sort(numbers)),
            Algorithm::Merge => |numbers| Box::new(merge_sort(numbers)),
            Algorithm::Quick => |numbers| Box::new(quick_sort(numbers)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation<T: Ord + Clone> {
    Compare(usize, usize),
    CompareValue(usize, T),
    Write(usize, T),
    Swap(usize, usize),
}

fn bubble_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Operation<T>> {
    gen move {}
}

fn selection_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Operation<T>> {
    gen move {}
}

fn insertion_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Operation<T>> {
    gen move {}
}

fn merge_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Operation<T>> {
    gen move {}
}

fn quick_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Operation<T>> {
    gen move {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    fn apply_operation(numbers: &mut [i32], operation: Operation<i32>) {
        match operation {
            Operation::Write(index, value) => numbers[index] = value,
            Operation::Swap(i, j) => numbers.swap(i, j),
            _ => {}
        }
    }

    macro_rules! test_algorithm {
        ($test_name:ident, $algorithm:expr) => {
            #[test]
            fn $test_name() {
                let sort_check = |mut numbers: Vec<i32>| {
                    let mut sorted = numbers.clone();
                    sorted.sort();

                    let numbers_clone = numbers.clone();
                    $algorithm.operations()(numbers_clone)
                        .for_each(|operation| apply_operation(&mut numbers, operation));

                    numbers == sorted
                };

                let empty = Vec::<i32>::new();
                assert!(sort_check(empty), "empty case");

                let single = vec![fastrand::i32(..)];
                assert!(sort_check(single), "single case");

                let numbers: Vec<_> = iter::repeat_with(|| fastrand::i32(..)).take(100).collect();

                let mut sorted = numbers.clone();
                sorted.sort();

                let mut reversed = sorted.clone();
                reversed.reverse();

                assert!(sort_check(numbers), "100 case");
                assert!(sort_check(sorted), "sorted case");
                assert!(sort_check(reversed), "reverse case");
            }
        };
    }

    test_algorithm!(bubble_sort_test, Algorithm::Bubble);
    test_algorithm!(selection_sort_test, Algorithm::Selection);
    test_algorithm!(insertion_sort_test, Algorithm::Insertion);
    test_algorithm!(merge_sort_test, Algorithm::Merge);
    test_algorithm!(quick_sort_test, Algorithm::Quick);
}
