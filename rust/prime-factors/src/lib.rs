struct Prime {
    curr: u64,
    primes: Vec<u64>,
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == 2 {
            self.primes.push(2_u64);
            return Some(3_u64);
        }

        loop {
            if self.primes.iter().all(|y| self.curr % y != 0) {
                self.primes.push(self.curr);
                break;
            }
            self.curr += 2;
        }
        Some(self.curr)
    }
}

fn primes() -> Prime {
    Prime {
        curr: 2,
        primes: Vec::new(),
    }
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];

    for p in primes() {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
        if n == 1 {
            break;
        }
    }

    factors
}
