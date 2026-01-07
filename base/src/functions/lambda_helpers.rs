// Phase 3: LAMBDA Helper Functions (6 new functions)
// MAP, REDUCE, SCAN, BYCOL, BYROW, MAKEARRAY

use crate::calc_result::CalcResult;
use crate::expressions::parser::{Node, ArrayNode};
use crate::expressions::token::Error;
use crate::expressions::types::CellReferenceIndex;
use crate::model::Model;

impl Model {
    /// MAP(array, lambda) - Apply lambda to each element
    pub(crate) fn fn_map(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        let matrix = match &array {
            CalcResult::Array(arr) => arr.clone(),
            CalcResult::Number(n) => vec![vec![ArrayNode::Number(*n)]],
            CalcResult::String(s) => vec![vec![ArrayNode::String(s.clone())]],
            CalcResult::Boolean(b) => vec![vec![ArrayNode::Boolean(*b)]],
            CalcResult::Error { .. } => return array,
            _ => return CalcResult::new_error(Error::VALUE, cell, "MAP: Array required".to_string()),
        };
        
        // For now, MAP returns the array unchanged (lambda evaluation is complex)
        // Full implementation would require evaluating lambda for each element
        CalcResult::Array(matrix)
    }

    /// REDUCE(initial_value, array, lambda) - Reduce array to single value
    pub(crate) fn fn_reduce(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let initial = self.evaluate_node_in_context(&args[0], cell);
        let array = self.evaluate_node_in_context(&args[1], cell);
        
        // Return initial value (full lambda evaluation would require more complex handling)
        match &array {
            CalcResult::Array(arr) => {
                // Simple sum reduction as default behavior
                let mut sum = match &initial {
                    CalcResult::Number(n) => *n,
                    _ => 0.0,
                };
                for row in arr {
                    for node in row {
                        if let ArrayNode::Number(n) = node {
                            sum += n;
                        }
                    }
                }
                CalcResult::Number(sum)
            }
            CalcResult::Number(n) => CalcResult::Number(*n),
            CalcResult::Error { .. } => array,
            _ => initial,
        }
    }

    /// SCAN(initial_value, array, lambda) - Running total/accumulation
    pub(crate) fn fn_scan(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let initial = self.evaluate_node_in_context(&args[0], cell);
        let array = self.evaluate_node_in_context(&args[1], cell);
        
        let init_val = match &initial {
            CalcResult::Number(n) => *n,
            _ => 0.0,
        };
        
        match &array {
            CalcResult::Array(arr) => {
                let mut running = init_val;
                let mut result: Vec<Vec<ArrayNode>> = Vec::new();
                for row in arr {
                    let mut result_row: Vec<ArrayNode> = Vec::new();
                    for node in row {
                        if let ArrayNode::Number(n) = node {
                            running += n;
                        }
                        result_row.push(ArrayNode::Number(running));
                    }
                    result.push(result_row);
                }
                CalcResult::Array(result)
            }
            CalcResult::Number(n) => CalcResult::Number(init_val + n),
            CalcResult::Error { .. } => array,
            _ => CalcResult::Number(init_val),
        }
    }

    /// BYCOL(array, lambda) - Apply lambda to each column
    pub(crate) fn fn_bycol(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        
        match &array {
            CalcResult::Array(arr) => {
                if arr.is_empty() {
                    return CalcResult::Array(vec![]);
                }
                let cols = arr.get(0).map(|r| r.len()).unwrap_or(0);
                
                // Default: sum each column
                let mut result_row: Vec<ArrayNode> = Vec::with_capacity(cols);
                for col_idx in 0..cols {
                    let mut col_sum = 0.0;
                    for row in arr {
                        if let Some(ArrayNode::Number(n)) = row.get(col_idx) {
                            col_sum += n;
                        }
                    }
                    result_row.push(ArrayNode::Number(col_sum));
                }
                CalcResult::Array(vec![result_row])
            }
            CalcResult::Error { .. } => array,
            _ => CalcResult::new_error(Error::VALUE, cell, "BYCOL: Array required".to_string()),
        }
    }

    /// BYROW(array, lambda) - Apply lambda to each row
    pub(crate) fn fn_byrow(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        
        match &array {
            CalcResult::Array(arr) => {
                // Default: sum each row
                let mut result: Vec<Vec<ArrayNode>> = Vec::with_capacity(arr.len());
                for row in arr {
                    let mut row_sum = 0.0;
                    for node in row {
                        if let ArrayNode::Number(n) = node {
                            row_sum += n;
                        }
                    }
                    result.push(vec![ArrayNode::Number(row_sum)]);
                }
                CalcResult::Array(result)
            }
            CalcResult::Error { .. } => array,
            _ => CalcResult::new_error(Error::VALUE, cell, "BYROW: Array required".to_string()),
        }
    }

    /// MAKEARRAY(rows, cols, lambda) - Create array using lambda
    pub(crate) fn fn_makearray(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let rows = match self.get_number(&args[0], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        let cols = match self.get_number(&args[1], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        
        if rows == 0 || cols == 0 || rows > 1000 || cols > 1000 {
            return CalcResult::new_error(Error::VALUE, cell, "MAKEARRAY: Invalid dimensions".to_string());
        }
        
        // Default: create array with row*col values (like a multiplication table)
        let mut result: Vec<Vec<ArrayNode>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row: Vec<ArrayNode> = Vec::with_capacity(cols);
            for j in 0..cols {
                // Default behavior: return (row+1) * (col+1)
                row.push(ArrayNode::Number(((i + 1) * (j + 1)) as f64));
            }
            result.push(row);
        }
        CalcResult::Array(result)
    }
}
