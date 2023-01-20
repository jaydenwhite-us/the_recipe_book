fn main() {
    use lu_decomposition::{crout, decompose};
    let mut matrix = matrix::Matrix::from(vec![vec![3.0, -2.0], vec![6.0, 4.0]]);
    println!("Matrix: {:?}", matrix);

    //LU decomposition.
    let result = decompose(&mut matrix);
    let (permutation, parity) = result.unwrap();
    println!("Decomposition: {:?}", matrix);

    //Calculate Right Hand Side. In this case, the solution will be the inverse.
    let mut b = matrix::Matrix::identity(2);
    crout(&matrix, &permutation, &mut b);
    println!("Inverse: {:?}", b);

    //Calculate Determinant
    let mut determinant = parity as f32;
    for row in 0..matrix.rows() {
        determinant *= matrix.values[row][row];
    }
    println!("Determinant: {:?}", determinant);
}
