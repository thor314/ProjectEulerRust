pub fn zip_default<T: Copy, U: Copy>(v1: &[const T], v2: &[const U], def: (T, U)) -> ~[(T, U)] {
    let mut result = ~[];
    let (l1, l2) = (vec::len(v1), vec::len(v2));
    let (d1, d2) = def;
    for uint::range(0u, uint::max(l1, l2)) |i| {
        let e1 = if i < l1 { v1[i] } else { d1 };
        let e2 = if i < l2 { v2[i] } else { d2 };
        result += ~[(e1, e2)];
    }
    return result;
}

pub fn each_window<T>(v: &[T], n: uint, f: &fn(&[T])->bool) {
    let len = v.len();
    if len < n { return; }
    for uint::range(0, v.len() - n) |i| {
        if !f(v.slice(i, i+n)) {
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_default() {
        assert_eq!(zip_default([1, 2, 3], [4u, 5u, 6u], (0, 0u)),
                   ~[(1, 4u), (2, 5u), (3, 6u)]);
        assert_eq!(zip_default([1, 2, 3], [4u], (0, 0u)),
                   ~[(1, 4u), (2, 0u), (3, 0u)]);
        assert_eq!(zip_default([], [], (0, 0u)),
                   ~[]);
    }
}
