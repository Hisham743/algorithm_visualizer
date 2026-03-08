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
    pub fn steps<T>(&self) -> fn(Vec<T>) -> Box<dyn Iterator<Item = Snapshot<T>>>
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
pub struct Snapshot<T: Ord + Clone> {
    pub numbers: Vec<T>,
    pub active_element: Option<usize>,
}

fn bubble_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Snapshot<T>> {
    gen move {
        let length = numbers.len();
        if length < 2 {
            yield Snapshot {
                numbers,
                active_element: None,
            };
            return;
        }

        for i in 0..(length - 1) {
            let mut swapped = false;

            for j in 0..(length - i - 1) {
                yield Snapshot {
                    numbers: numbers.clone(),
                    active_element: Some(j),
                };

                if numbers[j] > numbers[j + 1] {
                    numbers.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }

        yield Snapshot {
            numbers,
            active_element: None,
        };
    }
}

fn selection_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Snapshot<T>> {
    gen move {
        let length = numbers.len();
        if length < 2 {
            yield Snapshot {
                numbers,
                active_element: None,
            };
            return;
        }

        for i in 0..(length - 1) {
            let mut min_index = i;

            for j in (i + 1)..length {
                yield Snapshot {
                    numbers: numbers.clone(),
                    active_element: Some(j),
                };

                if numbers[j] < numbers[min_index] {
                    min_index = j;
                }
            }

            numbers.swap(i, min_index);
        }

        yield Snapshot {
            numbers,
            active_element: None,
        };
    }
}

fn insertion_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Snapshot<T>> {
    gen move {
        let length = numbers.len();
        if length < 2 {
            yield Snapshot {
                numbers,
                active_element: None,
            };
            return;
        }

        for i in 1..length {
            let mut insert_index = i;
            let current_value = numbers[i].clone();

            for j in (0..i).rev() {
                yield Snapshot {
                    numbers: numbers.clone(),
                    active_element: Some(j),
                };

                if numbers[j] > current_value {
                    numbers[j + 1] = numbers[j].clone();
                    insert_index = j;
                } else {
                    break;
                }
            }

            numbers[insert_index] = current_value;
        }

        yield Snapshot {
            numbers,
            active_element: None,
        };
    }
}

fn merge_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Snapshot<T>> {
    gen move {
        let length = numbers.len();
        if length < 2 {
            yield Snapshot {
                numbers,
                active_element: None,
            };
            return;
        }

        let middle = length / 2;
        let left_half = numbers[..middle].to_vec();
        let right_half = numbers[middle..].to_vec();
        let right_half_len = right_half.len();

        let mut sorted_left_half = Vec::new();
        for mut snapshot in Box::new(merge_sort(left_half)) {
            sorted_left_half = snapshot.numbers.clone();
            snapshot.numbers.extend_from_slice(&right_half);
            yield snapshot;
        }

        let mut sorted_right_half = Vec::new();
        for snapshot in Box::new(merge_sort(right_half)) {
            sorted_right_half = snapshot.numbers.clone();

            let mut full = Vec::with_capacity(middle + right_half_len);
            full.extend_from_slice(&sorted_left_half);
            full.extend_from_slice(&snapshot.numbers);

            let active_element = snapshot.active_element.map(|index| middle + index);
            yield Snapshot {
                numbers: full,
                active_element,
            }
        }

        let (mut i, mut j, mut k) = (0, 0, 0);

        while i < sorted_left_half.len() && j < sorted_right_half.len() {
            yield Snapshot {
                numbers: numbers.clone(),
                active_element: Some(k),
            };

            if sorted_left_half[i] < sorted_right_half[j] {
                numbers[k] = sorted_left_half[i].clone();
                i += 1;
            } else {
                numbers[k] = sorted_right_half[j].clone();
                j += 1;
            }

            k += 1;
        }

        while i < sorted_left_half.len() {
            yield Snapshot {
                numbers: numbers.clone(),
                active_element: Some(k),
            };
            numbers[k] = sorted_left_half[i].clone();
            i += 1;
            k += 1;
        }

        while j < sorted_right_half.len() {
            yield Snapshot {
                numbers: numbers.clone(),
                active_element: Some(k),
            };

            numbers[k] = sorted_right_half[j].clone();
            j += 1;
            k += 1;
        }

        yield Snapshot {
            numbers,
            active_element: None,
        };
    }
}

fn quick_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> impl Iterator<Item = Snapshot<T>> {
    gen move {
        let length = numbers.len();
        if length < 2 {
            yield Snapshot {
                numbers,
                active_element: None,
            };
            return;
        }

        let pivot = numbers[length - 1].clone();
        let mut i = 0;

        for j in 0..(length - 1) {
            yield Snapshot {
                numbers: numbers.clone(),
                active_element: Some(j),
            };

            if numbers[j] <= pivot {
                numbers.swap(i, j);
                i += 1;
            }
        }

        numbers.swap(i, length - 1);

        let left = numbers[..i].to_vec();
        let right = numbers[i + 1..].to_vec();
        let pivot = numbers[i].clone();

        let mut sorted_left = Vec::new();
        for mut snapshot in Box::new(quick_sort(left)) {
            sorted_left = snapshot.numbers.clone();
            snapshot.numbers.push(pivot.clone());
            snapshot.numbers.extend_from_slice(&right);
            yield snapshot;
        }

        let mut sorted_right = Vec::new();
        for snapshot in Box::new(quick_sort(right)) {
            sorted_right = snapshot.numbers.clone();

            let mut full = Vec::with_capacity(i + snapshot.numbers.len() + 1);
            full.extend_from_slice(&sorted_left);
            full.push(pivot.clone());
            full.extend_from_slice(&snapshot.numbers);

            let active_element = snapshot.active_element.map(|index| i + index + 1);
            yield Snapshot {
                numbers: full,
                active_element,
            }
        }

        numbers = sorted_left;
        numbers.push(pivot);
        numbers.extend_from_slice(&sorted_right);

        yield Snapshot {
            numbers,
            active_element: None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! test_algorithm {
        ($algorithm:ident, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let sorted_numbers =
                    |numbers: Vec<i32>| $algorithm(numbers).last().unwrap().numbers;

                let empty = Vec::<i32>::new();
                assert_eq!(sorted_numbers(empty), Vec::new(), "empty case");

                let single = vec![fastrand::i32(..)];
                let clone = single.clone();
                assert_eq!(sorted_numbers(single), clone, "single case");

                let numbers = iter::repeat_with(|| fastrand::i32(..)).take(100).collect();
                let sorted = sorted_numbers(numbers);

                let mut reverse = sorted.clone();
                reverse.reverse();

                assert!(sorted.is_sorted(), "100 case");
                assert!(sorted_numbers(sorted).is_sorted(), "sorted case");
                assert!(sorted_numbers(reverse).is_sorted(), "reverse case");
            }
        };
    }

    test_algorithm!(bubble_sort, bubble_sort_test);
    test_algorithm!(selection_sort, selection_sort_test);
    test_algorithm!(insertion_sort, insertion_sort_test);
    test_algorithm!(merge_sort, merge_sort_test);
    test_algorithm!(quick_sort, quick_sort_test);
}
