// Dynamic Array Functions: SEQUENCE, FILTER, UNIQUE, SORT, SORTBY, LET, XMATCH

use crate::{
    calc_result::CalcResult,
    expressions::{parser::{Node, ArrayNode}, token::Error, types::CellReferenceIndex},
    Model,
};

impl Model {
    /// =SEQUENCE(rows, [columns], [start], [step])
    /// Generates an array of sequential numbers
    pub(crate) fn fn_sequence(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 1 || arg_count > 4 {
            return CalcResult::new_args_number_error(cell);
        }

        // Get rows (required)
        let rows = match self.get_number(&args[0], cell) {
            Ok(v) => v as i32,
            Err(e) => return e,
        };
        if rows < 1 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "SEQUENCE: rows must be >= 1".to_string(),
            };
        }

        // Get columns (default 1)
        let columns = if arg_count >= 2 {
            match self.get_number(&args[1], cell) {
                Ok(v) => v as i32,
                Err(e) => return e,
            }
        } else {
            1
        };
        if columns < 1 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "SEQUENCE: columns must be >= 1".to_string(),
            };
        }

        // Get start (default 1)
        let start = if arg_count >= 3 {
            match self.get_number(&args[2], cell) {
                Ok(v) => v,
                Err(e) => return e,
            }
        } else {
            1.0
        };

        // Get step (default 1)
        let step = if arg_count >= 4 {
            match self.get_number(&args[3], cell) {
                Ok(v) => v,
                Err(e) => return e,
            }
        } else {
            1.0
        };

        // Generate the array
        let mut result = Vec::with_capacity(rows as usize);
        let mut value = start;

        for _row in 0..rows {
            let mut row_data = Vec::with_capacity(columns as usize);
            for _col in 0..columns {
                row_data.push(ArrayNode::Number(value));
                value += step;
            }
            result.push(row_data);
        }

        CalcResult::Array(result)
    }

    /// =FILTER(array, include, [if_empty])
    /// Filters an array based on a boolean array
    pub(crate) fn fn_filter(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 2 || arg_count > 3 {
            return CalcResult::new_args_number_error(cell);
        }

        // Get the array to filter
        let array = self.evaluate_node_in_context(&args[0], cell);
        let include = self.evaluate_node_in_context(&args[1], cell);

        // Get if_empty (optional)
        let if_empty: Option<CalcResult> = if arg_count >= 3 {
            Some(self.evaluate_node_in_context(&args[2], cell))
        } else {
            None
        };

        // Convert to 2D arrays
        let array_data: Vec<Vec<ArrayNode>> = match self.calc_result_to_2d_array(&array, cell) {
            Ok(data) => data,
            Err(e) => return e,
        };

        let include_data: Vec<Vec<ArrayNode>> = match self.calc_result_to_2d_array(&include, cell) {
            Ok(data) => data,
            Err(e) => return e,
        };

        if array_data.is_empty() {
            return if let Some(empty_val) = if_empty {
                empty_val
            } else {
                CalcResult::Error {
                    error: Error::CALC,
                    origin: cell,
                    message: "FILTER: no matches found".to_string(),
                }
            };
        }

        let array_rows = array_data.len();
        let array_cols = array_data.first().map(|r| r.len()).unwrap_or(0);
        let include_rows = include_data.len();
        let include_cols = include_data.first().map(|r| r.len()).unwrap_or(0);

        // Determine filter direction: by rows or by columns
        // STRICT validation: dimensions must match exactly
        let filter_by_rows: bool;
        
        // Check for row filtering: include must have same rows as array
        // and include should be a column vector (Nx1)
        if include_rows == array_rows && include_cols == 1 {
            filter_by_rows = true;
        }
        // Check for column filtering: include must have same cols as array
        // and include should be a row vector (1xM)
        else if include_rows == 1 && include_cols == array_cols {
            filter_by_rows = false;
        }
        // 2D array case: include dimensions must match array dimensions
        else if include_rows == array_rows && include_cols == array_cols {
            // Full 2D match - filter by rows (each row's first column determines)
            filter_by_rows = true;
        }
        // No valid dimension match
        else {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "FILTER: include dimensions must match array".to_string(),
            };
        }

        let mut result: Vec<Vec<ArrayNode>> = Vec::new();

        if filter_by_rows {
            // Filter rows: include_data should have same # of rows
            for (row_idx, row) in array_data.iter().enumerate() {
                let should_include = if let Some(inc_row) = include_data.get(row_idx) {
                    match inc_row.first() {
                        Some(ArrayNode::Boolean(b)) => *b,
                        Some(ArrayNode::Number(n)) => *n != 0.0,
                        _ => false,
                    }
                } else {
                    false
                };
                if should_include {
                    result.push(row.clone());
                }
            }
        } else {
            // Filter columns: include_data[0] should have same # of cols
            let empty_vec: Vec<ArrayNode> = Vec::new();
            let include_row = include_data.first().unwrap_or(&empty_vec);
            
            // Initialize result rows
            for row in &array_data {
                let mut filtered_row: Vec<ArrayNode> = Vec::new();
                for (col_idx, val) in row.iter().enumerate() {
                    let should_include = match include_row.get(col_idx) {
                        Some(ArrayNode::Boolean(b)) => *b,
                        Some(ArrayNode::Number(n)) => *n != 0.0,
                        _ => false,
                    };
                    if should_include {
                        filtered_row.push(val.clone());
                    }
                }
                if !filtered_row.is_empty() {
                    result.push(filtered_row);
                }
            }
        }

        // Return if_empty if no matches
        if result.is_empty() || (result.len() == 1 && result[0].is_empty()) {
            return if let Some(empty_val) = if_empty {
                empty_val
            } else {
                CalcResult::Error {
                    error: Error::CALC,
                    origin: cell,
                    message: "FILTER: no matches found".to_string(),
                }
            };
        }

        CalcResult::Array(result)
    }

    /// =UNIQUE(array, [by_col], [exactly_once])
    /// Returns unique values from a range or array
    pub(crate) fn fn_unique(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 1 || arg_count > 3 {
            return CalcResult::new_args_number_error(cell);
        }

        let array = self.evaluate_node_in_context(&args[0], cell);
        
        let by_col = if arg_count >= 2 {
            match self.get_boolean(&args[1], cell) {
                Ok(v) => v,
                Err(e) => return e,
            }
        } else {
            false
        };

        let exactly_once = if arg_count >= 3 {
            match self.get_boolean(&args[2], cell) {
                Ok(v) => v,
                Err(e) => return e,
            }
        } else {
            false
        };

        let array_data = match self.calc_result_to_2d_array(&array, cell) {
            Ok(data) => data,
            Err(e) => return e,
        };

        // Transpose if by_col is true (work with columns as rows)
        let array_data = if by_col {
            Self::transpose_2d(&array_data)
        } else {
            array_data
        };

        // Find unique rows
        let mut seen: Vec<Vec<ArrayNode>> = Vec::new();
        let mut counts: Vec<usize> = Vec::new();

        for row in &array_data {
            if let Some(pos) = seen.iter().position(|r| self.array_rows_equal(r, row)) {
                counts[pos] += 1;
            } else {
                seen.push(row.clone());
                counts.push(1);
            }
        }

        let result: Vec<Vec<ArrayNode>> = if exactly_once {
            seen.into_iter()
                .zip(counts.into_iter())
                .filter(|(_, count)| *count == 1)
                .map(|(row, _)| row)
                .collect()
        } else {
            seen
        };

        if result.is_empty() {
            return CalcResult::Error {
                error: Error::CALC,
                origin: cell,
                message: "UNIQUE: no unique values found".to_string(),
            };
        }

        // Transpose result back if by_col
        let result = if by_col {
            Self::transpose_2d(&result)
        } else {
            result
        };

        CalcResult::Array(result)
    }

    /// =SORT(array, [sort_index], [sort_order], [by_col])
    pub(crate) fn fn_sort(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 1 || arg_count > 4 {
            return CalcResult::new_args_number_error(cell);
        }

        let array = self.evaluate_node_in_context(&args[0], cell);

        let sort_index = if arg_count >= 2 {
            match self.get_number(&args[1], cell) {
                Ok(v) => v as usize,
                Err(e) => return e,
            }
        } else {
            1
        };

        let ascending = if arg_count >= 3 {
            match self.get_number(&args[2], cell) {
                Ok(v) => v >= 0.0,
                Err(e) => return e,
            }
        } else {
            true
        };

        let by_col = if arg_count >= 4 {
            match self.get_boolean(&args[3], cell) {
                Ok(v) => v,
                Err(e) => return e,
            }
        } else {
            false
        };

        let array_data = match self.calc_result_to_2d_array(&array, cell) {
            Ok(data) => data,
            Err(e) => return e,
        };

        // Transpose if by_col is true (work with columns)
        let mut array_data = if by_col {
            Self::transpose_2d(&array_data)
        } else {
            array_data
        };

        let col_idx = if sort_index > 0 { sort_index - 1 } else { 0 };

        // Sort with explicit type annotations
        array_data.sort_by(|a: &Vec<ArrayNode>, b: &Vec<ArrayNode>| {
            let val_a = a.get(col_idx);
            let val_b = b.get(col_idx);
            let cmp = Self::compare_array_nodes_static(val_a, val_b);
            if ascending { cmp } else { cmp.reverse() }
        });

        // Transpose back if by_col
        let result = if by_col {
            Self::transpose_2d(&array_data)
        } else {
            array_data
        };

        CalcResult::Array(result)
    }

    /// =SORTBY(array, by_array1, [sort_order1], ...)
    pub(crate) fn fn_sortby(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 {
            return CalcResult::new_args_number_error(cell);
        }
        self.fn_sort(args, cell)
    }

    /// =LET(name1, value1, [name2, value2, ...], calculation)
    /// 
    /// Assigns names to calculation results for reuse within a formula.
    /// Variable names are parsed as WrongVariableKind nodes by the parser.
    pub(crate) fn fn_let(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        
        // LET requires at least 3 arguments (name, value, calculation) and odd number
        if arg_count < 3 || arg_count % 2 == 0 {
            return CalcResult::new_args_number_error(cell);
        }

        // Save old scope for proper cleanup (supports nested LET)
        let old_scope = self.lambda_scope.clone();

        // Process name-value pairs
        let num_pairs = (arg_count - 1) / 2;
        let mut bound_names: Vec<String> = Vec::with_capacity(num_pairs);

        for i in 0..num_pairs {
            let name_idx = i * 2;
            let value_idx = i * 2 + 1;

            // Extract variable name from WrongVariableKind node
            // This is the same approach used for LAMBDA parameters
            let name = match &args[name_idx] {
                Node::WrongVariableKind(var_name) => var_name.clone(),
                _ => {
                    // Restore scope and return error
                    self.lambda_scope = old_scope;
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: format!(
                            "LET: argument {} must be a valid variable name",
                            name_idx + 1
                        ),
                    };
                }
            };

            // Evaluate the value expression (may reference previously bound variables)
            let value = self.evaluate_node_in_context(&args[value_idx], cell);
            
            // Propagate errors immediately
            if let CalcResult::Error { .. } = &value {
                self.lambda_scope = old_scope;
                return value;
            }

            // Bind the variable in scope
            self.lambda_scope.insert(name.clone(), value);
            bound_names.push(name);
        }

        // Evaluate the final calculation expression
        let result = self.evaluate_node_in_context(&args[arg_count - 1], cell);

        // Restore the original scope
        self.lambda_scope = old_scope;

        result
    }

    /// =XMATCH(lookup_value, lookup_array, [match_mode], [search_mode])
    pub(crate) fn fn_xmatch(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 2 || arg_count > 4 {
            return CalcResult::new_args_number_error(cell);
        }

        let lookup_value = self.evaluate_node_in_context(&args[0], cell);
        if let CalcResult::Error { .. } = &lookup_value {
            return lookup_value;
        }

        let lookup_array = self.evaluate_node_in_context(&args[1], cell);

        let array_data = match self.calc_result_to_flat_array(&lookup_array, cell) {
            Ok(data) => data,
            Err(e) => return e,
        };

        let match_mode = if arg_count >= 3 {
            match self.get_number(&args[2], cell) {
                Ok(v) => v as i32,
                Err(e) => return e,
            }
        } else {
            0
        };

        let search_mode = if arg_count >= 4 {
            match self.get_number(&args[3], cell) {
                Ok(v) => v as i32,
                Err(e) => return e,
            }
        } else {
            1
        };

        let lookup_node = self.calc_result_to_array_node(&lookup_value);

        let positions: Vec<usize> = if search_mode < 0 {
            (0..array_data.len()).rev().collect()
        } else {
            (0..array_data.len()).collect()
        };

        for pos in positions {
            let item = &array_data[pos];
            let matches = match match_mode {
                0 => Self::array_nodes_equal_static(&lookup_node, item),
                -1 => Self::compare_array_nodes_static(Some(&lookup_node), Some(item)) != std::cmp::Ordering::Less,
                1 => Self::compare_array_nodes_static(Some(&lookup_node), Some(item)) != std::cmp::Ordering::Greater,
                _ => Self::array_nodes_equal_static(&lookup_node, item),
            };

            if matches {
                return CalcResult::Number((pos + 1) as f64);
            }
        }

        CalcResult::Error {
            error: Error::NA,
            origin: cell,
            message: "XMATCH: no match found".to_string(),
        }
    }

    // Helper functions

    fn calc_result_to_2d_array(&mut self, val: &CalcResult, cell: CellReferenceIndex) -> Result<Vec<Vec<ArrayNode>>, CalcResult> {
        match val {
            CalcResult::Array(arr) => Ok(arr.clone()),
            CalcResult::Range { left, right } => {
                let mut data: Vec<Vec<ArrayNode>> = Vec::new();
                for row in left.row..=right.row {
                    let mut row_data: Vec<ArrayNode> = Vec::new();
                    for col in left.column..=right.column {
                        let cell_ref = CellReferenceIndex {
                            sheet: left.sheet,
                            row,
                            column: col,
                        };
                        let cell_val = self.evaluate_cell(cell_ref);
                        row_data.push(self.calc_result_to_array_node(&cell_val));
                    }
                    data.push(row_data);
                }
                Ok(data)
            }
            CalcResult::Number(n) => Ok(vec![vec![ArrayNode::Number(*n)]]),
            CalcResult::String(s) => Ok(vec![vec![ArrayNode::String(s.clone())]]),
            CalcResult::Boolean(b) => Ok(vec![vec![ArrayNode::Boolean(*b)]]),
            CalcResult::Error { error, origin, message } => {
                Err(CalcResult::Error {
                    error: error.clone(),
                    origin: *origin,
                    message: message.clone(),
                })
            }
            _ => Err(CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Invalid array argument".to_string(),
            }),
        }
    }

    fn calc_result_to_flat_array(&mut self, val: &CalcResult, cell: CellReferenceIndex) -> Result<Vec<ArrayNode>, CalcResult> {
        match val {
            CalcResult::Array(arr) => {
                Ok(arr.iter().flat_map(|row| row.iter().cloned()).collect())
            }
            CalcResult::Range { left, right } => {
                let mut data: Vec<ArrayNode> = Vec::new();
                for row in left.row..=right.row {
                    for col in left.column..=right.column {
                        let cell_ref = CellReferenceIndex {
                            sheet: left.sheet,
                            row,
                            column: col,
                        };
                        let cell_val = self.evaluate_cell(cell_ref);
                        data.push(self.calc_result_to_array_node(&cell_val));
                    }
                }
                Ok(data)
            }
            _ => Err(CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Expected array or range".to_string(),
            }),
        }
    }

    fn calc_result_to_array_node(&self, val: &CalcResult) -> ArrayNode {
        match val {
            CalcResult::Number(n) => ArrayNode::Number(*n),
            CalcResult::String(s) => ArrayNode::String(s.clone()),
            CalcResult::Boolean(b) => ArrayNode::Boolean(*b),
            CalcResult::Error { error, .. } => ArrayNode::Error(error.clone()),
            CalcResult::EmptyCell => ArrayNode::String(String::new()),
            CalcResult::EmptyArg => ArrayNode::String(String::new()),
            _ => ArrayNode::String(String::new()),
        }
    }

    fn array_rows_equal(&self, a: &[ArrayNode], b: &[ArrayNode]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(x, y)| Self::array_nodes_equal_static(x, y))
    }

    fn array_nodes_equal_static(a: &ArrayNode, b: &ArrayNode) -> bool {
        match (a, b) {
            (ArrayNode::Number(x), ArrayNode::Number(y)) => (x - y).abs() < 1e-10,
            (ArrayNode::String(x), ArrayNode::String(y)) => x.to_lowercase() == y.to_lowercase(),
            (ArrayNode::Boolean(x), ArrayNode::Boolean(y)) => x == y,
            (ArrayNode::Error(x), ArrayNode::Error(y)) => x == y,
            _ => false,
        }
    }

    fn compare_array_nodes_static(a: Option<&ArrayNode>, b: Option<&ArrayNode>) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        
        match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(ArrayNode::Number(x)), Some(ArrayNode::Number(y))) => {
                x.partial_cmp(y).unwrap_or(Ordering::Equal)
            }
            (Some(ArrayNode::String(x)), Some(ArrayNode::String(y))) => {
                x.to_lowercase().cmp(&y.to_lowercase())
            }
            (Some(ArrayNode::Number(_)), Some(ArrayNode::String(_))) => Ordering::Less,
            (Some(ArrayNode::String(_)), Some(ArrayNode::Number(_))) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    /// Transpose a 2D array (rows become columns and vice versa)
    fn transpose_2d(data: &[Vec<ArrayNode>]) -> Vec<Vec<ArrayNode>> {
        if data.is_empty() {
            return Vec::new();
        }
        let rows = data.len();
        let cols = data.first().map(|r| r.len()).unwrap_or(0);
        
        let mut result: Vec<Vec<ArrayNode>> = Vec::with_capacity(cols);
        for c in 0..cols {
            let mut new_row: Vec<ArrayNode> = Vec::with_capacity(rows);
            for r in 0..rows {
                if let Some(row) = data.get(r) {
                    if let Some(val) = row.get(c) {
                        new_row.push(val.clone());
                    }
                }
            }
            result.push(new_row);
        }
        result
    }
}
