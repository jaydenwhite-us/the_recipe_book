#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub values: Vec<Vec<f32>>,
    rows: usize,
    columns: usize,
    column_first: bool,
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
            column_first: false,
        }
    }
    pub fn new(rows: usize, columns: usize) -> Self {
        Matrix {
            values: vec![vec![0.0; columns.into()]; rows.into()],
            rows,
            columns,
            column_first: false,
        }
    }

    pub fn square(size: usize) -> Self {
        Matrix {
            values: vec![vec![0.0; size.into()]; size.into()],
            rows: size,
            columns: size,
            column_first: false,
        }
    }
    pub fn identity(size: usize) -> Self {
        let mut matrix = Matrix::square(size);
        for x in 0..matrix.rows() {
            matrix.values[x][x] = 1.0;
        }
        return matrix;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

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

    pub fn swap_rows(&mut self, rows: (usize, usize)) {
        for col in 0..self.columns() {
            let temp = self.values[rows.0][col];
            self.values[rows.0][col] = self.values[rows.1][col];
            self.values[rows.1][col] = temp;
        }
    }

    pub fn scale_row(&mut self, row: usize, scale: f32) {
        for element in &mut self.values[row][0..] {
            *element *= scale;
        }
    }
    pub fn add_to_row(&mut self, row: usize, addend: f32) {
        for element in &mut self.values[row][0..] {
            *element -= addend;
        }
    }
    pub fn swap_columns(&mut self, columns: (usize, usize)) {
        for row in &mut self.values {
            let temp = row[columns.0];
            row[columns.0] = row[columns.1];
            row[columns.1] = temp;
        }
    }

    pub fn new_row(&mut self) {
        self.values.push(vec![0.0; self.columns.into()]);
        self.rows = self.rows + 1;
    }

    pub fn new_column(mut self) -> Self {
        for row in &mut self.values {
            row.push(0.0);
        }
        self.columns = self.columns + 1;
        self
    }
}
