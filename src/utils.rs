use std::fmt::Debug;
use serde::Serialize;

use crate::store::CustomerInfo;

// find & get mutable CustomerInfo
pub fn find_n_get_mut_customer_info<'a>(
    customer_list: &'a mut Vec<CustomerInfo>,
    unique_id: &str,
) -> Option<&'a mut CustomerInfo> {
    for customer in customer_list {
        if customer.unique_id == unique_id {
            return Some(customer);
        }
    }
    None
}

pub fn find_by_mobile_number<'a>(mob: &str, customer_infos: &'a Vec<CustomerInfo>) -> Option<&'a CustomerInfo> {
    for c in customer_infos {
        if c.mobile_number == mob {
            return Some(c);
        }
    }
    None
}

// get customer info
pub fn is_customer_exits_by_mobile_number(mob: &str, customer_infos: &Vec<CustomerInfo>) -> bool {
    for customer in customer_infos {
        if mob == customer.mobile_number {
            return true;
        }
    }
    false
}

/// print the pretty json format if possible, else print with Debug
pub fn print_req_res<U>(item: &U, ty: &str) where U: Serialize + Debug {
    let json_ed = serde_json::to_string_pretty(item);

    match json_ed {
        Ok(s) => println!("{}: {}", ty, s),
        _ => println!("{}: {:?}", ty, item)
    }
}

pub fn find_by_unique_id<'a>(id: &String, all_customers: &'a Vec<CustomerInfo>) -> Option<&'a CustomerInfo> {
    for customer in all_customers {
        if customer.unique_id == *id {
            return Some(&customer);
        }
    }
    None
}

pub fn some_or_na<T: std::fmt::Display>(some_value: &Option<T>) -> String {
    match some_value {
        Some(value) => value.to_string(),
        None => "NA".to_string(),
    }
} 
