use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn from_vec(vec: Vec<T>, width: usize) -> Self {
        let height = vec.len() / width;
        Grid {
            data: vec,
            width,
            height,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    fn get_1d_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
    fn get_2d_index(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    pub fn row(&self, row_index: usize) -> &[T] {
        &self.data[(row_index * self.width)..((row_index + 1) * self.width)]
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|row_index| self.row(row_index))
    }

    pub fn windows<'a>(&'a self, shape: (usize, usize)) -> impl Iterator<Item = GridView<'a, T>> {
        (0..=(self.height - shape.0)).flat_map(move |row_idx| {
            (0..=(self.width - shape.1))
                .map(move |col_idx| GridView::new(self, (row_idx, col_idx), shape))
        })
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn defaults(size: (usize, usize)) -> Self {
        Self {
            data: vec![T::default(); size.0 * size.1],
            width: size.1,
            height: size.0,
        }
    }
}

impl<T: Eq> Grid<T> {
    pub fn position(&self, value: &T) -> Option<(usize, usize)> {
        self.data
            .iter()
            .position(|v| v == value)
            .map(|idx| self.get_2d_index(idx))
    }

    pub fn positions(&self, value: &T) -> impl Iterator<Item = (usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter(move |(_, v)| v == &value)
            .map(|(i, _)| self.get_2d_index(i))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data.index(self.get_1d_index(index.0, index.1))
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(self.get_1d_index(index.0, index.1))
    }
}

pub struct GridView<'a, T> {
    grid: &'a Grid<T>,
    pub index: (usize, usize),
    pub shape: (usize, usize),
}
impl<'a, T> GridView<'a, T> {
    pub fn new(grid: &'a Grid<T>, index: (usize, usize), shape: (usize, usize)) -> Self {
        GridView { grid, index, shape }
    }
}

impl<'a, T> Index<(usize, usize)> for GridView<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid
            .index((index.0 + self.index.0, index.1 + self.index.1))
    }
}
