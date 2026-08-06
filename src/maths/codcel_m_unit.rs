// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use nalgebra::DMatrix;
use std::error::Error;

/// Excel-compatible `MUNIT` that returns the unit (identity) matrix of a specified dimension.
/// - `size`: the dimension of the identity matrix (size × size).
///
/// Returns the identity matrix or an error when size is zero.
pub fn codcel_m_unit(size: usize) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    // Check that size is positive
    if size == 0 {
        return Err("MUNIT: Size must be a positive integer".into());
    }

    // Create an identity matrix using nalgebra
    let identity_matrix = DMatrix::<f64>::identity(size, size);

    // Convert the DMatrix back into a 2D Vec<Vec<f64>>
    let result: Vec<Vec<f64>> = (0..identity_matrix.nrows())
        .map(|i| identity_matrix.row(i).iter().copied().collect())
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
    fn test_m_unit_size_1() {
        // =MUNIT(1) in US format
        // =MUNIT(1) in German format
        let result = codcel_m_unit(1).unwrap();
        let expected = vec![vec![1.0]];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_unit_size_2() {
        // =MUNIT(2) in US format
        // =MUNIT(2) in German format
        let result = codcel_m_unit(2).unwrap();
        let expected = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_unit_size_3() {
        // =MUNIT(3) in US format
        // =MUNIT(3) in German format
        let result = codcel_m_unit(3).unwrap();
        let expected = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        assert_matrices_eq(&result, &expected, 1e-10);
    }

    #[test]
    fn test_m_unit_size_5() {
        // =MUNIT(5) in US format
        // =MUNIT(5) in German format
        let result = codcel_m_unit(5).unwrap();

        // Check dimensions
        assert_eq!(result.len(), 5);
        for row in &result {
            assert_eq!(row.len(), 5);
        }

        // Check that it's an identity matrix
        for i in 0..5 {
            for j in 0..5 {
                if i == j {
                    assert!((result[i][j] - 1.0).abs() < 1e-10);
                } else {
                    assert!((result[i][j] - 0.0).abs() < 1e-10);
                }
            }
        }
    }

    #[test]
    fn test_m_unit_size_0() {
        // =MUNIT(0) in US format (returns #VALUE! error)
        // =MUNIT(0) in German format (returns #VALUE! error)
        let result = codcel_m_unit(0);
        assert!(result.is_err());
    }
}
