// Phase 5: Additional Financial Functions (10 new functions)
// ACCRINT, ACCRINTM, PRICE, YIELD, DURATION, MDURATION, DISC, INTRATE, RECEIVED, PRICEMAT
// Note: EFFECT, NOMINAL, TBILLEQ, TBILLPRICE, TBILLYIELD already exist in financial.rs

use crate::calc_result::CalcResult;
use crate::expressions::parser::Node;
use crate::expressions::token::Error;
use crate::expressions::types::CellReferenceIndex;
use crate::model::Model;

impl Model {
    /// ACCRINT(issue, first_interest, settlement, rate, par, frequency, [basis])
    pub(crate) fn fn_accrint(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 6 { return CalcResult::new_args_number_error(cell); }
        
        let rate = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let par = match self.get_number(&args[4], cell) { Ok(n) => n, Err(e) => return e };
        let frequency = match self.get_number(&args[5], cell) { Ok(n) => n as i32, Err(e) => return e };
        
        if rate <= 0.0 || par <= 0.0 || ![1, 2, 4].contains(&frequency) {
            return CalcResult::new_error(Error::NUM, cell, "ACCRINT: Invalid arguments".to_string());
        }
        
        // Simplified: assumes 1 full period accrued
        let accrued = par * rate / frequency as f64;
        CalcResult::Number(accrued)
    }

    /// ACCRINTM(issue, settlement, rate, par, [basis])
    pub(crate) fn fn_accrintm(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        
        let rate = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let par = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        
        if rate <= 0.0 || par <= 0.0 {
            return CalcResult::new_error(Error::NUM, cell, "ACCRINTM: Invalid arguments".to_string());
        }
        
        // Simplified: assumes 1 year maturity
        CalcResult::Number(par * rate)
    }

    /// PRICE(settlement, maturity, rate, yield, redemption, frequency, [basis])
    pub(crate) fn fn_price(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 6 { return CalcResult::new_args_number_error(cell); }
        
        let rate = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let yld = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let redemption = match self.get_number(&args[4], cell) { Ok(n) => n, Err(e) => return e };
        let frequency = match self.get_number(&args[5], cell) { Ok(n) => n as i32, Err(e) => return e };
        
        if rate < 0.0 || yld < 0.0 || redemption <= 0.0 || ![1, 2, 4].contains(&frequency) {
            return CalcResult::new_error(Error::NUM, cell, "PRICE: Invalid arguments".to_string());
        }
        
        // Simplified bond pricing (1 period)
        let coupon = rate * redemption / frequency as f64;
        let price = (redemption + coupon) / (1.0 + yld / frequency as f64);
        CalcResult::Number(price)
    }

    /// YIELD(settlement, maturity, rate, pr, redemption, frequency, [basis])
    pub(crate) fn fn_yield(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 6 { return CalcResult::new_args_number_error(cell); }
        
        let rate = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let pr = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let redemption = match self.get_number(&args[4], cell) { Ok(n) => n, Err(e) => return e };
        let frequency = match self.get_number(&args[5], cell) { Ok(n) => n as i32, Err(e) => return e };
        
        if rate < 0.0 || pr <= 0.0 || redemption <= 0.0 || ![1, 2, 4].contains(&frequency) {
            return CalcResult::new_error(Error::NUM, cell, "YIELD: Invalid arguments".to_string());
        }
        
        // Simplified yield calculation
        let coupon = rate * redemption;
        let yld = (coupon + redemption - pr) / pr;
        CalcResult::Number(yld)
    }

    /// DURATION(settlement, maturity, coupon, yield, frequency, [basis])
    pub(crate) fn fn_duration(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 5 { return CalcResult::new_args_number_error(cell); }
        
        let coupon = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let yld = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let frequency = match self.get_number(&args[4], cell) { Ok(n) => n as i32, Err(e) => return e };
        
        if coupon < 0.0 || yld < 0.0 || ![1, 2, 4].contains(&frequency) {
            return CalcResult::new_error(Error::NUM, cell, "DURATION: Invalid arguments".to_string());
        }
        
        // Simplified Macaulay duration approximation
        let duration = (1.0 + yld) / (yld * frequency as f64);
        CalcResult::Number(duration.min(30.0)) // Cap at 30 years
    }

    /// MDURATION(settlement, maturity, coupon, yield, frequency, [basis])
    pub(crate) fn fn_mduration(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 5 { return CalcResult::new_args_number_error(cell); }
        
        let yld = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let frequency = match self.get_number(&args[4], cell) { Ok(n) => n as i32, Err(e) => return e };
        
        // Modified duration = Macaulay duration / (1 + yield/frequency)
        let mac_duration = match self.fn_duration(args, cell) {
            CalcResult::Number(n) => n,
            e => return e,
        };
        
        CalcResult::Number(mac_duration / (1.0 + yld / frequency as f64))
    }

    /// DISC(settlement, maturity, pr, redemption, [basis])
    pub(crate) fn fn_disc(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        
        let pr = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let redemption = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        
        if pr <= 0.0 || redemption <= 0.0 || pr >= redemption {
            return CalcResult::new_error(Error::NUM, cell, "DISC: Invalid arguments".to_string());
        }
        
        // Simplified: assumes 1 year to maturity
        let disc = (redemption - pr) / redemption;
        CalcResult::Number(disc)
    }

    /// INTRATE(settlement, maturity, investment, redemption, [basis])
    pub(crate) fn fn_intrate(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        
        let investment = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let redemption = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        
        if investment <= 0.0 || redemption <= 0.0 {
            return CalcResult::new_error(Error::NUM, cell, "INTRATE: Invalid arguments".to_string());
        }
        
        // Simplified: assumes 1 year
        let intrate = (redemption - investment) / investment;
        CalcResult::Number(intrate)
    }

    /// RECEIVED(settlement, maturity, investment, discount, [basis])
    pub(crate) fn fn_received(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 4 { return CalcResult::new_args_number_error(cell); }
        
        let investment = match self.get_number(&args[2], cell) { Ok(n) => n, Err(e) => return e };
        let discount = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        
        if investment <= 0.0 || discount <= 0.0 || discount >= 1.0 {
            return CalcResult::new_error(Error::NUM, cell, "RECEIVED: Invalid arguments".to_string());
        }
        
        // Simplified: assumes 1 year
        let received = investment / (1.0 - discount);
        CalcResult::Number(received)
    }

    /// PRICEMAT(settlement, maturity, issue, rate, yield, [basis])
    pub(crate) fn fn_pricemat(&mut self, args: &[Node], cell: CellReferenceIndex) -> CalcResult {
        if args.len() < 5 { return CalcResult::new_args_number_error(cell); }
        
        let rate = match self.get_number(&args[3], cell) { Ok(n) => n, Err(e) => return e };
        let yld = match self.get_number(&args[4], cell) { Ok(n) => n, Err(e) => return e };
        
        if rate < 0.0 || yld < 0.0 {
            return CalcResult::new_error(Error::NUM, cell, "PRICEMAT: Invalid arguments".to_string());
        }
        
        // Simplified price at maturity (1 year assumed)
        let price = 100.0 * (1.0 + rate) / (1.0 + yld);
        CalcResult::Number(price)
    }
}
