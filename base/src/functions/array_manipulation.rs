// Phase 4: SQL-like and Array Manipulation Functions (8 new functions)
// GROUPBY, PIVOTBY, TOCOL, TOROW, WRAPROWS, WRAPCOLS, EXPAND, TEXTSPLIT

use crate::calc_result::CalcResult;
use crate::expressions::parser::{Node, ArrayNode};
use crate::expressions::token::Error;
use crate::expressions::types::CellReferenceIndex;
use crate::model::Model;

impl Model {
    /// GROUPBY(row_fields, values, function) - SQL-like GROUP BY
    pub(crate) fn fn_groupby(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        // Stub implementation - full GROUP BY requires complex aggregation
        CalcResult::new_error(Error::CALC, cell, "GROUPBY: Not fully implemented".to_string())
    }

    /// PIVOTBY(row_fields, col_fields, values, function) - SQL-like PIVOT
    pub(crate) fn fn_pivotby(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 {
            return CalcResult::new_args_number_error(cell);
        }
        // Stub implementation - full PIVOT requires complex aggregation
        CalcResult::new_error(Error::CALC, cell, "PIVOTBY: Not fully implemented".to_string())
    }

    /// TOCOL(array, [ignore], [scan_by_column]) - Convert array to column
    pub(crate) fn fn_tocol(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        
        match &array {
            CalcResult::Array(arr) => {
                let mut result: Vec<Vec<ArrayNode>> = Vec::new();
                for row in arr {
                    for node in row {
                        result.push(vec![node.clone()]);
                    }
                }
                CalcResult::Array(result)
            }
            CalcResult::Number(n) => CalcResult::Array(vec![vec![ArrayNode::Number(*n)]]),
            CalcResult::String(s) => CalcResult::Array(vec![vec![ArrayNode::String(s.clone())]]),
            CalcResult::Error { .. } => array,
            _ => CalcResult::new_error(Error::VALUE, cell, "TOCOL: Array required".to_string()),
        }
    }

    /// TOROW(array, [ignore], [scan_by_column]) - Convert array to row
    pub(crate) fn fn_torow(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        
        match &array {
            CalcResult::Array(arr) => {
                let mut result_row: Vec<ArrayNode> = Vec::new();
                for row in arr {
                    for node in row {
                        result_row.push(node.clone());
                    }
                }
                CalcResult::Array(vec![result_row])
            }
            CalcResult::Number(n) => CalcResult::Array(vec![vec![ArrayNode::Number(*n)]]),
            CalcResult::String(s) => CalcResult::Array(vec![vec![ArrayNode::String(s.clone())]]),
            CalcResult::Error { .. } => array,
            _ => CalcResult::new_error(Error::VALUE, cell, "TOROW: Array required".to_string()),
        }
    }

    /// WRAPROWS(vector, wrap_count, [pad_with]) - Wrap vector into rows
    pub(crate) fn fn_wraprows(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        let wrap_count = match self.get_number(&args[1], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        
        if wrap_count == 0 {
            return CalcResult::new_error(Error::VALUE, cell, "WRAPROWS: Invalid wrap count".to_string());
        }
        
        let values: Vec<ArrayNode> = match &array {
            CalcResult::Array(arr) => arr.iter().flat_map(|row| row.iter().cloned()).collect(),
            CalcResult::Number(n) => vec![ArrayNode::Number(*n)],
            CalcResult::Error { .. } => return array,
            _ => return CalcResult::new_error(Error::VALUE, cell, "WRAPROWS: Array required".to_string()),
        };
        
        let mut result: Vec<Vec<ArrayNode>> = Vec::new();
        for chunk in values.chunks(wrap_count) {
            let mut row = chunk.to_vec();
            while row.len() < wrap_count {
                row.push(ArrayNode::Number(0.0)); // Default pad
            }
            result.push(row);
        }
        CalcResult::Array(result)
    }

    /// WRAPCOLS(vector, wrap_count, [pad_with]) - Wrap vector into columns
    pub(crate) fn fn_wrapcols(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        let wrap_count = match self.get_number(&args[1], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        
        if wrap_count == 0 {
            return CalcResult::new_error(Error::VALUE, cell, "WRAPCOLS: Invalid wrap count".to_string());
        }
        
        let values: Vec<ArrayNode> = match &array {
            CalcResult::Array(arr) => arr.iter().flat_map(|row| row.iter().cloned()).collect(),
            CalcResult::Number(n) => vec![ArrayNode::Number(*n)],
            CalcResult::Error { .. } => return array,
            _ => return CalcResult::new_error(Error::VALUE, cell, "WRAPCOLS: Array required".to_string()),
        };
        
        let num_cols = (values.len() + wrap_count - 1) / wrap_count;
        let mut result: Vec<Vec<ArrayNode>> = (0..wrap_count).map(|_| Vec::with_capacity(num_cols)).collect();
        
        for (i, val) in values.iter().enumerate() {
            let row = i % wrap_count;
            result[row].push(val.clone());
        }
        
        // Pad rows to equal length
        let max_len = result.iter().map(|r| r.len()).max().unwrap_or(0);
        for row in &mut result {
            while row.len() < max_len {
                row.push(ArrayNode::Number(0.0));
            }
        }
        
        CalcResult::Array(result)
    }

    /// EXPAND(array, rows, cols, [pad_with]) - Expand array to dimensions
    pub(crate) fn fn_expand(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let array = self.evaluate_node_in_context(&args[0], cell);
        let target_rows = match self.get_number(&args[1], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        let target_cols = match self.get_number(&args[2], cell) {
            Ok(n) => n as usize,
            Err(e) => return e,
        };
        
        let source = match &array {
            CalcResult::Array(arr) => arr.clone(),
            CalcResult::Number(n) => vec![vec![ArrayNode::Number(*n)]],
            CalcResult::Error { .. } => return array,
            _ => vec![vec![ArrayNode::Number(0.0)]],
        };
        
        let mut result: Vec<Vec<ArrayNode>> = Vec::with_capacity(target_rows);
        for i in 0..target_rows {
            let mut row: Vec<ArrayNode> = Vec::with_capacity(target_cols);
            for j in 0..target_cols {
                if i < source.len() && j < source[i].len() {
                    row.push(source[i][j].clone());
                } else {
                    row.push(ArrayNode::Number(0.0)); // Default pad
                }
            }
            result.push(row);
        }
        CalcResult::Array(result)
    }

    /// TEXTSPLIT(text, col_delimiter, [row_delimiter]) - Split text into array
    pub(crate) fn fn_textsplit(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        let col_delim = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        let row_delim = if args.len() > 2 {
            match self.get_string(&args[2], cell) {
                Ok(s) => Some(s),
                Err(e) => return e,
            }
        } else {
            None
        };
        
        let rows: Vec<&str> = if let Some(ref rd) = row_delim {
            text.split(rd.as_str()).collect()
        } else {
            vec![text.as_str()]
        };
        
        let mut result: Vec<Vec<ArrayNode>> = Vec::new();
        for row_str in rows {
            let cols: Vec<ArrayNode> = row_str
                .split(col_delim.as_str())
                .map(|s| ArrayNode::String(s.to_string()))
                .collect();
            result.push(cols);
        }
        
        CalcResult::Array(result)
    }
}
