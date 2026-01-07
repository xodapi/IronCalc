// Phase 2: Matrix/Math Functions (6 new functions)
// MMULT, MINVERSE, MDETERM, MUNIT, SERIESSUM, MULTINOMIAL

use crate::calc_result::CalcResult;
use crate::expressions::parser::{Node, ArrayNode};
use crate::expressions::token::Error;
use crate::expressions::types::CellReferenceIndex;
use crate::model::Model;

impl Model {
    /// MMULT(array1, array2) - Matrix multiplication
    pub(crate) fn fn_mmult(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 2 { return CalcResult::new_args_number_error(cell); }
        
        let arr1 = self.evaluate_node_in_context(&args[0], cell);
        let arr2 = self.evaluate_node_in_context(&args[1], cell);
        
        let (matrix1, rows1, cols1) = match &arr1 {
            CalcResult::Array(arr) => {
                let r = arr.len();
                let c = arr.get(0).map(|row| row.len()).unwrap_or(0);
                (arr.clone(), r, c)
            }
            CalcResult::Number(n) => (vec![vec![ArrayNode::Number(*n)]], 1, 1),
            CalcResult::Error { .. } => return arr1,
            _ => return CalcResult::new_error(Error::VALUE, cell, "MMULT: Array required".to_string()),
        };
        
        let (matrix2, rows2, cols2) = match &arr2 {
            CalcResult::Array(arr) => {
                let r = arr.len();
                let c = arr.get(0).map(|row| row.len()).unwrap_or(0);
                (arr.clone(), r, c)
            }
            CalcResult::Number(n) => (vec![vec![ArrayNode::Number(*n)]], 1, 1),
            CalcResult::Error { .. } => return arr2,
            _ => return CalcResult::new_error(Error::VALUE, cell, "MMULT: Array required".to_string()),
        };
        
        if cols1 != rows2 {
            return CalcResult::new_error(Error::VALUE, cell, 
                format!("MMULT: Incompatible dimensions {}x{} and {}x{}", rows1, cols1, rows2, cols2));
        }
        
        let mut result: Vec<Vec<ArrayNode>> = Vec::with_capacity(rows1);
        for i in 0..rows1 {
            let mut row: Vec<ArrayNode> = Vec::with_capacity(cols2);
            for j in 0..cols2 {
                let mut sum = 0.0;
                for k in 0..cols1 {
                    let v1 = Self::array_node_to_f64(&matrix1[i][k]);
                    let v2 = Self::array_node_to_f64(&matrix2[k][j]);
                    sum += v1 * v2;
                }
                row.push(ArrayNode::Number(sum));
            }
            result.push(row);
        }
        CalcResult::Array(result)
    }
    
    fn array_node_to_f64(node: &ArrayNode) -> f64 {
        match node {
            ArrayNode::Number(n) => *n,
            ArrayNode::Boolean(b) => if *b { 1.0 } else { 0.0 },
            _ => 0.0,
        }
    }

    /// MINVERSE(array) - Matrix inverse
    pub(crate) fn fn_minverse(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        
        let arr = self.evaluate_node_in_context(&args[0], cell);
        let matrix = match &arr {
            CalcResult::Array(arr) => arr.clone(),
            CalcResult::Number(n) => {
                if *n == 0.0 {
                    return CalcResult::new_error(Error::NUM, cell, "MINVERSE: Singular matrix".to_string());
                }
                return CalcResult::Number(1.0 / n);
            }
            CalcResult::Error { .. } => return arr,
            _ => return CalcResult::new_error(Error::VALUE, cell, "MINVERSE: Array required".to_string()),
        };
        
        let n = matrix.len();
        if n == 0 || matrix[0].len() != n {
            return CalcResult::new_error(Error::VALUE, cell, "MINVERSE: Square matrix required".to_string());
        }
        
        // Convert to f64 matrix
        let mut mat: Vec<Vec<f64>> = matrix.iter().map(|row| 
            row.iter().map(|node| Self::array_node_to_f64(node)).collect()
        ).collect();
        
        // Create identity matrix for augmented system
        let mut inv: Vec<Vec<f64>> = (0..n).map(|i| 
            (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect()
        ).collect();
        
        // Gauss-Jordan elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if mat[k][i].abs() > mat[max_row][i].abs() {
                    max_row = k;
                }
            }
            mat.swap(i, max_row);
            inv.swap(i, max_row);
            
            let pivot = mat[i][i];
            if pivot.abs() < 1e-10 {
                return CalcResult::new_error(Error::NUM, cell, "MINVERSE: Singular matrix".to_string());
            }
            
            // Scale row
            for j in 0..n {
                mat[i][j] /= pivot;
                inv[i][j] /= pivot;
            }
            
