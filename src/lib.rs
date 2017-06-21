pub fn public_function() {
    println!("no!")
}

pub struct Group {
    g: u32,
    p: u64,
}

pub struct Cycle {
    start: u64,
    end: u64,
    g: u64,
    p: u64,
    curr: Option<u64>,
    next: Option<u64>,
}

fn order(e: u32, p: u64) -> u64 {
    return p - 1;
}

fn is_generator(e: u32, p: u64) -> bool {
    return order(e, p) == p - 1;
}

fn is_prime(candidate: u64) -> bool {
    return true;
}

impl Group {
    fn validate(&self) -> bool {
        // Check that p is prime
        if !is_prime(self.p) {
            return false;
        }

        // Check that g is a generator
        if !is_generator(self.g, self.p) {
            return false;
        }

        // TODO: Check for overflow
        return true;
    }
}

impl Cycle {
    fn new(group: Group, start: u32, end: u64) -> Result<Cycle, &'static str> {
        if !group.validate() {
            return Err("invalid group");
        }
        match group.p.checked_mul(group.g as u64) {
            Some(_) => (),
            None => return Err("generator may cause overflow"),
        }
        let next: u64 = (start as u64 * group.g as u64) % group.p;
        let next_option = match next {
            _ if next != end => Some(next),
            _ => None
        };
        return Ok(Cycle {
            g: group.g as u64,
            p: group.p,
            start: start as u64,
            end: end,
            curr: Some(start as u64),
            next: next_option,
        });
    }
}

impl Iterator for Cycle {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = match self.curr {
            Some(c) => Some((c * self.g) % self.p),
            None => None
        };
        let result = self.next;
        self.next = match next {
            Some(c) if c != self.end => next,
            _ => None
        };
        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::Group;
    use super::order;
    use super::is_prime;

    struct OrderTest {
        group: Group,
        order: u64,
    }

    #[test]
    fn test_order() {
        let tests: [OrderTest; 2] = [
            OrderTest {
                group: Group {
                    g: 2,
                    p: 7,
                },
                order: 3,
            },
            OrderTest {
                group: Group {
                    g: 5,
                    p: 7,
                },
                order: 6,
            }
        ];
        for t in tests.iter() {
            let computed_order = order(t.group.g, t.group.p);
            assert_eq!(t.order, computed_order);
        }
    }

    #[test]
    fn test_is_prime() {
        let primes: [u64; 14] = [
            2, 3, 5, 7, 11, 13, 17, 19, 97, 257, 65537, 16777259, 268435459, 4294967311
        ];
        for p in primes.iter() {
            assert_eq!(true, is_prime(*p));
        }
        let mut composites: Vec<u64> = Vec::with_capacity(2*primes.len());
        for a in primes.iter() {
            for b in primes.iter() {
                let result = a.checked_mul(*b);
                match result {
                    Some(c) => composites.push(c),
                    None => {}
                }
            }
        }
        for c in composites.iter() {
            assert_eq!(false, is_prime(*c));
        }
    }

}
