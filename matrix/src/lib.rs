///A nested vector of type f32.
/// ### Arguments
///
#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub values: Vec<Vec<f32>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn from(values: Vec<Vec<f32>>) -> Self {
        let number_of_rows = values.len();
        let number_of_elements = values[0].len();
        for row in &values {
            assert_eq!(row.len(), number_of_elements, "Rows have differing lengths");
        }
        Matrix {
            values,
            rows: number_of_rows,
            columns: number_of_elements,
        }
    }

    pub fn new(rows: usize, columns: usize) -> Self {
        Matrix {
            values: vec![vec![0.0; columns.into()]; rows.into()],
            rows,
            columns,
        }
    }
    pub fn values(&self) -> &Vec<Vec<f32>> {
        &self.values
    }

    pub fn square(size: usize) -> Self {
        Matrix {
            values: vec![vec![0.0; size.into()]; size.into()],
            rows: size,
            columns: size,
        }
    }
    pub fn identity(size: usize) -> Self {
        let mut matrix = Matrix::square(size);
        for x in 0..matrix.rows() {
            matrix.values[x][x] = 1.0;
        }
        return matrix;
    }
    /// Returns the number of rows.
    ///
    /// Especially useful when working with square matrices.
    ///
    pub fn rows(&self) -> usize {
        self.values.len()
    } //(O(1))

    /// Returns the length of the largest row as the number of columns. `Use with caution.`
    ///
    /// # The Warning
    /// The value returned from columns may not be valid for every row.
    /// Rows in a Matrix are allowed to differ in length, leading to a mangled Matrix.
    /// Attempting to assign to an unallocated location in a Matrix will result in an out of bounds exception.
    ///
    /// Complexity: O(n)
    /// Where complexity is a concern, see columns_unchecked()
    pub fn columns(&self) -> usize {
        let mut longest_row: usize = 0;
        for row in 0..self.values().len() {
            if self.values[row].len() > longest_row {
                longest_row = self.values().len();
            }
        }
        longest_row
    }

    /// Returns the unchecked value of columns.
    ///
    /// # The Warning:
    /// The value returned from columns may not be valid for every row.
    /// Rows in a Matrix are allowed to differ in length, leading to a mangled Matrix.
    /// Meaning it is possible for a row length to be shorter or longer than the value returned from columns_unchecked().
    /// Attempting to assign to an unallocated location in a Matrix will result in an out of bounds exception.
    ///
    /// Complexity: O(1)
    pub fn columns_unchecked(&self) -> usize {
        self.columns
    }

    /// Returns true if the rows of the Matrix are different lengths.
    pub fn is_mangled(&self) -> bool {
        let longest = self.values[0].len();
        self.values.iter().all(|row| row.len() == longest)
    }

    /// Returns maximum value in a column.
    pub fn column_max(&self, column: usize) -> (f32, (usize, usize)) {
        let mut row_of_largest: usize = 0;
        //Search specified column for index of largest.
        for row in 0..self.rows() {
            if self.values[row][column].abs() <= self.values[row_of_largest][column].abs() {
                continue;
            }
            row_of_largest = row;
        }
        //Return tuple
        (
            self.values[row_of_largest][column],
            (row_of_largest, column),
        )
    }

    /// Returns maximum value in a row.
    pub fn row_max(&self, row: usize) -> (f32, (usize, usize)) {
        let mut col_of_largest: usize = 0;
        //Search specified column for index of largest.
        for col in 0..self.columns() {
            if self.values[row][col].abs() <= self.values[row][col_of_largest].abs() {
                continue;
            }
            col_of_largest = col;
        }
        //Return tuple
        (self.values[row][col_of_largest], (row, col_of_largest))
    }

    /// Takes a tuple (usize, usize) and attempts swap rows in the Matrix. Will panic if index is out of bounds.
    pub fn swap_rows(&mut self, rows: (usize, usize)) {
        for col in 0..self.columns() {
            let temp = self.values[rows.0][col];
            self.values[rows.0][col] = self.values[rows.1][col];
            self.values[rows.1][col] = temp;
        }
    }

    ///Multiply all elements in a row by some scalar value.
    pub fn scale_row(&mut self, row: usize, scale: f32) {
        for element in &mut self.values[row][0..] {
            *element *= scale;
        }
    }
    /// Add some addend to all elements in a row.
    pub fn add_to_row(&mut self, row: usize, addend: f32) {
        for element in &mut self.values[row][0..] {
            *element += addend;
        }
    }
    /// Takes a tuple (usize, usize) and attempts swap columns in the Matrix. Will panic if index is out of bounds.
    pub fn swap_columns(&mut self, columns: (usize, usize)) {
        for row in &mut self.values {
            let temp = row[columns.0];
            row[columns.0] = row[columns.1];
            row[columns.1] = temp;
        }
    }
    ///Appends a row to the Matrix
    pub fn new_row(&mut self) {
        self.values.push(vec![0.0; self.columns.into()]);
        self.rows = self.rows + 1;
    }
}
