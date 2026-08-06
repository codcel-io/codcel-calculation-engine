// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use nalgebra::DMatrix;
use std::error::Error;

/// Excel-compatible `MMULT` that returns the matrix product of two matrices.
/// - `matrix_a`: the first matrix (m × n).
/// - `matrix_b`: the second matrix (n × p).
///
/// Returns the product matrix (m × p) or an error for incompatible dimensions.
pub fn codcel_m_mult(
    matrix_a: Vec<Vec<f64>>,
    matrix_b: Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    // Ensure both matrices are non-empty
    if matrix_a.is_empty() || matrix_b.is_empty() {
        return Err("MMULT: Input matrices must be non-empty".into());
    }

    let rows_a = matrix_a.len();
    let cols_a = matrix_a[0].len();

    let rows_b = matrix_b.len();
    let cols_b = matrix_b[0].len();

    // Ensure the dimensions are compatible for multiplication
    if cols_a != rows_b {
        return Err("MMULT: Number of columns in the first matrix must equal the number of rows in the second matrix".into());
    }

    // Flatten the 2D Vec<Vec<f64>> into 1D slices for nalgebra's DMatrix
    let flat_a: Vec<f64> = matrix_a.iter().flatten().copied().collect();
    let flat_b: Vec<f64> = matrix_b.iter().flatten().copied().collect();

    // Convert to nalgebra DMatrix
    let dmatrix_a = DMatrix::from_row_slice(rows_a, cols_a, &flat_a);
    let dmatrix_b = DMatrix::from_row_slice(rows_b, cols_b, &flat_b);

    // Perform matrix multiplication
    let result_matrix = dmatrix_a * dmatrix_b;

    // Convert the resulting DMatrix back into a Vec<Vec<f64>>
    let result: Vec<Vec<f64>> = (0..result_matrix.nrows())
        .map(|i| result_matrix.row(i).iter().copied().collect())
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if two matrices are approximately equal
    fn assert_matrices_eq(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>, epsilon: f64) {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i].len(), b[i].len());
            for j in 0..a[i].len() {
                assert!(
                    (a[i][j] - b[i][j]).abs() < epsilon,
                    "Values at [{},{}] differ: {} vs {}",
                    i,
                    j,
                    a[i][j],
                    b[i][j]
                );
            }
        }
    }

    #[test]
    fn test_m_mult_1x1() {
        // =MMULT({2},{3}) in US format
        // =MMULT({2};{3}) in German format
        let matrix_a = vec![vec![2.0]];
        let matrix_b = vec![vec![3.0]];
        let result = codcel_m_mult(matrix_a, matrix_b).unwrap();
        let expected = vec![vec![6.0]]; // 2 * 3 = 6
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_mult_2x2() {
        // =MMULT({1,2;3,4},{5,6;7,8}) in US format
        // =MMULT({1;2;3;4};{5;6;7;8}) in German format
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = codcel_m_mult(matrix_a, matrix_b).unwrap();
        let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
        // [1,2] * [5,6] = 1*5 + 2*7 = 5 + 14 = 19
        // [1,2] * [7,8] = 1*6 + 2*8 = 6 + 16 = 22
        // [3,4] * [5,6] = 3*5 + 4*7 = 15 + 28 = 43
        // [3,4] * [7,8] = 3*6 + 4*8 = 18 + 32 = 50
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_mult_non_square() {
        // =MMULT({1,2,3;4,5,6},{7;8;9}) in US format
        // =MMULT({1;2;3;4;5;6};{7;8;9}) in German format
        let matrix_a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let matrix_b = vec![vec![7.0], vec![8.0], vec![9.0]];
        let result = codcel_m_mult(matrix_a, matrix_b).unwrap();
        let expected = vec![vec![50.0], vec![122.0]];
        // [1,2,3] * [7,8,9]^T = 1*7 + 2*8 + 3*9 = 7 + 16 + 27 = 50
        // [4,5,6] * [7,8,9]^T = 4*7 + 5*8 + 6*9 = 28 + 40 + 54 = 122
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_mult_identity() {
        // =MMULT({1,2;3,4},{1,0;0,1}) in US format
        // =MMULT({1;2;3;4};{1;0;0;1}) in German format
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = codcel_m_mult(matrix_a, matrix_b).unwrap();
        let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        // Multiplying by identity matrix should return the original matrix
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_mult_zeros() {
        // =MMULT({1,2;3,4},{0,0;0,0}) in US format
        // =MMULT({1;2;3;4};{0;0;0;0}) in German format
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let result = codcel_m_mult(matrix_a, matrix_b).unwrap();
        let expected = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        // Multiplying by zero matrix should return a zero matrix
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_mult_incompatible_dimensions() {
        // =MMULT({1,2;3,4},{5,6}) in US format (returns #VALUE! error)
        // =MMULT({1;2;3;4};{5;6}) in German format (returns #VALUE! error)
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![5.0, 6.0]];
        let result = codcel_m_mult(matrix_a, matrix_b);
        assert!(result.is_err());
    }

    #[test]
    fn test_m_mult_empty_matrix() {
        // =MMULT({},{1,2}) in US format (returns #VALUE! error)
        // =MMULT({};{1;2}) in German format (returns #VALUE! error)
        let matrix_a: Vec<Vec<f64>> = vec![];
        let matrix_b = vec![vec![1.0, 2.0]];
        let result = codcel_m_mult(matrix_a, matrix_b);
        assert!(result.is_err());
    }
}
