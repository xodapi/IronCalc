use crate::{
    calc_result::CalcResult,
    constants::{LAST_COLUMN, LAST_ROW},
    expressions::{parser::Node, token::Error, types::CellReferenceIndex},
    formatter::format::{format_number, parse_formatted_number},
    model::Model,
    number_format::to_precision,
};

use super::{
    text_util::{substitute, text_after, text_before, Case},
    util::from_wildcard_to_regex,
};

/// Finds the first instance of 'search_for' in text starting at char index start
fn find(search_for: &str, text: &str, start: usize) -> Option<i32> {
    let ch = text.chars();
    let mut byte_index = 0;
    for (char_index, c) in ch.enumerate() {
        if char_index + 1 >= start && text[byte_index..].starts_with(search_for) {
            return Some((char_index + 1) as i32);
        }
        byte_index += c.len_utf8();
    }
    None
}

/// You can use the wildcard characters — the question mark (?) and asterisk (*) — in the find_text argument.
/// * A question mark matches any single character.
/// * An asterisk matches any sequence of characters.
/// * If you want to find an actual question mark or asterisk, type a tilde (~) before the character.
fn search(search_for: &str, text: &str, start: usize) -> Option<i32> {
    let re = match from_wildcard_to_regex(search_for, false) {
        Ok(r) => r,
        Err(_) => return None,
    };

    let ch = text.chars();
    let mut byte_index = 0;
    for (char_index, c) in ch.enumerate() {
        if char_index + 1 >= start {
            if let Some(m) = re.find(&text[byte_index..]) {
                return Some((text[0..(m.start() + byte_index)].chars().count() as i32) + 1);
            } else {
                return None;
            }
        }
        byte_index += c.len_utf8();
    }
    None
}

impl Model {
    pub(crate) fn fn_concat(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let mut result = "".to_string();
        for arg in args {
            match self.evaluate_node_in_context(arg, cell) {
                CalcResult::String(value) => result = format!("{result}{value}"),
                CalcResult::Number(value) => result = format!("{result}{value}"),
                CalcResult::EmptyCell | CalcResult::EmptyArg => {}
                CalcResult::Boolean(value) => {
                    if value {
                        result = format!("{result}TRUE");
                    } else {
                        result = format!("{result}FALSE");
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { left, right } => {
                    if left.sheet != right.sheet {
                        return CalcResult::new_error(
                            Error::VALUE,
                            cell,
                            "Ranges are in different sheets".to_string(),
                        );
                    }
                    for row in left.row..(right.row + 1) {
                        for column in left.column..(right.column + 1) {
                            match self.evaluate_cell(CellReferenceIndex {
                                sheet: left.sheet,
                                row,
                                column,
                            }) {
                                CalcResult::String(value) => {
                                    result = format!("{result}{value}");
                                }
                                CalcResult::Number(value) => result = format!("{result}{value}"),
                                CalcResult::Boolean(value) => {
                                    if value {
                                        result = format!("{result}TRUE");
                                    } else {
                                        result = format!("{result}FALSE");
                                    }
                                }
                                error @ CalcResult::Error { .. } => return error,
                                CalcResult::EmptyCell | CalcResult::EmptyArg => {}
                                CalcResult::Range { .. } => {}
                                CalcResult::Array(_) => {
                                    return CalcResult::Error {
                                        error: Error::NIMPL,
                                        origin: cell,
                                        message: "Arrays not supported yet".to_string(),
                                    }
                                }
                                CalcResult::Lambda(_) => {
                                    return CalcResult::new_error(
                                        Error::VALUE,
                                        cell,
                                        "Cannot use LAMBDA as text value".to_string(),
                                    )
                                }
                            }
                        }
                    }
                }
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
        }
        CalcResult::String(result)
    }
    pub(crate) fn fn_text(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 2 {
            let value = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(f) => f,
                CalcResult::String(s) => {
                    return CalcResult::String(s);
                }
                CalcResult::Boolean(b) => {
                    return CalcResult::Boolean(b);
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => 0.0,
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
            let format_code = match self.get_string(&args[1], cell) {
                Ok(s) => s,
                Err(s) => return s,
            };
            let d = format_number(value, &format_code, &self.locale);
            if let Some(_e) = d.error {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Invalid format code".to_string(),
                };
            }
            CalcResult::String(d.text)
        } else {
            CalcResult::new_args_number_error(cell)
        }
    }

    /// FIND(find_text, within_text, [start_num])
    ///  * FIND and FINDB are case sensitive and don't allow wildcard characters.
    ///  * If find_text is "" (empty text), FIND matches the first character in the search string (that is, the character numbered start_num or 1).
    ///  * Find_text cannot contain any wildcard characters.
    ///  * If find_text does not appear in within_text, FIND and FINDB return the #VALUE! error value.
    ///  * If start_num is not greater than zero, FIND and FINDB return the #VALUE! error value.
    ///  * If start_num is greater than the length of within_text, FIND and FINDB return the #VALUE! error value.
    ///    NB: FINDB is not implemented. It is the same as FIND function unless locale is a DBCS (Double Byte Character Set)
    pub(crate) fn fn_find(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 || args.len() > 3 {
            return CalcResult::new_args_number_error(cell);
        }
        let find_text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(s) => return s,
        };
        let within_text = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(s) => return s,
        };
        let start_num = if args.len() == 3 {
            match self.get_number(&args[2], cell) {
                Ok(s) => s.floor(),
                Err(s) => return s,
            }
        } else {
            1.0
        };

        if start_num < 1.0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Start num must be >= 1".to_string(),
            };
        }
        let start_num = start_num as usize;

