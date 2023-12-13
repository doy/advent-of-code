#[derive(
    Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Default,
)]
pub struct Row(pub usize);

#[derive(
    Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Default,
)]
pub struct Col(pub usize);

impl Row {
    pub fn abs_diff(self, other: Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }
}

impl Col {
    pub fn abs_diff(self, other: Self) -> Self {
        Self(self.0.abs_diff(other.0))
    }
}

impl std::ops::Add<usize> for Row {
    type Output = Self;
    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl std::ops::Add<Row> for usize {
    type Output = Row;
    fn add(self, other: Row) -> Self::Output {
        Row(self + other.0)
    }
}

impl std::ops::Add<usize> for Col {
    type Output = Self;
    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl std::ops::Add<Col> for usize {
    type Output = Col;
    fn add(self, other: Col) -> Self::Output {
        Col(self + other.0)
    }
}

impl std::ops::Sub<usize> for Row {
    type Output = Self;
    fn sub(self, other: usize) -> Self::Output {
        Self(self.0 - other)
    }
}

impl std::ops::Sub<Row> for usize {
    type Output = Row;
    fn sub(self, other: Row) -> Self::Output {
        Row(self - other.0)
    }
}

impl std::ops::Sub<usize> for Col {
    type Output = Self;
    fn sub(self, other: usize) -> Self::Output {
        Self(self.0 - other)
    }
}

impl std::ops::Sub<Col> for usize {
    type Output = Col;
    fn sub(self, other: Col) -> Self::Output {
        Col(self - other.0)
    }
}

impl std::ops::Mul<usize> for Row {
    type Output = Self;
    fn mul(self, other: usize) -> Self::Output {
        Self(self.0 * other)
    }
}

impl std::ops::Mul<Row> for usize {
    type Output = Row;
    fn mul(self, other: Row) -> Self::Output {
        Row(self * other.0)
    }
}

impl std::ops::Mul<usize> for Col {
    type Output = Self;
    fn mul(self, other: usize) -> Self::Output {
        Self(self.0 * other)
    }
}

impl std::ops::Mul<Col> for usize {
    type Output = Col;
    fn mul(self, other: Col) -> Self::Output {
        Col(self * other.0)
    }
}

impl std::ops::Rem<usize> for Row {
    type Output = Self;
    fn rem(self, other: usize) -> Self::Output {
        Self(self.0 % other)
    }
}

impl std::ops::Rem<Row> for usize {
    type Output = Row;
    fn rem(self, other: Row) -> Self::Output {
        Row(self % other.0)
    }
}

impl std::ops::Rem<usize> for Col {
    type Output = Self;
    fn rem(self, other: usize) -> Self::Output {
        Self(self.0 % other)
    }
}

impl std::ops::Rem<Col> for usize {
    type Output = Col;
    fn rem(self, other: Col) -> Self::Output {
        Col(self % other.0)
    }
}

#[derive(
    Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Default,
)]
pub struct IRow(pub isize);

#[derive(
    Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Default,
)]
pub struct ICol(pub isize);

impl IRow {
    pub fn abs_diff(self, other: Self) -> Row {
        Row(self.0.abs_diff(other.0))
    }
}

impl ICol {
    pub fn abs_diff(self, other: Self) -> Col {
        Col(self.0.abs_diff(other.0))
    }
}

