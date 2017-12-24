#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_mr_works() {
        assert_eq!(is_prime_mr(0), false);
        assert_eq!(is_prime_mr(1), false);
        assert_eq!(is_prime_mr(2), true);
        assert_eq!(is_prime_mr(3), true);
        assert_eq!(is_prime_mr(4), false);
        assert_eq!(is_prime_mr(5), true);
        assert_eq!(is_prime_mr(6), false);
        assert_eq!(is_prime_mr(7), true);
        assert_eq!(is_prime_mr(8), false);
        assert_eq!(is_prime_mr(9), false);
        assert_eq!(is_prime_mr(10), false);
        assert_eq!(is_prime_mr(11), true);
    }

    #[test]
    fn next_prime_works() {
        assert_eq!(next_prime(11), 11);
        assert_eq!(next_prime(12), 13);
        assert_eq!(next_prime(2_047), 2_053);
        assert_eq!(next_prime(1_373_653), 1_373_677);
    }

    #[test]
    fn previous_prime_works() {
        assert_eq!(previous_prime(0), 0);
        assert_eq!(previous_prime(1), 0);
        assert_eq!(previous_prime(2), 2);
        assert_eq!(previous_prime(3), 3);
        assert_eq!(previous_prime(4), 3);
        assert_eq!(previous_prime(5), 5);
        assert_eq!(previous_prime(6), 5);
        assert_eq!(previous_prime(7), 7);
        assert_eq!(previous_prime(17), 17);
        assert_eq!(previous_prime(18), 17);
        assert_eq!(previous_prime(26), 23);
    }

    #[test]
    fn prime_factors_works() {
        assert_eq!(prime_factors(0), vec![]);
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(2), vec![(2, 1)]);
        assert_eq!(prime_factors(2 * 2 * 7 * 13), vec![(2, 2), (7, 1), (13, 1)]);
        assert_eq!(
            prime_factors(37 * 37 * 37 * 7 * 13),
            vec![(7, 1), (13, 1), (37, 3)]
        );
    }

    #[test]
    fn iterator_works() {
        assert_eq!(primes(0).take(4).collect::<Vec<_>>(), vec![2, 3, 5, 7]);
        assert_eq!(primes(0).find(|&x| x > 1000).unwrap(), 1009);
        assert_eq!(primes(0).nth(10_000 - 1).unwrap(), 104_729);
    }
}

fn power(a: u64, n: u64, m: u64) -> u64 {
    let mut power = a;
    let mut result = 1;
    let mut n = n;
    while n > 0 {
        if n & 1 > 0 {
            result = (result * power) % m;
        }
        power = (power * power) % m;
        n >>= 1;
    }
    result
}

/// Miller–Rabin primality test
/// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Deterministic_variants
pub fn is_prime_mr(n: u64) -> bool {
    if n <= 3 {
        return n >= 2;
    }
    if n % 2 == 0 {
        return false;
    }

    let (d, s) = {
        let mut d = n / 2;
        let mut s = 1;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        (d, s)
    };

    if n < 2_047 {
        [2].iter()
    } else if n < 1_373_653 {
        [2, 3].iter()
    } else if n < 9_080_191 {
        [31, 73].iter()
    } else if n < 25_326_001 {
        [2, 3, 5].iter()
    } else if n < 3_215_031_751 {
        [2, 3, 5, 7].iter()
    } else if n < 4_759_123_141 {
        [2, 7, 61].iter()
    } else if n < 1_122_004_669_633 {
        [2, 13, 23, 1_662_803].iter()
    } else if n < 2_152_302_898_747 {
        [2, 3, 5, 7, 11].iter()
    } else if n < 3_474_749_660_383 {
        [2, 3, 5, 7, 11, 13].iter()
    } else if n < 341_550_071_728_321 {
        [2, 3, 5, 7, 11, 13, 17].iter()
    } else if n < 3_825_123_056_546_413_051 {
        [2, 3, 5, 7, 11, 13, 17, 19, 23].iter()
    } else {
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter()
    }.cloned()
        .all(|a: u64| -> bool {
            let mut x = power(a, d, n);
            let mut y: u64 = 0;
            let mut s = s;

            while s > 0 {
                y = (x * x) % n;
                if y == 1 && x != 1 && x != n - 1 {
                    return false;
                }
                x = y;
                s = s - 1;
            }
            y == 1
        })
}

pub fn next_prime(x: u64) -> u64 {
    match x {
        0 | 1 | 2 => {
            return 2;
        }
        3 => {
            return 3;
        }
        4 | 5 => return 5,
        _ => {}
    }
    let k = x / 6;

    let o = if x % 6 < 2 { 1 } else { 5 };

    let mut x = 6 * k + o;
    let mut i = (3 + o) / 2;
    while !is_prime_mr(x) {
        i ^= 6;
        x += i;
    }
    x
}

pub fn previous_prime(mut x: u64) -> u64 {
    match x {
        0 | 1 => return 0,
        2 => return 2,
        3 | 4 => return 3,
        _ => {}
    }
    if x % 2 == 0 {
        x -= 1;
    }

    // x is odd
    let (o, mut i) = if x % 6 == 5 { (5, 4) } else { (1, 2) };

    x = (x / 6) * 6 + o;
    while !is_prime_mr(x) {
        x -= i;
        i ^= 6;
    }
    x
}

pub fn prime_factors(mut x: u64) -> Vec<(u64, u32)> {
    let mut xs = Vec::new();

    let mut y = 2;
    while x > 1 {
        if is_prime_mr(x) {
            xs.push((x, 1));
            break;
        }

        let mut n = 0;
        while x % y == 0 {
            x /= y;
            n += 1;
        }
        if n > 0 {
            xs.push((y, n));
        }
        y = next_prime(y + 1);
    }
    xs
}

pub struct PrimeIter {
    last: u64,
}

impl Iterator for PrimeIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.last = next_prime(self.last + 1);
        Some(self.last)
    }
}

pub fn primes(from: u64) -> PrimeIter {
    PrimeIter { last: from }
}
