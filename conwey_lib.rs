#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![T::default(); rows * cols];
        Self { rows, cols, grid }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert_eq!(grid.len(), rows * cols);
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let neighbours = vec![
            (row.wrapping_sub(1), col.wrapping_sub(1)),
            (row.wrapping_sub(1), col),
            (row.wrapping_sub(1), col.wrapping_add(1)),
            (row, col.wrapping_sub(1)),
            (row, col.wrapping_add(1)),
            (row.wrapping_add(1), col.wrapping_sub(1)),
            (row.wrapping_add(1), col),
            (row.wrapping_add(1), col.wrapping_add(1)),
        ];
        neighbours
            .into_iter()
            .filter(|&(cell_row, cell_col)| cell_row < self.rows && cell_col < self.cols)
            .collect()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut next = Grid::new(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
                let cell = *self.grid.get(r, c);
                let alive_neighbors = self
                    .grid
                    .neighbours(r, c)
                    .iter()
                    .filter(|&&(nr, nc)| *self.grid.get(nr, nc) == Cell::Alive)
                    .count();
                match (cell, alive_neighbors) {
                    (Cell::Alive, 2 | 3) => next.set(Cell::Alive, r, c),
                    (Cell::Dead, 3) => next.set(Cell::Alive, r, c),
                    _ => next.set(Cell::Dead, r, c),
                };
            }
        }
        self.grid = next;
    }
}
