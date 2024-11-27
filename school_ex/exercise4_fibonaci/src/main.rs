struct Fib {
    first: u64,
    second: u64,
}

impl Fib {
    fn new() -> Self {
        Fib {
            first: 0,
            second: 1,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.first;
        let (next, _) = self.first.overflowing_add(self.second);
        self.first = self.second;
        self.second = next;
        Some(res)
    }
}

fn main() {
    let my_precious = Fib::new();


    println!(
        "{:?}",
        my_precious
            .take_while(|i| *i <= 1_000_000)
            .map(|item| item * 2)
            .sum::<u64>()
    );
}
