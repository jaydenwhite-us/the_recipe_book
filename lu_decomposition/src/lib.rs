use matrix;
#[derive(Debug)]
pub enum Error {
    SingularMatrix,
}

///Performs LU decomposition in place on a matrix A
///
///### Usage
/// To be used with Crouts algorithm when solving right hand side columns in series or parellel.
/// Returns the parity for use in calculating the determinant as an isize,
/// and the permutation of the matrix as a vector.
/// ```
/// //Calculate the LU Decomposition
///    use lu_decomposition::{decompose, crout};
///    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
///    let solution = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![2.0, 8.0]]);
///    let result = decompose(&mut matrix);
///    assert!(result.is_ok());
///    assert_eq!(matrix, solution);
/// ```
pub fn decompose(a: &mut crate::matrix::Matrix) -> Result<(Vec<usize>, isize), Error> {
    let mut scalars = vec![1.0; a.values.len()];
    let mut operations: Vec<usize> = vec![0; a.values.len()];
    let mut parity = 1;

    //Record implicit scaling
    for row in 0..a.rows() {
        let max = a.row_abs_max(row).0;
        if max == 0.0 {
            return Err(Error::SingularMatrix);
        }
        scalars[row] /= max;
    }

    assert!(scalars.len() == a.rows());
    assert!(!a.is_mangled(), "Input matrix is mangled. Cannot Solve.");

    //Crout's algorithm
    //2.3.12
    for column in 0..a.columns() {
        let (mut scaled_column_max, mut row_of_max): (f32, usize) = (0.00, column);

        'summations: for row in 0..a.columns() {
            let mut sum = a.values[row][column];

            //Summation: According to figures 2.3.8, 2.3.9, and 2.3.10 pg 46, The end of the loop is minimum of row and column.
            for k in 0..row.min(column) {
                sum -= a.values[row][k] * a.values[k][column];
            }
            a.values[row][column] = sum;

            //Row-only Pivot Tracking: Track largest alpha on or below the diagonal.
            let above_diagonal = row < column;
            if above_diagonal {
                continue 'summations;
            } else {
                let scaled_sum = scalars[row] * sum.abs();
                if scaled_sum > scaled_column_max {
                    (scaled_column_max, row_of_max) = (scaled_sum, row);
                }
            }
        }

        //Pivot if necessary.
        let on_diagonal = column == row_of_max;
        if !on_diagonal {
            a.swap_rows((row_of_max, column));
            scalars.swap(row_of_max, column);
            parity *= -1; //Flip parity for odd or even number of swaps.
        }
        operations[column] = row_of_max;

        //??TINY
        let pivot_element = a.values[column][column];
        if pivot_element == 0.0 {
            a.values[column][column] = 1.0f32.powi(-40);
            /*If the pivot element is zero the matrix is singular (at least to the precision
                of the algorithm). For some applications on singular matrices, it is desirable to substitute TINY(an extremely small number) for zero.
            */
        }
        //Divide alphas (below the diagonal) by the pivot_element.
        '_scale_column: for row in column + 1..a.columns() {
            a.values[row][column] *= 1.0 / a.values[column][column]
        }
    }
    Ok((operations, parity))
}

///Performs Crouts algorithm on an LU decomposed matrix, A,  and a right hand side matrix B.
///### Arguments
///     a: a left hand side decomposed matrix.
///     permutation: The row-wise permutation of the variable a as a vector
///     b: right hand side matrix b.
///### Usage
/// Returns in place of `b` a solution vector that is not permutated.
/// Does not check for compatibility, or mangled matrix. Panics on out of bounds.
/// ```
/// //Calculate the inverse
///    use lu_decomposition::{decompose, crout};
///    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
///    let mut matrix_b = matrix::Matrix::identity(2);
///    let solution = matrix::Matrix::from(vec![
///        vec![4.0 / 24.0, 2.0 / 24.0],
///        vec![-6.0 / 24.0, 3.0 / 24.0],
///    ]);
///    let result = decompose(&mut matrix);
///    assert!(result.is_ok());
///    let (permutation, ..) = result.unwrap();
///    crout(&matrix, &permutation, &mut matrix_b);
///    assert_eq!(matrix_b, solution);
/// ```
pub fn crout(a: &matrix::Matrix, permutation: &Vec<usize>, b: &mut matrix::Matrix) {
    forward_substitution(a, permutation, b);
    backward_substitution(a, b);
    //Reverse Permutation
    for row in (0..b.rows()).rev() {
        b.swap_rows((row, permutation[row]));
    }
}