        if start_num > within_text.len() {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Start num greater than length".to_string(),
            };
        }
        if let Some(s) = find(&find_text, &within_text, start_num) {
            CalcResult::Number(s as f64)
        } else {
            CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Text not found".to_string(),
            }
        }
    }

    /// Same API as FIND but:
    ///  * Allows wildcards
    ///  * It is case insensitive
    ///    SEARCH(find_text, within_text, [start_num])
    pub(crate) fn fn_search(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 || args.len() > 3 {
            return CalcResult::new_args_number_error(cell);
        }
        let find_text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(s) => return s,
        };
        let within_text = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(s) => return s,
        };
        let start_num = if args.len() == 3 {
            match self.get_number(&args[2], cell) {
                Ok(s) => s.floor(),
                Err(s) => return s,
            }
        } else {
            1.0
        };

        if start_num < 1.0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Start num must be >= 1".to_string(),
            };
        }
        let start_num = start_num as usize;

        if start_num > within_text.len() {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Start num greater than length".to_string(),
            };
        }
        // SEARCH is case insensitive
        if let Some(s) = search(
            &find_text.to_lowercase(),
            &within_text.to_lowercase(),
            start_num,
        ) {
            CalcResult::Number(s as f64)
        } else {
            CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Text not found".to_string(),
            }
        }
    }

    // LEN, LEFT, RIGHT, MID, LOWER, UPPER, TRIM
    pub(crate) fn fn_len(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 1 {
            let s = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(v) => format!("{v}"),
                CalcResult::String(v) => v,
                CalcResult::Boolean(b) => {
                    if b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
            return CalcResult::Number(s.chars().count() as f64);
        }
        CalcResult::new_args_number_error(cell)
    }

    pub(crate) fn fn_trim(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 1 {
            let s = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(v) => format!("{v}"),
                CalcResult::String(v) => v,
                CalcResult::Boolean(b) => {
                    if b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
            return CalcResult::String(s.trim().to_owned());
        }
        CalcResult::new_args_number_error(cell)
    }

    pub(crate) fn fn_lower(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 1 {
            let s = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(v) => format!("{v}"),
                CalcResult::String(v) => v,
                CalcResult::Boolean(b) => {
                    if b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
            return CalcResult::String(s.to_lowercase());
        }
        CalcResult::new_args_number_error(cell)
    }

    pub(crate) fn fn_unicode(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 1 {
            let s = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(v) => format!("{v}"),
                CalcResult::String(v) => v,
                CalcResult::Boolean(b) => {
                    if b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Empty cell".to_string(),
                    }
                }
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };

            match s.chars().next() {
                Some(c) => {
                    let unicode_number = c as u32;
                    return CalcResult::Number(unicode_number as f64);
                }
                None => {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Empty cell".to_string(),
                    };
                }
            }
        }
        CalcResult::new_args_number_error(cell)
    }

    pub(crate) fn fn_upper(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() == 1 {
            let s = match self.evaluate_node_in_context(&args[0], cell) {
                CalcResult::Number(v) => format!("{v}"),
                CalcResult::String(v) => v,
                CalcResult::Boolean(b) => {
                    if b {
                        "TRUE".to_string()
                    } else {
                        "FALSE".to_string()
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
            return CalcResult::String(s.to_uppercase());
        }
        CalcResult::new_args_number_error(cell)
    }

    pub(crate) fn fn_left(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() > 2 || args.is_empty() {
            return CalcResult::new_args_number_error(cell);
        }
        let s = match self.evaluate_node_in_context(&args[0], cell) {
            CalcResult::Number(v) => format!("{v}"),
            CalcResult::String(v) => v,
            CalcResult::Boolean(b) => {
                if b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            error @ CalcResult::Error { .. } => return error,
            CalcResult::Range { .. } => {
                // Implicit Intersection not implemented
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Implicit Intersection not implemented".to_string(),
                };
            }
            CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
            CalcResult::Array(_) => {
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Arrays not supported yet".to_string(),
                }
            }
            CalcResult::Lambda(_) => {
                return CalcResult::new_error(
                    Error::VALUE,
                    cell,
                    "Cannot use LAMBDA as text value".to_string(),
                )
            }
        };
        let num_chars = if args.len() == 2 {
            match self.evaluate_node_in_context(&args[1], cell) {
                CalcResult::Number(v) => {
                    if v < 0.0 {
                        return CalcResult::Error {
                            error: Error::VALUE,
                            origin: cell,
                            message: "Number must be >= 0".to_string(),
                        };
                    }
                    v.floor() as usize
                }
                CalcResult::Boolean(_) | CalcResult::String(_) => {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Expecting number".to_string(),
                    };
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => 0,
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as number".to_string(),
                    )
                }
            }
        } else {
            1
        };
        let mut result = "".to_string();
        for (index, ch) in s.chars().enumerate() {
            if index >= num_chars {
                break;
            }
            result.push(ch);
        }
        CalcResult::String(result)
    }

    pub(crate) fn fn_right(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() > 2 || args.is_empty() {
            return CalcResult::new_args_number_error(cell);
        }
        let s = match self.evaluate_node_in_context(&args[0], cell) {
            CalcResult::Number(v) => format!("{v}"),
            CalcResult::String(v) => v,
            CalcResult::Boolean(b) => {
                if b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            error @ CalcResult::Error { .. } => return error,
            CalcResult::Range { .. } => {
                // Implicit Intersection not implemented
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Implicit Intersection not implemented".to_string(),
                };
            }
            CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
            CalcResult::Array(_) => {
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Arrays not supported yet".to_string(),
                }
            }
            CalcResult::Lambda(_) => {
                return CalcResult::new_error(
                    Error::VALUE,
                    cell,
                    "Cannot use LAMBDA as text value".to_string(),
                )
            }
        };
        let num_chars = if args.len() == 2 {
            match self.evaluate_node_in_context(&args[1], cell) {
                CalcResult::Number(v) => {
                    if v < 0.0 {
                        return CalcResult::Error {
                            error: Error::VALUE,
                            origin: cell,
                            message: "Number must be >= 0".to_string(),
                        };
                    }
                    v.floor() as usize
                }
                CalcResult::Boolean(_) | CalcResult::String(_) => {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Expecting number".to_string(),
                    };
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::Range { .. } => {
                    // Implicit Intersection not implemented
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Implicit Intersection not implemented".to_string(),
                    };
                }
                CalcResult::EmptyCell | CalcResult::EmptyArg => 0,
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as number".to_string(),
                    )
                }
            }
        } else {
            1
        };
        let mut result = "".to_string();
        for (index, ch) in s.chars().rev().enumerate() {
            if index >= num_chars {
                break;
            }
            result.push(ch);
        }
        CalcResult::String(result.chars().rev().collect::<String>())
    }

    pub(crate) fn fn_mid(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 3 {
            return CalcResult::new_args_number_error(cell);
        }
        let s = match self.evaluate_node_in_context(&args[0], cell) {
            CalcResult::Number(v) => format!("{v}"),
            CalcResult::String(v) => v,
            CalcResult::Boolean(b) => {
                if b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            error @ CalcResult::Error { .. } => return error,
            CalcResult::Range { .. } => {
                // Implicit Intersection not implemented
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Implicit Intersection not implemented".to_string(),
                };
            }
            CalcResult::EmptyCell | CalcResult::EmptyArg => "".to_string(),
            CalcResult::Array(_) => {
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Arrays not supported yet".to_string(),
                }
            }
            CalcResult::Lambda(_) => {
                return CalcResult::new_error(
                    Error::VALUE,
                    cell,
                    "Cannot use LAMBDA as text value".to_string(),
                )
            }
        };
        let start_num = match self.evaluate_node_in_context(&args[1], cell) {
            CalcResult::Number(v) => {
                if v < 1.0 {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Number must be >= 1".to_string(),
                    };
                }
                v.floor() as usize
            }
            error @ CalcResult::Error { .. } => return error,
            CalcResult::Range { .. } => {
                // Implicit Intersection not implemented
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Implicit Intersection not implemented".to_string(),
                };
            }
            _ => {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Expecting number".to_string(),
                };
            }
        };
        let num_chars = match self.evaluate_node_in_context(&args[2], cell) {
            CalcResult::Number(v) => {
                if v < 0.0 {
                    return CalcResult::Error {
                        error: Error::VALUE,
                        origin: cell,
                        message: "Number must be >= 0".to_string(),
                    };
                }
                v.floor() as usize
            }
            CalcResult::String(_) => {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Expecting number".to_string(),
                };
            }
            CalcResult::Boolean(_) => {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Expecting number".to_string(),
                }
            }
            error @ CalcResult::Error { .. } => return error,
            CalcResult::Range { .. } => {
                // Implicit Intersection not implemented
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Implicit Intersection not implemented".to_string(),
                };
            }
            CalcResult::EmptyCell | CalcResult::EmptyArg => 0,
            CalcResult::Array(_) => {
                return CalcResult::Error {
                    error: Error::NIMPL,
                    origin: cell,
                    message: "Arrays not supported yet".to_string(),
                }
            }
            CalcResult::Lambda(_) => {
                return CalcResult::new_error(
                    Error::VALUE,
                    cell,
                    "Cannot use LAMBDA as number".to_string(),
                )
            }
        };
        let mut result = "".to_string();
        let mut count: usize = 0;
        for (index, ch) in s.chars().enumerate() {
            if count >= num_chars {
                break;
            }
            if index + 1 >= start_num {
                result.push(ch);
                count += 1;
            }
        }
        CalcResult::String(result)
    }

    // REPT(text, number_times)
    pub(crate) fn fn_rept(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 2 {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let number_times = match self.get_number(&args[1], cell) {
            Ok(f) => f.floor() as i32,
            Err(s) => return s,
        };
        let text_len = text.len() as i32;

        // We normally don't follow Excel's sometimes archaic size's restrictions
        // But this might be a security issue
        if text_len * number_times > 32767 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "number times too high".to_string(),
            };
        }
        if number_times < 0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "number times too high".to_string(),
            };
        }
        if number_times == 0 {
            return CalcResult::String("".to_string());
        }
        CalcResult::String(text.repeat(number_times as usize))
    }

    // TEXTAFTER(text, delimiter, [instance_num], [match_mode], [match_end], [if_not_found])
    pub(crate) fn fn_textafter(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if !(2..=6).contains(&arg_count) {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let delimiter = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let instance_num = if arg_count > 2 {
            match self.get_number(&args[2], cell) {
                Ok(f) => f.floor() as i32,
                Err(s) => return s,
            }
        } else {
            1
        };
        let match_mode = if arg_count > 3 {
            match self.get_number(&args[3], cell) {
                Ok(f) => {
                    if f == 0.0 {
                        Case::Sensitive
                    } else {
                        Case::Insensitive
                    }
                }
                Err(s) => return s,
            }
        } else {
            Case::Sensitive
        };

        let match_end = if arg_count > 4 {
            match self.get_number(&args[4], cell) {
                Ok(f) => f,
                Err(s) => return s,
            }
        } else {
            // disabled by default
            // the delimiter is specified in the formula
            0.0
        };
        if instance_num == 0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "instance_num must be <> 0".to_string(),
            };
        }
        if delimiter.len() > text.len() {
            // so this is fun(!)
            // if the function was provided with two arguments is a #VALUE!
            // if it had more is a #N/A (irrespective of their values)
            if arg_count > 2 {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "The delimiter is longer than the text is trying to match".to_string(),
                };
            } else {
                return CalcResult::Error {
                    error: Error::NA,
                    origin: cell,
                    message: "The delimiter is longer than the text is trying to match".to_string(),
                };
            }
        }
        if match_end != 0.0 && match_end != 1.0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "argument must be 0 or 1".to_string(),
            };
        };
        match text_after(&text, &delimiter, instance_num, match_mode) {
            Some(s) => CalcResult::String(s),
            None => {
                if match_end == 1.0 {
                    if instance_num == 1 {
                        return CalcResult::String("".to_string());
                    } else if instance_num == -1 {
                        return CalcResult::String(text);
                    }
                }
                if arg_count == 6 {
                    // An empty cell is converted to empty string (not 0)
                    match self.evaluate_node_in_context(&args[5], cell) {
                        CalcResult::EmptyCell => CalcResult::String("".to_string()),
                        result => result,
                    }
                } else {
                    CalcResult::Error {
                        error: Error::NA,
                        origin: cell,
                        message: "Value not found".to_string(),
                    }
                }
            }
        }
    }

    pub(crate) fn fn_textbefore(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if !(2..=6).contains(&arg_count) {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let delimiter = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let instance_num = if arg_count > 2 {
            match self.get_number(&args[2], cell) {
                Ok(f) => f.floor() as i32,
                Err(s) => return s,
            }
        } else {
            1
        };
        let match_mode = if arg_count > 3 {
            match self.get_number(&args[3], cell) {
                Ok(f) => {
                    if f == 0.0 {
                        Case::Sensitive
                    } else {
                        Case::Insensitive
                    }
                }
                Err(s) => return s,
            }
        } else {
            Case::Sensitive
        };

        let match_end = if arg_count > 4 {
            match self.get_number(&args[4], cell) {
                Ok(f) => f,
                Err(s) => return s,
            }
        } else {
            // disabled by default
            // the delimiter is specified in the formula
            0.0
        };
        if instance_num == 0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "instance_num must be <> 0".to_string(),
            };
        }
        if delimiter.len() > text.len() {
            // so this is fun(!)
            // if the function was provided with two arguments is a #VALUE!
            // if it had more is a #N/A (irrespective of their values)
            if arg_count > 2 {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "The delimiter is longer than the text is trying to match".to_string(),
                };
            } else {
                return CalcResult::Error {
                    error: Error::NA,
                    origin: cell,
                    message: "The delimiter is longer than the text is trying to match".to_string(),
                };
            }
        }
        if match_end != 0.0 && match_end != 1.0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "argument must be 0 or 1".to_string(),
            };
        };
        match text_before(&text, &delimiter, instance_num, match_mode) {
            Some(s) => CalcResult::String(s),
            None => {
                if match_end == 1.0 {
                    if instance_num == -1 {
                        return CalcResult::String("".to_string());
                    } else if instance_num == 1 {
                        return CalcResult::String(text);
                    }
                }
                if arg_count == 6 {
                    // An empty cell is converted to empty string (not 0)
                    match self.evaluate_node_in_context(&args[5], cell) {
                        CalcResult::EmptyCell => CalcResult::String("".to_string()),
                        result => result,
                    }
                } else {
                    CalcResult::Error {
                        error: Error::NA,
                        origin: cell,
                        message: "Value not found".to_string(),
                    }
                }
            }
        }
    }

    // TEXTJOIN(delimiter, ignore_empty, text1, [text2], …)
    pub(crate) fn fn_textjoin(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count < 3 {
            return CalcResult::new_args_number_error(cell);
        }
        let delimiter = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let ignore_empty = match self.get_boolean(&args[1], cell) {
            Ok(b) => b,
            Err(error) => return error,
        };
        let mut values = Vec::new();
        for arg in &args[2..] {
            match self.evaluate_node_in_context(arg, cell) {
                CalcResult::Number(value) => values.push(format!("{value}")),
                CalcResult::Range { left, right } => {
                    if left.sheet != right.sheet {
                        return CalcResult::new_error(
                            Error::VALUE,
                            cell,
                            "Ranges are in different sheets".to_string(),
                        );
                    }
                    let row1 = left.row;
                    let mut row2 = right.row;
                    let column1 = left.column;
                    let mut column2 = right.column;
                    if row1 == 1 && row2 == LAST_ROW {
                        row2 = match self.workbook.worksheet(left.sheet) {
                            Ok(s) => s.dimension().max_row,
                            Err(_) => {
                                return CalcResult::new_error(
                                    Error::ERROR,
                                    cell,
                                    format!("Invalid worksheet index: '{}'", left.sheet),
                                );
                            }
                        };
                    }
                    if column1 == 1 && column2 == LAST_COLUMN {
                        column2 = match self.workbook.worksheet(left.sheet) {
                            Ok(s) => s.dimension().max_column,
                            Err(_) => {
                                return CalcResult::new_error(
                                    Error::ERROR,
                                    cell,
                                    format!("Invalid worksheet index: '{}'", left.sheet),
                                );
                            }
                        };
                    }
                    for row in row1..row2 + 1 {
                        for column in column1..(column2 + 1) {
                            match self.evaluate_cell(CellReferenceIndex {
                                sheet: left.sheet,
                                row,
                                column,
                            }) {
                                CalcResult::Number(value) => {
                                    values.push(format!("{value}"));
                                }
                                CalcResult::String(value) => values.push(value),
                                CalcResult::Boolean(value) => {
                                    if value {
                                        values.push("TRUE".to_string())
                                    } else {
                                        values.push("FALSE".to_string())
                                    }
                                }
                                CalcResult::EmptyCell => {
                                    if !ignore_empty {
                                        values.push("".to_string())
                                    }
                                }
                                error @ CalcResult::Error { .. } => return error,
                                CalcResult::EmptyArg | CalcResult::Range { .. } => {}
                                CalcResult::Array(_) => {
                                    return CalcResult::Error {
                                        error: Error::NIMPL,
                                        origin: cell,
                                        message: "Arrays not supported yet".to_string(),
                                    }
                                }
                                CalcResult::Lambda(_) => {
                                    return CalcResult::new_error(
                                        Error::VALUE,
                                        cell,
                                        "Cannot use LAMBDA as text value".to_string(),
                                    )
                                }
                            }
                        }
                    }
                }
                error @ CalcResult::Error { .. } => return error,
                CalcResult::String(value) => values.push(value),
                CalcResult::Boolean(value) => {
                    if value {
                        values.push("TRUE".to_string())
                    } else {
                        values.push("FALSE".to_string())
                    }
                }
                CalcResult::EmptyCell => {
                    if !ignore_empty {
                        values.push("".to_string())
                    }
                }
                CalcResult::EmptyArg => {}
                CalcResult::Array(_) => {
                    return CalcResult::Error {
                        error: Error::NIMPL,
                        origin: cell,
                        message: "Arrays not supported yet".to_string(),
                    }
                }
                CalcResult::Lambda(_) => {
                    return CalcResult::new_error(
                        Error::VALUE,
                        cell,
                        "Cannot use LAMBDA as text value".to_string(),
                    )
                }
            };
        }
        let result = values.join(&delimiter);
        CalcResult::String(result)
    }

    // SUBSTITUTE(text, old_text, new_text, [instance_num])
    pub(crate) fn fn_substitute(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if !(2..=4).contains(&arg_count) {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let old_text = match self.get_string(&args[1], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let new_text = match self.get_string(&args[2], cell) {
            Ok(s) => s,
            Err(error) => return error,
        };
        let instance_num = if arg_count > 3 {
            match self.get_number(&args[3], cell) {
                Ok(f) => Some(f.floor() as i32),
                Err(s) => return s,
            }
        } else {
            // means every instance is replaced
            None
        };
        if let Some(num) = instance_num {
            if num < 1 {
                return CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Invalid value".to_string(),
                };
            }
            if old_text.is_empty() {
                return CalcResult::String(text);
            }
            CalcResult::String(substitute(&text, &old_text, &new_text, num))
        } else {
            if old_text.is_empty() {
                return CalcResult::String(text);
            }
            CalcResult::String(text.replace(&old_text, &new_text))
        }
    }
    pub(crate) fn fn_concatenate(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        let arg_count = args.len();
        if arg_count == 0 {
            return CalcResult::new_args_number_error(cell);
        }
        let mut text_array = Vec::new();
        for arg in args {
            let text = match self.get_string(arg, cell) {
                Ok(s) => s,
                Err(error) => return error,
            };
            text_array.push(text)
        }
        CalcResult::String(text_array.join(""))
    }

    pub(crate) fn fn_exact(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 2 {
            return CalcResult::new_args_number_error(cell);
        }
        let result1 = &self.evaluate_node_in_context(&args[0], cell);
        let result2 = &self.evaluate_node_in_context(&args[1], cell);
        // FIXME: Implicit intersection
        if let (CalcResult::Number(number1), CalcResult::Number(number2)) = (result1, result2) {
            // In Excel two numbers are the same if they are the same up to 15 digits.
            CalcResult::Boolean(to_precision(*number1, 15) == to_precision(*number2, 15))
        } else {
            let string1 = match self.cast_to_string(result1.clone(), cell) {
                Ok(s) => s,
                Err(error) => return error,
            };
            let string2 = match self.cast_to_string(result2.clone(), cell) {
                Ok(s) => s,
                Err(error) => return error,
            };
            CalcResult::Boolean(string1 == string2)
        }
    }
    // VALUE(text)
    pub(crate) fn fn_value(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        match self.evaluate_node_in_context(&args[0], cell) {
            CalcResult::String(text) => {
                let currencies = vec!["$", "€"];
                if let Ok((value, _)) = parse_formatted_number(&text, &currencies) {
                    return CalcResult::Number(value);
                };
                CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Invalid number".to_string(),
                }
            }
            CalcResult::Number(f) => CalcResult::Number(f),
            CalcResult::Boolean(_) => CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Invalid number".to_string(),
            },
            error @ CalcResult::Error { .. } => error,
            CalcResult::Range { .. } => {
                // TODO Implicit Intersection
                CalcResult::Error {
                    error: Error::VALUE,
                    origin: cell,
                    message: "Invalid number".to_string(),
                }
            }
            CalcResult::EmptyCell | CalcResult::EmptyArg => CalcResult::Number(0.0),
            CalcResult::Array(_) => CalcResult::Error {
                error: Error::NIMPL,
                origin: cell,
                message: "Arrays not supported yet".to_string(),
            },
            CalcResult::Lambda(_) => CalcResult::new_error(
                Error::VALUE,
                cell,
                "Cannot use LAMBDA as number".to_string(),
            ),
        }
    }

    pub(crate) fn fn_t(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        // FIXME: Implicit intersection
        let result = self.evaluate_node_in_context(&args[0], cell);
        match result {
            CalcResult::String(_) => result,
            error @ CalcResult::Error { .. } => error,
            _ => CalcResult::String("".to_string()),
        }
    }

    // VALUETOTEXT(value)
    pub(crate) fn fn_valuetotext(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(error) => match error {
                CalcResult::Error { error, .. } => error.to_string(),
                _ => "".to_string(),
            },
        };
        CalcResult::String(text)
    }

    /// CHAR(number)
    /// Returns the character specified by the code number (1-255 for ASCII/ANSI)
    pub(crate) fn fn_char(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let number = match self.get_number(&args[0], cell) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let code = number.floor() as i64;
        if code < 1 || code > 255 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Number must be between 1 and 255".to_string(),
            };
        }
        CalcResult::String((code as u8 as char).to_string())
    }

    /// CODE(text)
    /// Returns a numeric code for the first character in a text string (ANSI code)
    pub(crate) fn fn_code(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        if text.is_empty() {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Text cannot be empty".to_string(),
            };
        }
        let first_char = text.chars().next().unwrap();
        // Return ASCII/ANSI code (0-255 range)
        let code = first_char as u32;
        if code > 255 {
            // For non-ASCII, return the Unicode code point (like Excel does for extended chars)
            CalcResult::Number(code as f64)
        } else {
            CalcResult::Number(code as f64)
        }
    }

    /// UNICHAR(number)
    /// Returns the Unicode character referenced by the given numeric value
    pub(crate) fn fn_unichar(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let number = match self.get_number(&args[0], cell) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let code = number.floor() as u32;
        if code < 1 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Number must be greater than 0".to_string(),
            };
        }
        match char::from_u32(code) {
            Some(c) => CalcResult::String(c.to_string()),
            None => CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Invalid Unicode code point".to_string(),
            },
        }
    }

    /// CLEAN(text)
    /// Removes all nonprintable characters from text (ASCII codes 0-31)
    pub(crate) fn fn_clean(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        // Remove characters with ASCII codes 0-31 (non-printable)
        let cleaned: String = text
            .chars()
            .filter(|c| {
                let code = *c as u32;
                code > 31
            })
            .collect();
        CalcResult::String(cleaned)
    }

    /// PROPER(text)
    /// Capitalizes the first letter in each word of a text value
    pub(crate) fn fn_proper(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 1 {
            return CalcResult::new_args_number_error(cell);
        }
        let text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        let mut result = String::with_capacity(text.len());
        let mut capitalize_next = true;
        for c in text.chars() {
            if c.is_alphabetic() {
                if capitalize_next {
                    result.extend(c.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.extend(c.to_lowercase());
                }
            } else {
                result.push(c);
                // Non-alphabetic chars trigger capitalization of next letter
                capitalize_next = !c.is_alphanumeric();
            }
        }
        CalcResult::String(result)
    }

    /// REPLACE(old_text, start_num, num_chars, new_text)
    /// Replaces part of a text string with a different text string
    pub(crate) fn fn_replace(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() != 4 {
            return CalcResult::new_args_number_error(cell);
        }
        let old_text = match self.get_string(&args[0], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };
        let start_num = match self.get_number(&args[1], cell) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let num_chars = match self.get_number(&args[2], cell) {
            Ok(n) => n,
            Err(e) => return e,
        };
        let new_text = match self.get_string(&args[3], cell) {
            Ok(s) => s,
            Err(e) => return e,
        };

        let start = start_num.floor() as i64;
        let count = num_chars.floor() as i64;

        if start < 1 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Start position must be >= 1".to_string(),
            };
        }
        if count < 0 {
            return CalcResult::Error {
                error: Error::VALUE,
                origin: cell,
                message: "Number of characters must be >= 0".to_string(),
            };
        }

        let start = (start - 1) as usize; // Convert to 0-indexed
        let count = count as usize;

        let chars: Vec<char> = old_text.chars().collect();
        let mut result = String::new();

        // Add characters before start position
        for c in chars.iter().take(start.min(chars.len())) {
            result.push(*c);
        }
        // Add new text
        result.push_str(&new_text);
        // Add characters after the replaced section
        let skip_until = start + count;
        for c in chars.iter().skip(skip_until) {
            result.push(*c);
        }

        CalcResult::String(result)
    }
}
