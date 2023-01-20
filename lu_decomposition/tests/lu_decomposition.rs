#[test]
fn return_decomposition_in_place_of_a_2x2() {
    use lu_decomposition::decompose;
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![vec![-2.0, 3.0], vec![6.0, 4.0]]);
    let result = decompose(&mut matrix);
    assert!(result.is_ok());
    let (operations, parity) = result.unwrap();
    let solution = matrix::Matrix::from(vec![vec![6.0, 4.0], vec![-1.0 / 3.0, 13.0 / 3.0]]);
    assert_eq!(matrix, solution);
    assert_eq!(parity, -1);
    assert_eq!(operations, vec![1, 1]);
}

#[test]
fn return_solution_in_place_of_2x1() {
    use lu_decomposition::{crout, decompose};
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::from(vec![vec![4.0], vec![-8.0]]);
    let solution = matrix::Matrix::from(vec![vec![0.0], vec![-2.0]]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();

    crout(&matrix, &permutation, &mut matrix_b);
    assert_eq!(matrix_b, solution);
}
#[test]
fn return_decomposition_in_place_of_a_3x3() {
    use lu_decomposition::decompose;
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![
        vec![-12.0, 6.0, 18.0],
        vec![6.0, 4.0, 5.0],
        vec![-48.0, 24.0, 42.0],
    ]);

    let result = decompose(&mut matrix);
    assert!(result.is_ok());
    let (operations, parity) = result.unwrap();
    dbg!(&matrix);
    let solution = matrix::Matrix::from(vec![
        vec![6.0, 4.0, 5.0],
        vec![-8.0, 56.0, 82.0],
        vec![-2.0, 0.25, 7.5],
    ]);
    assert_eq!(matrix, solution);
    assert_eq!(parity, 1);
    assert_eq!(operations, vec![1, 2, 2]);
}

#[test]
fn return_solution_in_place_of_3x1() {
    use lu_decomposition::{crout, decompose};
    let mut matrix = matrix::Matrix::from(vec![
        vec![-12.0, 6.0, 18.0],
        vec![6.0, 4.0, 5.0],
        vec![-48.0, 24.0, 42.0],
    ]);
    let result = decompose(&mut matrix);
    let (permutation, ..) = result.unwrap();
    let mut b = matrix::Matrix::from(vec![vec![19.5], vec![6.0], vec![48.0]]);
    let solution = matrix::Matrix::from(vec![vec![1.0], vec![0.0], vec![0.25]]);
    crout(&matrix, &permutation, &mut b); //Rounding error present. Why?
    assert_eq!(b, solution);
}
#[test]
fn return_inverse() {
    use lu_decomposition::{crout, decompose};
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::identity(2);
    let solution = matrix::Matrix::from(vec![
        vec![4.0 / 24.0, 2.0 / 24.0],
        vec![-6.0 / 24.0, 3.0 / 24.0],
    ]);
    let result = decompose(&mut matrix);
    assert!(result.is_ok());
    let (permutation, ..) = result.unwrap();
    crout(&matrix, &permutation, &mut matrix_b);
    assert_eq!(matrix_b, solution);
}
#[test]
fn return_determinant() {
    use lu_decomposition::decompose;
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut determinant = 1.0;
    let solution = 24.0;
    let result = decompose(&mut matrix);
    let (.., parity) = result.unwrap();
    determinant *= parity as f32;
    for row in 0..matrix.rows() {
        determinant *= matrix.values[row][row];
    }
    assert_eq!(determinant, solution);
}
