#[test]
fn return_solution_in_place_of_b() {
    use gauss_jordan_elimination::gauss_jordan;
    use matrix;
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::from(vec![vec![4.0], vec![-8.0]]);
    let solution = matrix::Matrix::from(vec![vec![0.0], vec![-2.0]]);

    gauss_jordan(&mut matrix, &mut matrix_b, false);

    assert_eq!(matrix_b, solution);
}
#[test]
fn return_inverse_in_place_of_a() {
    use gauss_jordan_elimination::gauss_jordan;
    use matrix;
    let mut matrix_a = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut matrix_b = matrix::Matrix::from(vec![vec![4.0], vec![-8.0]]);
    let solution = matrix::Matrix::from(vec![
        vec![4.0 / 24.0, 2.0 / 24.0],
        vec![-6.0 / 24.0, 3.0 / 24.0],
    ]);
    gauss_jordan(&mut matrix_a, &mut matrix_b, true); //set inverse flag to true.
    assert_eq!(matrix_a, solution);
}
