#![feature(gen_blocks)]

mod algorithms;

#[derive(Debug)]
pub struct SortingEngine<T: Ord>(Vec<T>);

impl<T: Ord> SortingEngine<T> {
    pub fn new() -> Self {
        SortingEngine(Vec::new())
    }

    pub fn get_elements(&self) -> &[T] {
        &self.0
    }

    pub fn set_elements(&mut self, elements: impl IntoIterator<Item = T>) {
        self.0 = elements.into_iter().collect();
    }

    pub fn shuffle(&mut self) {
        fastrand::shuffle(&mut self.0);
    }
}

impl<T: Ord> Default for SortingEngine<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I> From<I> for SortingEngine<T>
where
    T: Ord,
    I: Into<Vec<T>>,
{
    fn from(value: I) -> Self {
        SortingEngine(value.into())
    }
}
