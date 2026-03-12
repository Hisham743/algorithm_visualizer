use std::{fmt::Display, vec::IntoIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Bubble,
    Selection,
    Insertion,
    Merge,
    Quick,
    Heap,
    Gnome,
    Cocktail,
    OddEven,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} Sort", &self)
    }
}

impl Algorithm {
    pub fn operations<T>(&self) -> fn(Vec<T>) -> IntoIter<Operation<T>>
    where
        T: Ord + Clone + 'static,
    {
        match self {
            Self::Bubble => |numbers| bubble_sort(numbers),
            Self::Selection => |numbers| selection_sort(numbers),
            Self::Insertion => |numbers| insertion_sort(numbers),
            Self::Merge => |numbers| merge_sort(numbers),
            Self::Quick => |numbers| quick_sort(numbers),
            Self::Heap => |numbers| heap_sort(numbers),
            Self::Gnome => |numbers| gnome_sort(numbers),
            Self::Cocktail => |numbers| cocktail_sort(numbers),
            Self::OddEven => |numbers| odd_even_sort(numbers),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation<T: Ord + Clone> {
    Compare(usize, usize),
    CompareToValue(usize),
    Write(usize, usize),
    WriteValue(usize, T),
    Swap(usize, usize),
}

impl<T: Ord + Copy> Copy for Operation<T> {}

impl<T: Ord + Clone> Operation<T> {
    pub fn apply(self, numbers: &mut [T]) {
        match self {
            Operation::Write(i, j) => numbers[i] = numbers[j].clone(),
            Operation::WriteValue(index, value) => numbers[index] = value,
            Operation::Swap(i, j) => numbers.swap(i, j),
            _ => {}
        }
    }
}

impl<T: Ord + Clone> Operation<T> {
    fn shift_index(&mut self, shift: usize) {
        *self = match self {
            Self::Compare(i, j) => Self::Compare(*i + shift, *j + shift),
            Self::CompareToValue(index) => Self::CompareToValue(*index + shift),
            Self::Write(i, j) => Self::Write(*i + shift, *j + shift),
            Self::WriteValue(index, value) => Self::WriteValue(*index + shift, value.clone()),
            Self::Swap(i, j) => Self::Swap(*i + shift, *j + shift),
        };
    }
}

fn bubble_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    for i in 0..(length - 1) {
        let mut swapped = false;

        for j in 0..(length - i - 1) {
            operations.push(Operation::Compare(j, j + 1));

            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
                operations.push(Operation::Swap(j, j + 1));
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    operations.into_iter()
}

fn selection_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    for i in 0..(length - 1) {
        let mut min_index = i;

        for j in (i + 1)..length {
            operations.push(Operation::Compare(j, min_index));

            if numbers[j] < numbers[min_index] {
                min_index = j;
            }
        }

        numbers.swap(i, min_index);
        operations.push(Operation::Swap(i, min_index));
    }

    operations.into_iter()
}

fn insertion_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    for i in 1..length {
        let mut insert_index = i;
        let current_value = numbers[i].clone();

        for j in (0..i).rev() {
            operations.push(Operation::CompareToValue(j));

            if numbers[j] > current_value {
                numbers[j + 1] = numbers[j].clone();
                operations.push(Operation::Write(j + 1, j));
                insert_index = j;
            } else {
                break;
            }
        }

        numbers[insert_index] = current_value.clone();
        operations.push(Operation::WriteValue(insert_index, current_value))
    }

    operations.into_iter()
}

fn merge_sort_inner<T: Ord + Clone>(numbers: &mut [T]) -> Vec<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations;
    }

    let middle = length / 2;
    let mut buffer = numbers.to_vec();
    let (left_half, right_half) = buffer.split_at_mut(middle);

    operations.extend(merge_sort_inner(left_half));

    let mut right_operations = merge_sort_inner(right_half);
    for operation in &mut right_operations {
        operation.shift_index(middle);
    }
    operations.extend(right_operations);

    let (mut i, mut j, mut k) = (0, 0, 0);
    let left_half_len = left_half.len();
    let right_half_len = right_half.len();

    while i < left_half_len && j < right_half_len {
        if left_half[i] < right_half[j] {
            numbers[k] = left_half[i].clone();
            operations.push(Operation::WriteValue(k, left_half[i].clone()));
            i += 1;
        } else {
            numbers[k] = right_half[j].clone();
            operations.push(Operation::WriteValue(k, right_half[j].clone()));
            j += 1;
        }

        k += 1;
    }

    while i < left_half_len {
        numbers[k] = left_half[i].clone();
        operations.push(Operation::WriteValue(k, left_half[i].clone()));
        i += 1;
        k += 1;
    }

