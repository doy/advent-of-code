use rayon::iter::{
    IndexedParallelIterator as _, IntoParallelIterator as _,
    IntoParallelRefIterator as _, ParallelIterator as _,
};

macro_rules! impl_op {
    ($ty:ident, $inner:ident, $other:ident, $op_class:ident, $op_method:ident) => {
        impl std::ops::$op_class<$inner> for $ty {
            type Output = Self;
            fn $op_method(self, other: $inner) -> Self::Output {
                Self(self.0.$op_method(other))
            }
        }
        impl std::ops::$op_class<$ty> for $inner {
            type Output = $ty;
            fn $op_method(self, other: $ty) -> Self::Output {
                $ty(self.$op_method(other.0))
            }
        }
        impl std::ops::$op_class<$ty> for $ty {
            type Output = $ty;
            fn $op_method(self, other: $ty) -> Self::Output {
                $ty(self.0.$op_method(other.0))
            }
        }
        impl std::ops::$op_class<$other> for $ty {
            type Output = Self;
            fn $op_method(self, other: $other) -> Self::Output {
                Self(self.0.$op_method($inner::try_from(other).unwrap()))
            }
        }
        impl std::ops::$op_class<$ty> for $other {
            type Output = $ty;
            fn $op_method(self, other: $ty) -> Self::Output {
                $ty($inner::try_from(self).unwrap().$op_method(other.0))
            }
        }
        impl std::ops::$op_class<i32> for $ty {
            type Output = Self;
            fn $op_method(self, other: i32) -> Self::Output {
                Self(self.0.$op_method($inner::try_from(other).unwrap()))
            }
        }
        impl std::ops::$op_class<$ty> for i32 {
            type Output = $ty;
            fn $op_method(self, other: $ty) -> Self::Output {
                $ty($inner::try_from(self).unwrap().$op_method(other.0))
            }
        }
    };
}

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

impl_op!(Row, usize, isize, Add, add);
impl_op!(Row, usize, isize, Sub, sub);
impl_op!(Row, usize, isize, Mul, mul);
impl_op!(Row, usize, isize, Rem, rem);
impl_op!(Col, usize, isize, Add, add);
impl_op!(Col, usize, isize, Sub, sub);
impl_op!(Col, usize, isize, Mul, mul);
impl_op!(Col, usize, isize, Rem, rem);

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

impl_op!(IRow, isize, usize, Add, add);
impl_op!(IRow, isize, usize, Sub, sub);
impl_op!(IRow, isize, usize, Mul, mul);
impl_op!(IRow, isize, usize, Rem, rem);
impl_op!(ICol, isize, usize, Add, add);
impl_op!(ICol, isize, usize, Sub, sub);
impl_op!(ICol, isize, usize, Mul, mul);
impl_op!(ICol, isize, usize, Rem, rem);

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
    pub fn size(&self) -> (Row, Col) {
        (self.rows(), self.cols())
    }

    pub fn unshift_rows(&mut self, count: usize) {
        self.rows = self.rows.split_off(count);
    }

    pub fn rows(&self) -> Row {
        Row(self.rows.len())
    }

    pub fn each_row(
        &self,
    ) -> impl DoubleEndedIterator<Item = Row> + ExactSizeIterator {
        (0..self.rows().0).map(Row)
    }

    pub fn par_each_row(
        &self,
    ) -> impl rayon::iter::ParallelIterator<Item = Row> {
        (0..self.rows().0).into_par_iter().map(Row)
    }

    pub fn row_vec(&self, row: Row) -> Vec<T> {
        self.rows[row.0].cells.clone()
    }

    pub fn cols(&self) -> Col {
        Col(self.rows[0].cells.len())
    }

    pub fn each_col(
        &self,
    ) -> impl DoubleEndedIterator<Item = Col> + ExactSizeIterator {
        (0..self.cols().0).map(Col)
    }

    pub fn par_each_col(
        &self,
    ) -> impl rayon::iter::ParallelIterator<Item = Col> {
        (0..self.cols().0).into_par_iter().map(Col)
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

    pub fn par_cells(&self) -> impl rayon::iter::ParallelIterator<Item = &T>
    where
        T: Sync,
    {
        self.rows.par_iter().flat_map(|row| row.cells.par_iter())
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

    pub fn par_indexed_cells(
        &self,
    ) -> impl rayon::iter::ParallelIterator<Item = ((Row, Col), &T)>
    where
        T: Sync,
    {
        self.rows.par_iter().enumerate().flat_map(|(i, row)| {
            row.cells
                .par_iter()
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

    pub fn flood_fill(
        &mut self,
        row: Row,
        col: Col,
        fill: &T,
        diagonal: bool,
    ) {
        let mut todo = vec![(row, col)];
        while let Some((row, col)) = todo.pop() {
            self[row][col] = fill.clone();
            for (row, col) in self.adjacent(row, col, diagonal) {
                if self[row][col] != *fill {
                    todo.push((row, col));
                }
            }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Self::Up),
            "v" => Ok(Self::Down),
            "<" => Ok(Self::Left),
            ">" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl std::convert::TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl std::convert::TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'^' => Ok(Self::Up),
            b'v' => Ok(Self::Down),
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            b'U' => Ok(Self::Up),
            b'D' => Ok(Self::Down),
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn from_pos(row: Row, col: Col, new_row: Row, new_col: Col) -> Self {
        if row.abs_diff(new_row) == Row(1) && col.abs_diff(new_col) == Col(0)
        {
            if row > new_row {
                Self::Up
            } else {
                Self::Down
            }
        } else if col.abs_diff(new_col) == Col(1)
            && row.abs_diff(new_row) == Row(0)
        {
            if col > new_col {
                Self::Left
            } else {
                Self::Right
            }
        } else {
            panic!("invalid direction ({row:?}, {col:?}) -> ({new_row:?}, {new_col:?})")
        }
    }

    pub fn move_checked(
        self,
        pos: (Row, Col),
        size: (Row, Col),
    ) -> Option<(Row, Col)> {
        match self {
            Self::Up => pos.0 .0.checked_sub(1).map(|row| (Row(row), pos.1)),
            Self::Down => {
                if pos.0 .0 >= size.0 .0 - 1 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            Self::Left => {
                pos.1 .0.checked_sub(1).map(|col| (pos.0, Col(col)))
            }
            Self::Right => {
                if pos.1 .0 >= size.1 .0 - 1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
        }
    }

    pub fn move_wrapped(
        self,
        pos: (Row, Col),
        size: (Row, Col),
    ) -> (Row, Col) {
        match self {
            Self::Up => ((size.0 .0 + pos.0 - 1) % size.0 .0, pos.1),
            Self::Down => ((pos.0 + 1) % size.0 .0, pos.1),
            Self::Left => (pos.0, (size.1 .0 + pos.1 - 1) % size.1 .0),
            Self::Right => (pos.0, (pos.1 + 1) % size.1 .0),
        }
    }

    pub fn horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn increasing(&self) -> bool {
        matches!(self, Self::Down | Self::Right)
    }

    pub fn turns(&self) -> [Self; 2] {
        match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn offset(&self) -> (IRow, ICol) {
        match self {
            Self::Up => (IRow(-1), ICol(0)),
            Self::Down => (IRow(1), ICol(0)),
            Self::Left => (IRow(0), ICol(-1)),
            Self::Right => (IRow(0), ICol(1)),
        }
    }
}
