// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use nalgebra::DMatrix;
use std::error::Error;

/// Excel-compatible `MDETERM` that returns the determinant of a matrix.
/// - `matrix`: a square matrix as a 2D vector.
///
/// Returns the determinant or an error for empty or non-square matrices.
pub fn codcel_m_determ(matrix: Vec<Vec<f64>>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Ensure the matrix is non-empty and square
    if matrix.is_empty() || matrix.len() != matrix[0].len() {
        return Err("MDETERM: Matrix must be square and non-empty".into());
    }

    // Convert the 2D Vec into a nalgebra DMatrix
    let size = matrix.len();
    let flat_matrix: Vec<f64> = matrix.into_iter().flatten().collect();
    let dmatrix = DMatrix::from_row_slice(size, size, &flat_matrix);

    // Compute the determinant
    Ok(dmatrix.determinant())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m_determ_1x1() {
        // =MDETERM({5}) in US format
        // =MDETERM({5}) in German format
        let matrix = vec![vec![5.0]];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_m_determ_2x2() {
        // =MDETERM({4,3;2,1}) in US format
        // =MDETERM({4;3;2;1}) in German format
        let matrix = vec![vec![4.0, 3.0], vec![2.0, 1.0]];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, -2.0); // 4*1 - 3*2 = 4 - 6 = -2
    }

    #[test]
    fn test_m_determ_3x3() {
        // =MDETERM({1,2,3;4,5,6;7,8,9}) in US format
        // =MDETERM({1;2;3;4;5;6;7;8;9}) in German format
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, 0.0); // This matrix has determinant 0
    }

    #[test]
    fn test_m_determ_3x3_nonzero() {
        // =MDETERM({1,2,3;4,5,6;7,8,0}) in US format
        // =MDETERM({1;2;3;4;5;6;7;8;0}) in German format
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 0.0],
        ];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, 27.0);
    }

    #[test]
    fn test_m_determ_identity() {
        // =MDETERM({1,0,0;0,1,0;0,0,1}) in US format
        // =MDETERM({1;0;0;0;1;0;0;0;1}) in German format
        let matrix = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, 1.0); // Identity matrix has determinant 1
    }

    #[test]
    fn test_m_determ_negative_values() {
        // =MDETERM({-1,2;3,-4}) in US format
        // =MDETERM({-1;2;3;-4}) in German format
        let matrix = vec![vec![-1.0, 2.0], vec![3.0, -4.0]];
        let result = codcel_m_determ(matrix).unwrap();
        assert_eq!(result, -2.0); // (-1)*(-4) - 2*3 = 4 - 6 = -2
    }

    #[test]
    fn test_m_determ_non_square() {
        // =MDETERM({1,2,3;4,5,6}) in US format (returns #VALUE! error)
        // =MDETERM({1;2;3;4;5;6}) in German format (returns #VALUE! error)
        let matrix = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = codcel_m_determ(matrix);
        assert!(result.is_err());
    }

    #[test]
    fn test_m_determ_empty() {
        // =MDETERM({}) in US format (returns #VALUE! error)
        // =MDETERM({}) in German format (returns #VALUE! error)
        let matrix: Vec<Vec<f64>> = vec![];
        let result = codcel_m_determ(matrix);
        assert!(result.is_err());
    }
}
