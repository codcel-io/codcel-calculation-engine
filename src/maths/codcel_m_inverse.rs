// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use nalgebra::DMatrix;
use std::error::Error;

/// Excel-compatible `MINVERSE` that returns the inverse matrix of a square matrix.
/// - `matrix`: a square matrix as a 2D vector.
///
/// Returns the inverse matrix or an error for empty, non-square, or singular matrices.
pub fn codcel_m_inverse(
    matrix: Vec<Vec<f64>>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    // Ensure matrix is square
    if matrix.is_empty() || matrix.len() != matrix[0].len() {
        return Err("MINVERSE: Matrix must be square and non-empty".into());
    }

    let size = matrix.len();
    let flat_matrix: Vec<f64> = matrix.iter().flatten().copied().collect();
    let dmatrix = DMatrix::from_row_slice(size, size, &flat_matrix);

    // Directly try to invert the matrix using nalgebra's method
    match dmatrix.try_inverse() {
        Some(inverse) => {
            let inverse_vec: Vec<Vec<f64>> = (0..size)
                .map(|i| inverse.row(i).iter().copied().collect())
                .collect();
            Ok(inverse_vec)
        }
        None => Err("MINVERSE: Matrix is singular and cannot be inverted".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if two matrices are approximately equal
    fn assert_matrices_eq(a: &[Vec<f64>], b: &[Vec<f64>], epsilon: f64) {
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
    fn test_m_inverse_1x1() {
        // =MINVERSE({4}) in US format
        // =MINVERSE({4}) in German format
        let matrix = vec![vec![4.0]];
        let result = codcel_m_inverse(matrix).unwrap();
        let expected = vec![vec![0.25]]; // 1/4 = 0.25
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_inverse_2x2() {
        // =MINVERSE({4,7;2,6}) in US format
        // =MINVERSE({4;7;2;6}) in German format
        let matrix = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
        let result = codcel_m_inverse(matrix).unwrap();
        let expected = vec![vec![0.6, -0.7], vec![-0.2, 0.4]];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_inverse_3x3() {
        // =MINVERSE({1,2,3;0,1,4;5,6,0}) in US format
        // =MINVERSE({1;2;3;0;1;4;5;6;0}) in German format
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![0.0, 1.0, 4.0],
            vec![5.0, 6.0, 0.0],
        ];
        let result = codcel_m_inverse(matrix).unwrap();
        let expected = vec![
            vec![-24.0, 18.0, 5.0],
            vec![20.0, -15.0, -4.0],
            vec![-5.0, 4.0, 1.0],
        ];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_inverse_identity() {
        // =MINVERSE({1,0,0;0,1,0;0,0,1}) in US format
        // =MINVERSE({1;0;0;0;1;0;0;0;1}) in German format
        let matrix = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let result = codcel_m_inverse(matrix).unwrap();
        let expected = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_inverse_singular() {
        // =MINVERSE({1,2,3;4,5,6;7,8,9}) in US format (returns #NUM! error)
        // =MINVERSE({1;2;3;4;5;6;7;8;9}) in German format (returns #NUM! error)
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let result = codcel_m_inverse(matrix);
        println!("{result:?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_m_inverse_non_square() {
        // =MINVERSE({1,2,3;4,5,6}) in US format (returns #VALUE! error)
        // =MINVERSE({1;2;3;4;5;6}) in German format (returns #VALUE! error)
        let matrix = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = codcel_m_inverse(matrix);
        assert!(result.is_err());
    }

    #[test]
    fn test_m_inverse_empty() {
        // =MINVERSE({}) in US format (returns #VALUE! error)
        // =MINVERSE({}) in German format (returns #VALUE! error)
        let matrix: Vec<Vec<f64>> = vec![];
        let result = codcel_m_inverse(matrix);
        assert!(result.is_err());
    }
}
