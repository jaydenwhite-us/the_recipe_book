#[test]
fn new_matrix() {
    use matrix::Matrix;
    let matrix = Matrix::new(2, 2);
    let assertion = vec![vec![0.0; 2]; 2];
    assert_eq!(*matrix.values(), assertion);
    assert_eq!(matrix.columns(), 2);
    assert_eq!(matrix.rows(), 2);
}
#[test]
fn square_matrix() {
    use matrix::Matrix;
    let matrix = Matrix::square(2);
    let assertion = vec![vec![0.0; 2]; 2];
    assert_eq!(*matrix.values(), assertion);
    assert_eq!(matrix.columns(), 2);
    assert_eq!(matrix.rows(), 2);
}
#[test]
fn identity_matrix() {
    use matrix::Matrix;
    let identity = Matrix::identity(3);
    assert_eq!(
        *identity.values(),
        vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0]
        ]
    );
}
#[test]
fn from_vector() {
    use matrix::Matrix;
    let matrix = Matrix::from(vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ]);
    assert_eq!(matrix.columns(), 3);
    assert_eq!(matrix.rows(), 4);
}

#[test]
fn largest_in_column() {
    use matrix::Matrix;
    let mut matrix = Matrix::square(2);
    let target_col = 0;
    matrix.values[1][target_col] = 2.0;

    let (largest, (x, y)) = matrix.column_max(target_col);
    assert_eq!(largest, 2.0);
    assert_eq!((1, 0), (x, y));
}
#[test]
fn largest_in_row() {
    use matrix::Matrix;
    let mut matrix = Matrix::square(2);
    let target_row = 1;
    matrix.values[target_row][1] = 2.0;
    let (largest, (x, y)) = matrix.row_max(target_row);
    assert_eq!(largest, 2.0);
    assert_eq!((1, 1), (x, y));
}

#[test]
fn swap_rows() {
    use matrix::Matrix;
    let mut matrix = Matrix::from(vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ]);
    matrix.swap_rows((0, 1));
    matrix.swap_rows((1, 2));
    let assertion_matrix = Matrix::from(vec![
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
        vec![2.0, 1.0, -1.0],
    ]);
    assert_eq!(matrix, assertion_matrix);
}

#[test]
fn scale() {}

#[test]
fn swap_columns() {
    use matrix::Matrix;
    let mut matrix = Matrix::from(vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ]);
    matrix.swap_columns((0, 1));
    let assertion_matrix = Matrix::from(vec![
        vec![1.0, 2.0, -1.0],
        vec![-1.0, -3.0, 2.0],
        vec![1.0, -2.0, 2.0],
    ]);
    assert_eq!(matrix, assertion_matrix);
}

#[test]
fn return_valid_row_length() {
    use matrix::Matrix;
    let mut matrix = Matrix::from(vec![
        vec![2.0, 1.0, -1.0],
        vec![-3.0, -1.0, 2.0],
        vec![-2.0, 1.0, 2.0],
    ]);
    matrix.values[0].push(1.0);

    assert_eq!(matrix.columns(), 4);
}
