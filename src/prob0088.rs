#[link(name = "prob0088", vers = "0.0")];
#[crate_type = "lib"];



use std::{uint, vec};
use std::iterator::AdditiveIterator;
use std::hashmap::HashSet;

pub static EXPECTED_ANSWER: &'static str = "7587457";

fn each_sum_product(start: uint, end: uint, f: &fn(uint, uint, uint) -> bool) -> bool {
    return sub(start, end, 0, 1, 0, f);

    fn sub(start: uint, end: uint, sum: uint, prod: uint, len: uint,
           f: &fn(uint, uint, uint) -> bool) -> bool {
        for n in range(start, end / prod + 1) {
            if len > 0 {
                if !f(sum + n, prod * n, len + 1) { return false; }
            }

            if !sub(n, end, sum + n, prod * n, len + 1, |a, b, c| f(a, b, c)) { return false; }
        }
        return true;
    }
}

pub fn solve() -> ~str {
    let limit = 12000;

    let start = 2;
    let mut end = 4;
    let mut cnt = limit - 1;
    let mut nums = vec::from_elem(limit + 1, uint::max_value);

    while cnt > 0 {
        do each_sum_product(start, end) |sum, prod, len| {
            let k = prod - sum + len;
            if k <= limit && prod < nums[k] {
                if nums[k] == uint::max_value { cnt -= 1; }
                nums[k] = prod;
            }
            true
        };
        end *= 2;
    }

    let mut set = HashSet::new();
    for &n in nums.iter() {
        if n != uint::max_value { set.insert(n); }
    }

    return set.move_iter().sum().to_str();
}
