/// Linear equation solution by Gauss-Jordan elimination, equation (2.1.1) above.
///
///
/// The input matrix is a[0..n-1][0..n-1]. b[0..n-1][0..m-1] is input containing the m right-hand side vectors.
///On output, a is replaced by its matrix inverse, and b is replaced by the corresponding set of
///solution vectors.
pub fn gaussj(
    a: &mut crate::matrix::Matrix,
    b: &mut crate::matrix::Matrix,
    return_inverse_in_a: bool,
) {
    let n = a.rows();
    let m = b.columns();
    //This usize vector is used for bookkeeping on the pivoting.
    let mut ipiv = vec![0; a.columns()];
    let mut swaps: Vec<(usize, usize)> = Vec::new();

    //This is the main loop over the columns to be reduced.
    '_main: for step in 0..a.columns() {
        let (mut irow, mut icol): (usize, usize) = (0, 0);

        dbg!(step, &a.values);
        select_pivot_location(&a, (&mut irow, &mut icol), &mut ipiv);
        ipiv[icol] += 1;

        // If element is at (0, 2), move it to (2,2) by swapping row0 with row2.
        // If element is on the diagonal, e.g.(row == col) then return doing nothing.
        a.put_element_on_diagonal((irow, icol));
        b.put_element_on_diagonal((irow, icol));

        assert_ne!(a.values[icol][icol], 0.0, "gaussj: Singular Matrix");
        /*We are now ready to divide the pivot row by the pivot element, located at irow and icol.*/
        let pivot_index = icol; //Code after this point assumes the pivot element is on the diagonal;
                                //The pivot row is equal to the pivot column.
        let pivot = a.values[pivot_index][pivot_index];
        let pivot_inverse = 1.0 / pivot;

        //Multiplicatively Scale the pivot_row by the pivot element.
        if return_inverse_in_a {
            swaps.push((irow, icol));
            a.values[pivot_index][pivot_index] = pivot * pivot_inverse; //1.0 <-Set pivot point to 1 before scaling and reduction.
        }

        a.scale_row(pivot_index, pivot_inverse);
        b.scale_row(pivot_index, pivot_inverse);

        '_traverse_matrix: for row in 0..n {
            //Next, we reduce the rows... //...except for the pivot one, of course.
            if row == pivot_index {
                continue;
            }

            let linear_combination_constant: f32 = a.values[row][pivot_index]; //Get element in the same columns as the pivot element.
            if return_inverse_in_a {
                a.values[row][pivot_index] = 0.0;
            } // <- set pivot column

            '_traverse_elements_in_row: for col in 0..n {
                a.values[row][col] -= a.values[pivot_index][col] * linear_combination_constant;
            }
            for col in 0..m {
                b.values[row][col] -= b.values[pivot_index][col] * linear_combination_constant;
            }
        }
    }
    //THE STEPS THAT RETURN INVERSE IN A
    // Set pivot location to 1.0 just before scaling pivot row by selected pivot element(above)
    // Set the rest of pivot column to 0.00 just before replacing each row with a linear combination.(sbove)
    // Apply swap row operations in reverse order as swap column operations to bring out Matrix A^-1 (below)
    if return_inverse_in_a {
        for (row, column) in swaps.iter().rev() {
            if row == column {
                continue;
            }
            a.swap_columns((*row, *column));
        }
    }
}

fn select_pivot_location(
    a: &crate::matrix::Matrix,
    (row_of_max, col_of_max): (&mut usize, &mut usize),
    ipiv: &mut Vec<u32>,
) {
    let mut max_absolute_value = 0.0_f32.abs();
    '_traverse_matrix: for row in 0..a.rows() {
        if ipiv[row] == 1 {
            continue;
        }
        'traverse_remaining_columns: for column in 0..a.columns() {
            assert!(ipiv[column] <= 1, "Interesting: see select_pivot_location");
            if ipiv[column] != 0 {
                continue;
            }
            dbg!(a.values[row][column].abs(), row, column);
            if a.values[row][column].abs() <= max_absolute_value {
                continue 'traverse_remaining_columns;
            }
            //record location of new max
            max_absolute_value = a.values[row][column].abs();
            *row_of_max = row;
            *col_of_max = column;
        }
    }
}
