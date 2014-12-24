#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate seq;

use std::cmp;
use seq::PrimitivePythagoreans;

fn get_count(m: uint) -> uint {
    let mut cnt = 0u;
    for max_a in range(0, m) {
        for (p, q, _) in PrimitivePythagoreans::new(max_a) {
            for k in range(1, m / q + 1) {
                cnt += k * p / 2;
            }

            for k in range(1, m / p + 1) {
                let end = cmp::min(k * p, k * q / 2) + 1;
                let start = k * q - k * p;
                if end > start { cnt += end - start; }
            }
        }
    }
    cnt
}

// cuboid: (a, b, c),  a <= b <= c <= M
// => S = sqrt(c^2 + (a + b)^2)
fn get_min_m(limit: uint) -> uint {
    let mut lim = 1;
    let mut cnt = get_count(lim);
    while cnt < limit {
        lim *= 2;
        cnt = get_count(lim);
    }

    let mut m = 0;
    while lim != 0 {
        let ix = m + (lim / 2);
        let cnt = get_count(ix);
        match cnt.cmp(&limit) {
            Equal => break,
            Less  => {
                m = ix + 1;
                lim -= 1;
            }
            Greater => {}
        }
        lim /= 2;
    }

    m
}

fn solve() -> String {
    get_min_m(1000000).to_string()
}

problem!("1818", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn exceed_two_thousand() {
        assert_eq!(100, super::get_min_m(2000));
    }

    #[test]
    fn get_count() {
        assert_eq!(1975, super::get_count(99));
        assert_eq!(2060, super::get_count(100));
    }
}