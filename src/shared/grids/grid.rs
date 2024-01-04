use std::ops::Index;

use super::{GridIter, Neighbors};

pub struct Grid<T> {
    data: Vec<Row<T>>,
    row_len: usize,
    column_len: usize,
    // max_row: usize,
    // max_column: usize,
}

impl<T> Grid<T> {
    /// Builds a new grid
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
            data: data.into_iter().map(|r| Row(r)).collect(),
            row_len: rows,
            column_len: columns,
            // max_row: rows - 1,
            // max_column: columns - 1,
        }
    }

    pub fn find(&self, value: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for ((row_index, column_index), v) in self.row_column_index_value_iter() {
            if v == value {
                return Some((row_index, column_index));
            }
        }

        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Row<T>(Vec<T>);

impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> GridIter for Grid<T> {
    type GridRow = Row<T>;

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

impl<T> Neighbors for Grid<T> {
    type Index = usize;

    fn neighbors(
        &self,
        (row_index, column_index): (Self::Index, Self::Index),
    ) -> Vec<(Self::Index, Self::Index)> {
        let mut neighbors = vec![];

        if row_index > 0 {
            neighbors.push((row_index - 1, column_index));
        }

        if column_index + 1 < self.column_len {
            neighbors.push((row_index, column_index + 1));
        }

        if column_index > 0 {
            neighbors.push((row_index, column_index - 1));
        }

        if row_index + 1 < self.row_len {
            neighbors.push((row_index + 1, column_index));
        }

        neighbors
    }
}

impl<T> std::fmt::Display for Grid<T>
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

impl<T> std::fmt::Debug for Grid<T>
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

impl<T> Index<usize> for Grid<T> {
    type Output = Row<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::grids::GridIter;

    use super::Grid;

    #[test]
    fn test_rows() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let rows = g.data.iter().collect::<Vec<_>>();

        assert_eq!(rows, g.row_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_columns() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let columns = vec![
            vec![&'a', &'d', &'g'],
            vec![&'b', &'e', &'h'],
            vec![&'c', &'f', &'i'],
        ];

        let transposed = g.column_iter().collect::<Vec<_>>();

        assert_eq!(columns, transposed);
    }

    #[test]

    fn test_row_columns_iter() {
        let g = Grid::new(vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);

        let v = vec![
            ((0, 0), &'a'),
            ((0, 1), &'b'),
            ((0, 2), &'c'),
            ((1, 0), &'d'),
            ((1, 1), &'e'),
            ((1, 2), &'f'),
            ((2, 0), &'g'),
            ((2, 1), &'h'),
            ((2, 2), &'i'),
        ];

        assert_eq!(v, g.row_column_index_value_iter().collect::<Vec<_>>());
    }
}
