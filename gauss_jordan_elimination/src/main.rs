fn main() {
    use gauss_jordan_elimination::gauss_jordan;
    use matrix::Matrix;

    let mut a = Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    let mut i = Matrix::identity(a.rows());
    let return_inverse_in_a = false;
    gauss_jordan(&mut a, &mut i, return_inverse_in_a);
    dbg!(&a.values, &i.values);

    assert_eq!(a, Matrix::identity(a.rows()));
}
