use core::{f64};
use std::{fmt, ops::Add};

#[macro_export]
macro_rules! convert {
    ($value:expr => $type:ty) => {{
        let s: &str = &$value.to_string();
        s.parse::<$type>()
    }};
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "Denominator cannot be zero");

        // simplification alg
        let gcd = Self::gcd(numerator, denominator);

        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    pub fn from_decimal(dec: f64) -> Self {
        let precision: f64 = 1000000000.0;
        let num = (dec * precision).round();
        Fraction::new(num as i64, precision as i64)
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }

        a.abs()
    }
}

// Display formatting to match original input format
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else if self.numerator == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator +
                        rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;

        Fraction::new(numerator, denominator)
    }
}

#[macro_export]
macro_rules! frac {
    ($num:literal / $den:literal) => {{
        $crate::solution::Fraction::new($num,$den)
    }};

    ($num:literal) => {{
        $crate::solution::Fraction::new($num,1)
    }};

    (dec $num:literal) => {{
        $crate::solution::Fraction::from_decimal($num)
    }};
}
