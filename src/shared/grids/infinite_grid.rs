use std::ops::Index;

use super::{GridIter, Neighbors};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InfiniteRow<T>(Vec<T>);

impl<T> Index<isize> for InfiniteRow<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let row_index = index
            .rem_euclid(self.0.len().try_into().expect("Row too long"))
            .unsigned_abs();

        &self.0[row_index]
    }
}

impl<T> Index<usize> for InfiniteRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub struct InfiniteGrid<T> {
    data: Vec<InfiniteRow<T>>,
    row_len: usize,
    column_len: usize,
    // max_row: usize,
    // max_column: usize,
}

impl<T> InfiniteGrid<T> {
    /// Creates a grid that repeats infinitely in each direction repeating on the data
    ///
    /// # Panics
    /// When rows are not equal length
    #[must_use]
    pub fn new(data: Vec<Vec<T>>) -> Self {
        for w in data.windows(2) {
            assert_eq!(w[0].len(), w[1].len());
        }

        let rows = data.len();

        let columns = data[0].len();

        Self {
            data: data.into_iter().map(|r| InfiniteRow(r)).collect(),
            row_len: rows,
            column_len: columns,
            // max_row: rows - 1,
            // max_column: columns - 1,
        }
    }
}

impl<T> GridIter for InfiniteGrid<T> {
    type GridRow = InfiniteRow<T>;

    fn get_grid(&self) -> &Vec<Self::GridRow> {
        &self.data
    }

    fn get_row_length(&self) -> usize {
        self.row_len
    }

    fn get_column_length(&self) -> usize {
        self.column_len
    }
}

impl<T> Neighbors for InfiniteGrid<T> {
    type Index = isize;

    fn neighbors(
        &self,
        (row_index, column_index): (Self::Index, Self::Index),
    ) -> Vec<(Self::Index, Self::Index)> {
        vec![
            (row_index - 1, column_index),
            (row_index, column_index + 1),
            (row_index, column_index - 1),
            (row_index + 1, column_index),
        ]
    }
}

impl<T> std::fmt::Display for InfiniteGrid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for t in &row.0 {
                write!(f, "{}", t)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> std::fmt::Debug for InfiniteGrid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rows: {}, Columns: {}", self.row_len, self.column_len)?;
        for row in &self.data {
            for t in &row.0 {
                write!(f, "{:?}", t)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Index<usize> for InfiniteGrid<T> {
    type Output = InfiniteRow<T>;

    fn index(&self, index: usize) -> &Self::Output {
        let row_index = index.rem_euclid(self.row_len);

        &self.data[row_index]
    }
}

impl<T> Index<isize> for InfiniteGrid<T> {
    type Output = InfiniteRow<T>;

    fn index(&self, index: isize) -> &Self::Output {
        let row_index = index
            .rem_euclid(self.row_len.try_into().expect("row_count too large"))
            .unsigned_abs();

        &self.data[row_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::grids::infinite_grid::InfiniteGrid;

    #[test]
    fn test_infinite_grid() {
        let g = InfiniteGrid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        assert_eq!('b', g[-9isize][-5isize]);

        assert_eq!('i', g[8isize][8isize]);
    }
}
