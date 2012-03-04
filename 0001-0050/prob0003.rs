use std;

fn gen_prime(primes: [u64]) -> [u64] {
    let num = alt vec::last(primes) {
      none       { ret [2u64] }
      some(2u64) { ret primes + [3u64] }
      some(x)    { x + 2u64 }
    };
    while true {
        for p in primes {
            if p * p > num {
                ret primes + [num];
            }
            if num % p == 0u64 {
                break;
            }
        }
        num += 2u64;
    }
    fail;
}

fn main() {
    let primes = gen_prime([]);
    let num = 600851475143u64;
    while num != 1u64 {
        check vec::is_not_empty(primes);
        let p = vec::last_total(primes);
        while num % p == 0u64 {
            num /= p;
            std::io::println(#fmt("%u", p));
        }
        primes = gen_prime(primes);
    }
}