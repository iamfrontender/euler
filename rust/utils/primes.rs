use std::vec::Vec;

/// Simplest struct for prime representation
pub struct Prime {
    curr: u64
}

impl Iterator for Prime {
    type Item = u64;

    /// Returns next prime factor after current one
    fn next(&mut self) -> Option<u64> {
        let mut new_next = self.curr + 1;

        while !is_prime(new_next) {
            new_next += 1;
        }

        self.curr = new_next;

        Some(self.curr)
    }
}

/// Simple 'constructor' with 1 as default start
impl Prime {
    pub fn new() -> Prime {
        Prime { curr: 1 }
    }
}

/// Returns true if given `num` is prime number
pub fn is_prime(num: u64) -> bool {
    if num < 4 {
        return true;
    }

    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    (1u64..)
        .map(|x| 6 * x - 1)
        .take_while(|x| (x * x) <= num)
        .all(|x| num % x != 0 && num % (x + 2) != 0)
}

/// Generates a vector of primes up to given limit
/// 1  -> 1
/// 3  -> 1, 2, 3
/// 4  -> 1, 2, 3
/// 10 -> 1, 2, 3, 5, 7
pub fn generate_primes(upper: u64) -> Vec<u64> {
    let mut out = Vec::new();

    if upper < 3 {
         for i in 1..upper {
             out.push(i)
         }
    } else {
        let mut i = 3u64;

        out.push(1);
        out.push(2);

        while i < upper {
            if is_prime(i) {
                out.push(i);
            }

            i += 2;
        }
    }


    out
}