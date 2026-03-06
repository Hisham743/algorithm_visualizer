#![feature(gen_blocks)]

use std::marker::PhantomData;

pub mod algorithms;

#[derive(Debug)]
pub struct SortingEngine<T, U>
where
    T: Ord,
    U: AsRef<[T]> + AsMut<[T]>,
{
    data: U,
    _marker: PhantomData<T>,
}

impl<T, U> SortingEngine<T, U>
where
    T: Ord,
    U: AsRef<[T]> + AsMut<[T]>,
{
    pub fn get_elements(&self) -> &[T] {
        self.data.as_ref()
    }

    pub fn set_elements(&mut self, elements: U) {
        self.data = elements;
    }

    pub fn shuffle(&mut self) {
        fastrand::shuffle(self.data.as_mut());
    }
}

impl<T, U> From<U> for SortingEngine<T, U>
where
    T: Ord,
    U: AsRef<[T]> + AsMut<[T]>,
{
    fn from(value: U) -> Self {
        SortingEngine {
            data: value,
            _marker: PhantomData,
        }
    }
}
