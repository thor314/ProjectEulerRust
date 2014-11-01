#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;

use common::Solver;

fn compute() -> uint {
    let mut facts: [uint, ..10] = [ 0, ..10 ];
    facts[0] = 1;
    for i in range(1, facts.len()) {
        facts[i] = facts[i - 1] * i;
    }

    let mut answer = 0;
    for n in range(0, facts[9].to_string().len() * facts[9]) {
        let mut itr = n;
        let mut sum = 0;
        while itr > 0 {
            sum += facts[itr % 10];
            itr /= 10;
        }
        if sum == n {
            answer += sum;
        }
    }
    answer - 1 - 2
}

fn solve() -> String {
    compute().to_string()
}

fn main() { Solver::new("40730", solve).run(); }
