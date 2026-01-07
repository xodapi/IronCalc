// Phase 1: Missing Text Functions (14 new functions)
// FIXED, DOLLAR, NUMBERVALUE, BAHTTEXT, ASC, DBCS, JIS, LEFTB, LENB, MIDB, RIGHTB, FINDB, SEARCHB, REPLACEB

use crate::calc_result::CalcResult;
use crate::expressions::parser::Node;
use crate::expressions::token::Error;
use crate::expressions::types::CellReferenceIndex;
use crate::model::Model;

impl Model {
    /// FIXED(number, [decimals], [no_commas])
    pub(crate) fn fn_fixed(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let number = match self.get_number(&args[0], cell) { Ok(n) => n, Err(e) => return e };
        let decimals = if args.len() > 1 {
            match self.get_number(&args[1], cell) { Ok(n) => n as i32, Err(e) => return e }
        } else { 2 };
        let no_commas = if args.len() > 2 {
            match self.get_boolean(&args[2], cell) { Ok(b) => b, Err(e) => return e }
        } else { false };
        let decimals = decimals.max(0) as usize;
        let factor = 10f64.powi(decimals as i32);
        let rounded = (number * factor).round() / factor;
        let formatted = if decimals > 0 { format!("{:.prec$}", rounded, prec = decimals) }
                        else { format!("{}", rounded as i64) };
        let result = if no_commas { formatted } else {
            let parts: Vec<&str> = formatted.split('.').collect();
            let integer_part = parts[0];
            let decimal_part = parts.get(1);
            let is_negative = integer_part.starts_with('-');
            let digits: String = integer_part.chars().filter(|c| c.is_ascii_digit()).collect();
            let with_commas: String = digits.chars().rev().collect::<Vec<_>>().chunks(3)
                .map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join(",").chars().rev().collect();
            let signed = if is_negative { format!("-{}", with_commas) } else { with_commas };
            if let Some(dec) = decimal_part { format!("{}.{}", signed, dec) } else { signed }
        };
        CalcResult::String(result)
    }

    /// DOLLAR(number, [decimals])
    pub(crate) fn fn_dollar(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let number = match self.get_number(&args[0], cell) { Ok(n) => n, Err(e) => return e };
        let decimals = if args.len() > 1 {
            match self.get_number(&args[1], cell) { Ok(n) => n as i32, Err(e) => return e }
        } else { 2 };
        let decimals = decimals.max(0) as usize;
        let factor = 10f64.powi(decimals as i32);
        let rounded = (number.abs() * factor).round() / factor;
        let formatted = if decimals > 0 { format!("{:.prec$}", rounded, prec = decimals) }
                        else { format!("{}", rounded as i64) };
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let digits: String = integer_part.chars().filter(|c| c.is_ascii_digit()).collect();
        let with_commas: String = digits.chars().rev().collect::<Vec<_>>().chunks(3)
            .map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join(",").chars().rev().collect();
        let amount = if let Some(dec) = parts.get(1) { format!("{}.{}", with_commas, dec) } else { with_commas };
        let result = if number < 0.0 { format!("(${})", amount) } else { format!("${}", amount) };
        CalcResult::String(result)
    }

