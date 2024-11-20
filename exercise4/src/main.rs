use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
struct Fraction<T=i64> {
    up: T,
    down: T
}

impl Fraction {
    fn new(a:i64, b:i64) -> Self {
        Fraction {
            up: a,
            down: b
        }
    }

    fn simplify(&mut self) {
        let x = gcd(self.up, self.down);
        self.up /= x;
        self.down /= x;
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = Self {
            up: self.up*rhs.down + rhs.up*self.down,
            down: self.down*rhs.down
        };
        r.simplify();
        r
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = Self {
            up: self.up*rhs.up,
            down: self.down*rhs.down
        };
        r.simplify();
        r
    }
}


fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a%b)
}

fn main() {
    println!("Hello, world!");

    let fr = Fraction::new(1, 2);
    let frb = Fraction::new(2,3);
    println!("{:?}", fr*frb)
   
}
