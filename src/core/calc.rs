use crate::store::CustomerInfo;

/// Cumulative balance is the amount after the actual debit is done
pub fn calculate_cum_balance(cus: &CustomerInfo, req_amt: &String) -> String {
    let amt_f32: f32 = req_amt.parse::<f32>().unwrap_or(0.00);     // this is very unlikely to fallback to the default value cuz the check limit does the same conversion once
    let cum_bal = cus.consumed + amt_f32;
    format!("{:.02}", cum_bal)
}