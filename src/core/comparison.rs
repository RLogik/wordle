// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use std::cmp::Ordering;

// ----------------------------------------------------------------
// Method generic comparison functions for i8,i16,...,f64
// ----------------------------------------------------------------

pub fn cmp_type<T>(x1: T, x2: T) -> Ordering
    where T: std::cmp::PartialOrd
{
    if x1 < x2 {
        return Ordering::Less;
    } else if x1 > x2 {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

// ----------------------------------------------------------------
// Method combine comparison functions
// ----------------------------------------------------------------

pub fn lexical_comparison(ord: &Vec<Ordering>) -> Ordering {
    for &order in ord.iter() {
        if order == Ordering::Less {
            return Ordering::Less;
        } else if order == Ordering::Greater {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}