    while j < right_half_len {
        numbers[k] = right_half[j].clone();
        operations.push(Operation::WriteValue(k, right_half[j].clone()));
        j += 1;
        k += 1;
    }

    operations
}

fn merge_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    merge_sort_inner(&mut numbers).into_iter()
}

fn quick_sort_inner<T: Ord + Clone>(numbers: &mut [T]) -> Vec<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations;
    }

    let pivot_element = numbers[(length - 1) / 2].clone();
    let (mut i, mut j) = (0, length - 1);

    let pivot_index = loop {
        while numbers[i] < pivot_element {
            operations.push(Operation::CompareToValue(i));
            i += 1;
        }

        while numbers[j] > pivot_element {
            operations.push(Operation::CompareToValue(j));
            j -= 1;
        }

        if i >= j {
            break j;
        }

        numbers.swap(i, j);
        operations.push(Operation::Swap(i, j));
    };

    let (left, right) = numbers.split_at_mut(pivot_index);

    operations.extend(quick_sort_inner(left));

    let mut right_operations = quick_sort_inner(&mut right[1..]);
    for operation in &mut right_operations {
        operation.shift_index(pivot_index + 1);
    }
    operations.extend(right_operations);

    operations
}

fn quick_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    quick_sort_inner(&mut numbers).into_iter()
}

fn heapify<T: Ord + Clone>(numbers: &mut [T], n: usize, i: usize) -> Vec<Operation<T>> {
    let mut operations = Vec::new();

    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n {
        operations.push(Operation::Compare(left, largest));
        if numbers[left] > numbers[largest] {
            largest = left;
        }
    }

    if right < n {
        operations.push(Operation::Compare(right, largest));
        if numbers[right] > numbers[largest] {
            largest = right;
        }
    }

    if largest != i {
        numbers.swap(i, largest);
        operations.push(Operation::Swap(i, largest));
        operations.extend(heapify(numbers, n, largest));
    }

    operations
}

fn heap_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    (0..=(length / 2 - 1))
        .rev()
        .for_each(|i| operations.extend(heapify(&mut numbers, length, i)));

    (1..=(length - 1)).rev().for_each(|i| {
        numbers.swap(0, i);
        operations.push(Operation::Swap(0, i));
        operations.extend(heapify(&mut numbers, i, 0));
    });

    operations.into_iter()
}

fn gnome_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    let mut index = 0;
    while index < length {
        if index == 0 {
            index += 1;
        }

        operations.push(Operation::Compare(index, index - 1));
        if numbers[index] >= numbers[index - 1] {
            index += 1;
        } else {
            numbers.swap(index, index - 1);
            operations.push(Operation::Swap(index, index - 1));
            index -= 1;
        }
    }

    operations.into_iter()
}

fn cocktail_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    let mut swapped = true;
    let mut start = 0;
    let mut end = length - 1;

    while swapped {
        swapped = false;

        for i in start..end {
            operations.push(Operation::Compare(i, i + 1));
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                operations.push(Operation::Swap(i, i + 1));
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        for i in (start..=(end - 1)).rev() {
            operations.push(Operation::Compare(i, i + 1));
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                operations.push(Operation::Swap(i, i + 1));
                swapped = true;
            }
        }

        start += 1;
    }

    operations.into_iter()
}

fn odd_even_sort<T: Ord + Clone>(mut numbers: Vec<T>) -> IntoIter<Operation<T>> {
    let mut operations = Vec::new();

    let length = numbers.len();
    if length < 2 {
        return operations.into_iter();
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;

        for i in (1..(length - 1)).filter(|i| i % 2 == 1) {
            operations.push(Operation::Compare(i, i + 1));
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                operations.push(Operation::Swap(i, i + 1));
                sorted = false;
            }
        }

        for i in (0..(length - 1)).filter(|i| i % 2 == 0) {
            operations.push(Operation::Compare(i, i + 1));
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                operations.push(Operation::Swap(i, i + 1));
                sorted = false;
            }
        }
    }

    operations.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! test_algorithm {
        ($test_name:ident, $algorithm:expr) => {
            #[test]
            fn $test_name() {
                let sort_check = |mut numbers: Vec<i32>| {
                    let mut sorted = numbers.clone();
                    sorted.sort();

                    let numbers_clone = numbers.clone();
                    $algorithm.operations()(numbers_clone)
                        .for_each(|operation| operation.apply(&mut numbers));

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
    test_algorithm!(heap_sort_test, Algorithm::Heap);
    test_algorithm!(gnome_sort_test, Algorithm::Gnome);
    test_algorithm!(cocktail_sort_test, Algorithm::Cocktail);
    test_algorithm!(odd_even_sort, Algorithm::OddEven);
}
