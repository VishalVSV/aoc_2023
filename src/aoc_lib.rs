use std::ops::{Index, IndexMut};

pub struct Grid<T: Sized> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize
}

impl<T: Sized> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Sized> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Sized + Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![T::default(); width]; height],
            width,
            height
        }
    }
}