///Performs the forward substitution step of Crout\'s algorithm. Returns in place of `b`
///a solution vector that is permutated by A's permutation.
fn forward_substitution(a: &matrix::Matrix, permutation: &Vec<usize>, b: &mut matrix::Matrix) {
    for column in 0..b.columns_unchecked() {
        let mut non_zero_rhs_encountered = false; //Optimization for sparse right hand side with many leading zero's
        for row in 0..a.rows() {
            b.swap_rows((row, permutation[row])); //Permute b like a to line up solutions with the equations they solve.
            if non_zero_rhs_encountered {
                for k in 0..row {
                    b.values[row][column] -= a.values[row][k] * b.values[k][column];
                }
            } else {
                non_zero_rhs_encountered = b.values[row][column] != 0.0;
            }
        }
    }
}

///Performs the backwards substitution step of Crout's algorithm. Returns in place of `b`
///a solution vector that is not permutated. Primarily for testing. Use crout when hoping to apply Crout's.
fn backward_substitution(a: &matrix::Matrix, b: &mut matrix::Matrix) {
    //Allow for multi dimensional matrices solved one column at a time.

    for column in 0..b.columns_unchecked() {
        for row in (0..a.rows()).rev() {
            let mut sum = b.values[row][column];
            for k in row..a.rows() {
                if row == k {
                    continue;
                }
                sum -= a.values[row][k] * b.values[k][column];
            }
            b.values[row][column] = sum / a.values[row][row];
        }
    }
}

#[test]
fn forward_substitution_3x1() {
    let mut matrix = matrix::Matrix::from(vec![
        vec![-12.0, 6.0, 18.0],
        vec![6.0, 4.0, 5.0],
        vec![-48.0, 24.0, 42.0],
    ]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();
    let mut b = matrix::Matrix::from(vec![vec![19.5], vec![6.0], vec![48.0]]);

    let permutated_solution = matrix::Matrix::from(vec![vec![6.0], vec![96.0], vec![7.5]]);
    forward_substitution(&matrix, &permutation, &mut b);
    assert_eq!(b, permutated_solution);
}

#[test]
fn backward_substitution_3x1() {
    let mut matrix = matrix::Matrix::from(vec![
        vec![-12.0, 6.0, 18.0],
        vec![6.0, 4.0, 5.0],
        vec![-48.0, 24.0, 42.0],
    ]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();
    let mut b = matrix::Matrix::from(vec![vec![19.5], vec![6.0], vec![48.0]]);
    let solution = matrix::Matrix::from(vec![vec![0.0], vec![0.25], vec![1.0]]);
    forward_substitution(&matrix, &permutation, &mut b);
    backward_substitution(&matrix, &mut b);
    assert_eq!(b, solution);
}

#[test]
fn forward_substitution_2x1() {
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::from(vec![vec![4.0], vec![-8.0]]);

    let solution = matrix::Matrix::from(vec![vec![4.0], vec![-16.0]]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();

    forward_substitution(&matrix, &permutation, &mut matrix_b);
    assert_eq!(matrix_b, solution);
}

#[test]
fn backward_substitution_2x1() {
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::from(vec![vec![4.0], vec![-8.0]]);
    let solution = matrix::Matrix::from(vec![vec![0.0], vec![-2.0]]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();

    forward_substitution(&matrix, &permutation, &mut matrix_b);
    backward_substitution(&matrix, &mut matrix_b);
    assert_eq!(matrix_b, solution);
}

#[test]
fn forward_substitution_2x2() {
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();
    let mut b = matrix::Matrix::identity(2);

    let permutated_solution = matrix::Matrix::from(vec![vec![1.0, 0.0], vec![-2.0, 1.0]]);
    forward_substitution(&matrix, &permutation, &mut b);
    assert_eq!(b, permutated_solution);
}
#[test]
fn backward_substitution_2x2() {
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();
    let mut b = matrix::Matrix::identity(2);

    let permutated_solution = matrix::Matrix::from(vec![
        vec![1.0 / 6.0, 1.0 / 12.0],
        vec![-1.0 / 4.0, 1.0 / 8.0],
    ]);
    forward_substitution(&matrix, &permutation, &mut b);
    backward_substitution(&matrix, &mut b);
    assert_eq!(b, permutated_solution);
}

#[test]
#[should_panic]
fn forward_substitution_2x2_complex() {
    todo!();
}

#[test]
#[should_panic]
fn backward_substitution_2x2_complex() {
    todo!();
}