    /// NUMBERVALUE(text, [decimal_separator], [group_separator])
    pub(crate) fn fn_numbervalue(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s.trim().to_string(), Err(e) => return e };
        let decimal_sep = if args.len() > 1 {
            match self.get_string(&args[1], cell) { Ok(s) => s.chars().next().unwrap_or('.'), Err(e) => return e }
        } else { '.' };
        let group_sep = if args.len() > 2 {
            match self.get_string(&args[2], cell) { Ok(s) => s.chars().next(), Err(e) => return e }
        } else { None };
        let mut cleaned = text.clone();
        if let Some(sep) = group_sep { cleaned = cleaned.replace(sep, ""); }
        if decimal_sep != '.' { cleaned = cleaned.replace(decimal_sep, "."); }
        let is_percent = cleaned.ends_with('%');
        if is_percent { cleaned = cleaned.trim_end_matches('%').to_string(); }
        match cleaned.parse::<f64>() {
            Ok(n) => CalcResult::Number(if is_percent { n / 100.0 } else { n }),
            Err(_) => CalcResult::new_error(Error::VALUE, cell, "NUMBERVALUE: Invalid".to_string()),
        }
    }

    /// BAHTTEXT(number) - stub
    pub(crate) fn fn_bahttext(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        CalcResult::String("BAHTTEXT not implemented".to_string())
    }

    /// ASC(text)
    pub(crate) fn fn_asc(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let result: String = text.chars().map(|c| {
            let code = c as u32;
            if (0xFF01..=0xFF5E).contains(&code) { char::from_u32(code - 0xFEE0).unwrap_or(c) }
            else if code == 0x3000 { ' ' } else { c }
        }).collect();
        CalcResult::String(result)
    }

    /// DBCS(text)
    pub(crate) fn fn_dbcs(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        self.fn_jis(args, cell)
    }

    /// JIS(text)
    pub(crate) fn fn_jis(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let result: String = text.chars().map(|c| {
            let code = c as u32;
            if (0x0021..=0x007E).contains(&code) { char::from_u32(code + 0xFEE0).unwrap_or(c) }
            else if c == ' ' { '\u{3000}' } else { c }
        }).collect();
        CalcResult::String(result)
    }

    /// LEFTB(text, [num_bytes])
    pub(crate) fn fn_leftb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let num_bytes = if args.len() > 1 {
            match self.get_number(&args[1], cell) { Ok(n) => n as usize, Err(e) => return e }
        } else { 1 };
        CalcResult::String(text.chars().take(num_bytes).collect())
    }

    /// LENB(text)
    pub(crate) fn fn_lenb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        CalcResult::Number(text.len() as f64)
    }

    /// MIDB(text, start_num, num_bytes)
    pub(crate) fn fn_midb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 3 { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let start = match self.get_number(&args[1], cell) { Ok(n) => (n as usize).saturating_sub(1), Err(e) => return e };
        let num_bytes = match self.get_number(&args[2], cell) { Ok(n) => n as usize, Err(e) => return e };
        CalcResult::String(text.chars().skip(start).take(num_bytes).collect())
    }

    /// RIGHTB(text, [num_bytes])
    pub(crate) fn fn_rightb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.is_empty() { return CalcResult::new_args_number_error(cell); }
        let text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let num_bytes = if args.len() > 1 {
            match self.get_number(&args[1], cell) { Ok(n) => n as usize, Err(e) => return e }
        } else { 1 };
        let chars: Vec<char> = text.chars().collect();
        let start = chars.len().saturating_sub(num_bytes);
        CalcResult::String(chars[start..].iter().collect())
    }

    /// FINDB(find_text, within_text, [start_num])
    pub(crate) fn fn_findb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 { return CalcResult::new_args_number_error(cell); }
        let find_text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let within_text = match self.get_string(&args[1], cell) { Ok(s) => s, Err(e) => return e };
        let start_num = if args.len() > 2 {
            match self.get_number(&args[2], cell) { Ok(n) => (n as usize).saturating_sub(1), Err(e) => return e }
        } else { 0 };
        let search_text = &within_text[start_num.min(within_text.len())..];
        match search_text.find(&find_text) {
            Some(pos) => CalcResult::Number((pos + start_num + 1) as f64),
            None => CalcResult::new_error(Error::VALUE, cell, "FINDB: Not found".to_string()),
        }
    }

    /// SEARCHB(find_text, within_text, [start_num])
    pub(crate) fn fn_searchb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 2 { return CalcResult::new_args_number_error(cell); }
        let find_text = match self.get_string(&args[0], cell) { Ok(s) => s.to_lowercase(), Err(e) => return e };
        let within_text = match self.get_string(&args[1], cell) { Ok(s) => s.to_lowercase(), Err(e) => return e };
        let start_num = if args.len() > 2 {
            match self.get_number(&args[2], cell) { Ok(n) => (n as usize).saturating_sub(1), Err(e) => return e }
        } else { 0 };
        let search_text = &within_text[start_num.min(within_text.len())..];
        match search_text.find(&find_text) {
            Some(pos) => CalcResult::Number((pos + start_num + 1) as f64),
            None => CalcResult::new_error(Error::VALUE, cell, "SEARCHB: Not found".to_string()),
        }
    }

    /// REPLACEB(old_text, start_num, num_bytes, new_text)
    pub(crate) fn fn_replaceb(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        let old_text = match self.get_string(&args[0], cell) { Ok(s) => s, Err(e) => return e };
        let start_num = match self.get_number(&args[1], cell) { Ok(n) => (n as usize).saturating_sub(1), Err(e) => return e };
        let num_bytes = match self.get_number(&args[2], cell) { Ok(n) => n as usize, Err(e) => return e };
        let new_text = match self.get_string(&args[3], cell) { Ok(s) => s, Err(e) => return e };
        let chars: Vec<char> = old_text.chars().collect();
        let before: String = chars[..start_num.min(chars.len())].iter().collect();
        let after: String = chars[(start_num + num_bytes).min(chars.len())..].iter().collect();
        CalcResult::String(format!("{}{}{}", before, new_text, after))
    }
}