impl std::ops::Add<isize> for IRow {
    type Output = Self;
    fn add(self, other: isize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl std::ops::Add<IRow> for isize {
    type Output = IRow;
    fn add(self, other: IRow) -> Self::Output {
        IRow(self + other.0)
    }
}

impl std::ops::Add<isize> for ICol {
    type Output = Self;
    fn add(self, other: isize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl std::ops::Add<ICol> for isize {
    type Output = ICol;
    fn add(self, other: ICol) -> Self::Output {
        ICol(self + other.0)
    }
}

impl std::ops::Sub<isize> for IRow {
    type Output = Self;
    fn sub(self, other: isize) -> Self::Output {
        Self(self.0 - other)
    }
}

impl std::ops::Sub<IRow> for isize {
    type Output = IRow;
    fn sub(self, other: IRow) -> Self::Output {
        IRow(self - other.0)
    }
}

impl std::ops::Sub<isize> for ICol {
    type Output = Self;
    fn sub(self, other: isize) -> Self::Output {
        Self(self.0 - other)
    }
}

impl std::ops::Sub<ICol> for isize {
    type Output = ICol;
    fn sub(self, other: ICol) -> Self::Output {
        ICol(self - other.0)
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridRow<T: Clone + Eq + PartialEq + std::hash::Hash> {
    cells: Vec<T>,
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> GridRow<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> + Clone {
        self.cells.iter()
    }

    pub fn get(&self, col: Col) -> Option<&T> {
        self.cells.get(col.0)
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> std::ops::Index<Col>
    for GridRow<T>
{
    type Output = T;
    fn index(&self, col: Col) -> &Self::Output {
        &self.cells[col.0]
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> std::ops::IndexMut<Col>
    for GridRow<T>
{
    fn index_mut(&mut self, col: Col) -> &mut Self::Output {
        &mut self.cells[col.0]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grid<T: Clone + Eq + PartialEq + std::hash::Hash> {
    rows: Vec<GridRow<T>>,
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> Default for Grid<T> {
    fn default() -> Self {
        Self { rows: vec![] }
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> Grid<T> {
    pub fn unshift_rows(&mut self, count: usize) {
        self.rows = self.rows.split_off(count);
    }

    pub fn rows(&self) -> Row {
        Row(self.rows.len())
    }

    pub fn each_row(
        &self,
    ) -> impl Iterator<Item = Row> + DoubleEndedIterator + ExactSizeIterator
    {
        (0..self.rows().0).map(Row)
    }

    pub fn row_vec(&self, row: Row) -> Vec<T> {
        self.rows[row.0].cells.clone()
    }

    pub fn cols(&self) -> Col {
        Col(self.rows[0].cells.len())
    }

    pub fn each_col(
        &self,
    ) -> impl Iterator<Item = Col> + DoubleEndedIterator + ExactSizeIterator
    {
        (0..self.cols().0).map(Col)
    }

    pub fn col_vec(&self, col: Col) -> Vec<T> {
        self.rows.iter().map(|row| row[col].clone()).collect()
    }

    pub fn get(&self, row: Row) -> Option<&GridRow<T>> {
        self.rows.get(row.0)
    }

    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.rows.iter().flat_map(|row| row.cells.iter())
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.rows.iter_mut().flat_map(|row| row.cells.iter_mut())
    }

    pub fn indexed_cells(&self) -> impl Iterator<Item = ((Row, Col), &T)> {
        self.rows.iter().enumerate().flat_map(|(i, row)| {
            row.cells
                .iter()
                .enumerate()
                .map(move |(j, cell)| ((Row(i), Col(j)), cell))
        })
    }

    pub fn indexed_cells_mut(
        &mut self,
    ) -> impl Iterator<Item = ((Row, Col), &mut T)> {
        self.rows.iter_mut().enumerate().flat_map(|(i, row)| {
            row.cells
                .iter_mut()
                .enumerate()
                .map(move |(j, cell)| ((Row(i), Col(j)), cell))
        })
    }

    pub fn adjacent(&self, row: Row, col: Col, diagonal: bool) -> Adjacent {
        Adjacent {
            row: row.0,
            col: col.0,
            rows: self.rows().0,
            cols: self.cols().0,
            diagonal,
            pos: 0,
        }
    }
}

impl<T: Default + Clone + Eq + PartialEq + std::hash::Hash> Grid<T> {
    pub fn grow(&mut self, rows: Row, cols: Col) {
        self.rows
            .resize_with(rows.0.max(self.rows.len()), GridRow::default);
        for row in &mut self.rows {
            row.cells
                .resize_with(cols.0.max(row.cells.len()), T::default);
        }
    }

    pub fn insert_row(&mut self, row: Row) {
        let mut cells = vec![];
        cells.resize_with(self.cols().0, Default::default);
        self.rows.insert(row.0, GridRow { cells });
    }

    pub fn insert_col(&mut self, col: Col) {
        for row in self.each_row() {
            let row = &mut self[row];
            row.cells.insert(col.0, Default::default());
        }
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash + std::fmt::Display>
    Grid<T>
{
    pub fn display_packed<F: Fn(&T) -> char>(
        &self,
        f: F,
    ) -> DisplayPacked<T, F> {
        DisplayPacked(self, f)
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash + std::fmt::Display>
    std::fmt::Display for Grid<T>
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            for col in &row.cells {
                write!(f, "{} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> std::ops::Index<Row>
    for Grid<T>
{
    type Output = GridRow<T>;
    fn index(&self, row: Row) -> &Self::Output {
        &self.rows[row.0]
    }
}

impl<T: Clone + Eq + PartialEq + std::hash::Hash> std::ops::IndexMut<Row>
    for Grid<T>
{
    fn index_mut(&mut self, row: Row) -> &mut Self::Output {
        &mut self.rows[row.0]
    }
}

impl<T: Default + Clone + Eq + PartialEq + std::hash::Hash>
    FromIterator<Vec<T>> for Grid<T>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Vec<T>>,
    {
        let mut self_ = Self {
            rows: iter.into_iter().map(|v| GridRow { cells: v }).collect(),
        };
        let nrows = self_.rows.len();
        let ncols = self_
            .rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        self_.grow(Row(nrows), Col(ncols));

        self_
    }
}

impl<T: Default + Clone + Eq + PartialEq + std::hash::Hash>
    FromIterator<((Row, Col), T)> for Grid<T>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = ((Row, Col), T)>,
    {
        let mut self_ = Self::default();
        for ((row, col), cell) in iter {
            self_.grow(Row(row.0 + 1), Col(col.0 + 1));
            self_[row][col] = cell;
        }
        self_
    }
}

pub struct DisplayPacked<
    'a,
    T: Clone + Eq + PartialEq + std::hash::Hash + std::fmt::Display,
    F: Fn(&'a T) -> char,
>(&'a Grid<T>, F);

impl<
        'a,
        T: Default
            + Clone
            + Eq
            + PartialEq
            + std::hash::Hash
            + std::fmt::Display,
        F: Fn(&'a T) -> char,
    > std::fmt::Display for DisplayPacked<'a, T, F>
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        for row in &self.0.rows {
            for col in &row.cells {
                write!(f, "{}", self.1(col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Adjacent {
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    diagonal: bool,
    pos: u8,
}

impl Iterator for Adjacent {
    type Item = (Row, Col);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= 9 {
                return None;
            }
            let pos_row = self.pos / 3;
            let pos_col = self.pos - pos_row * 3;
            self.pos += 1;
            if pos_row == 0 && self.row == 0
                || pos_col == 0 && self.col == 0
                || pos_row == 2 && self.row == self.rows - 1
                || pos_col == 2 && self.col == self.cols - 1
                || pos_row == 1 && pos_col == 1
                || (!self.diagonal
                    && ((pos_row == pos_col)
                        || (pos_row == 2 && pos_col == 0)
                        || (pos_row == 0 && pos_col == 2)))
            {
                continue;
            }
            return Some((
                Row(self.row + usize::from(pos_row) - 1),
                Col(self.col + usize::from(pos_col) - 1),
            ));
        }
    }
}
