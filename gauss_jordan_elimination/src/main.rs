fn main() {
    use gauss_jordan_elimination::gauss_jordan;
    use matrix::Matrix;

    let mut a = Matrix::from(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ]);
    let mut b = Matrix::from(vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ]);
    let return_inverse_in_a = false;
    gauss_jordan(&mut a, &mut b, return_inverse_in_a);
    dbg!(&b.values);

    // assert_eq!(a, Matrix::identity(a.rows()));
}