            // Eliminate column
            for k in 0..n {
                if k != i {
                    let factor = mat[k][i];
                    for j in 0..n {
                        mat[k][j] -= factor * mat[i][j];
                        inv[k][j] -= factor * inv[i][j];
                    }
                }
            }
        }
        
        let result: Vec<Vec<ArrayNode>> = inv.iter().map(|row|
            row.iter().map(|&v| ArrayNode::Number(v)).collect()
        ).collect();
        CalcResult::Array(result)
    }

    /// MDETERM(array) - Matrix determinant
    pub(crate) fn fn_mdeterm(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        
        let arr = self.evaluate_node_in_context(&args[0], cell);
        let matrix = match &arr {
            CalcResult::Array(arr) => arr.clone(),
            CalcResult::Number(n) => return CalcResult::Number(*n),
            CalcResult::Error { .. } => return arr,
            _ => return CalcResult::new_error(Error::VALUE, cell, "MDETERM: Array required".to_string()),
        };
        
        let n = matrix.len();
        if n == 0 || matrix[0].len() != n {
            return CalcResult::new_error(Error::VALUE, cell, "MDETERM: Square matrix required".to_string());
        }
        
        let mut mat: Vec<Vec<f64>> = matrix.iter().map(|row|
            row.iter().map(|node| Self::array_node_to_f64(node)).collect()
        ).collect();
        
        // LU decomposition to compute determinant
        let mut det = 1.0;
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if mat[k][i].abs() > mat[max_row][i].abs() {
                    max_row = k;
                }
            }
            if max_row != i {
                mat.swap(i, max_row);
                det = -det;
            }
            
            let pivot = mat[i][i];
            if pivot.abs() < 1e-10 {
                return CalcResult::Number(0.0);
            }
            det *= pivot;
            
            for k in (i + 1)..n {
                let factor = mat[k][i] / pivot;
                for j in (i + 1)..n {
                    mat[k][j] -= factor * mat[i][j];
                }
            }
        }
        CalcResult::Number(det)
    }

    /// MUNIT(dimension) - Identity matrix
    pub(crate) fn fn_munit(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        
        let n = match self.get_number(&args[0], cell) {
            Ok(v) => v as usize,
            Err(e) => return e,
        };
        
        if n == 0 || n > 1000 {
            return CalcResult::new_error(Error::VALUE, cell, "MUNIT: Invalid dimension".to_string());
        }
        
        let result: Vec<Vec<ArrayNode>> = (0..n).map(|i|
            (0..n).map(|j| ArrayNode::Number(if i == j { 1.0 } else { 0.0 })).collect()
        ).collect();
        CalcResult::Array(result)
    }

    /// SERIESSUM(x, n, m, coefficients) - Power series sum
    pub(crate) fn fn_seriessum(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        
        let x = match self.get_number(&args[0], cell) { Ok(v) => v, Err(e) => return e };
        let n = match self.get_number(&args[1], cell) { Ok(v) => v, Err(e) => return e };
        let m = match self.get_number(&args[2], cell) { Ok(v) => v, Err(e) => return e };
        
        let coeffs = self.evaluate_node_in_context(&args[3], cell);
        let coefficients: Vec<f64> = match &coeffs {
            CalcResult::Array(arr) => arr.iter().flat_map(|row| 
                row.iter().map(|node| Self::array_node_to_f64(node))
            ).collect(),
            CalcResult::Number(v) => vec![*v],
            CalcResult::Error { .. } => return coeffs,
            _ => return CalcResult::new_error(Error::VALUE, cell, "SERIESSUM: Coefficients required".to_string()),
        };
        
        let mut sum = 0.0;
        for (i, &a) in coefficients.iter().enumerate() {
            let power = n + (i as f64) * m;
            sum += a * x.powf(power);
        }
        CalcResult::Number(sum)
    }

    /// MULTINOMIAL(n1, n2, ...) - Multinomial coefficient
    pub(crate) fn fn_multinomial(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        
        let mut sum = 0.0;
        let mut product = 1.0;
        
        for arg in args {
            let result = self.evaluate_node_in_context(arg, cell);
            let values: Vec<f64> = match &result {
                CalcResult::Number(n) => vec![*n],
                CalcResult::Array(arr) => arr.iter().flat_map(|row|
                    row.iter().map(|node| Self::array_node_to_f64(node))
                ).collect(),
                CalcResult::Error { .. } => return result,
                _ => continue,
            };
            
            for n in values {
                if n < 0.0 || n != n.trunc() {
                    return CalcResult::new_error(Error::NUM, cell, "MULTINOMIAL: Positive integers required".to_string());
                }
                sum += n;
                product *= Self::factorial(n as u64);
            }
        }
        
        CalcResult::Number(Self::factorial(sum as u64) / product)
    }
    
    fn factorial(n: u64) -> f64 {
        if n <= 1 { return 1.0; }
        let mut result = 1.0;
        for i in 2..=n {
            result *= i as f64;
        }
        result
    }
}
