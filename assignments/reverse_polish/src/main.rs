use std::io;

fn ackermann(a: i32, b: i32) -> i32 {
    if b == 0 { return 0; }
    if a == 0 { return 2*b; }
    if b == 1 { return 2; }

    return ackermann(a-1, ackermann(a, b-1))
}

fn factorial(a: i32) -> i32 {
    if a < 2 { return 1 }
    return a * factorial(a-1)
}

fn pair_op<F>(stack: &mut Vec<i32>, f: F) -> ()
    where 
        F: FnOnce(i32,i32) -> i32
{
    if stack.len() < 2 {
        eprintln!("TOMÁŠ MY NÁM ŘEKL, ŽE TOHLE JE PHYLOSOFICKY DOBŘE přece ne?");
    }
    else {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();

        let res = f(a,b);
        stack.push(res);
    }

}

fn main() {
    let stdin = io::stdin();
    let mut stack: Vec<i32> = vec![];

    for line in stdin.lines() {
        let line = line.expect("Could not read line").trim().to_string();

        match line.as_str() {
            "+"               => pair_op(&mut stack, |b, a| a + b),                                     
            "-"               => pair_op(&mut stack, |b, a| a - b),                                     
            "*"               => pair_op(&mut stack, |b, a| a * b),                                     
            "/"               => pair_op(&mut stack, |b, a| a / b),                                     
            "A"               => pair_op(&mut stack, |a, b| ackermann(b,a)),
            "pascal-triangle" => pair_op(&mut stack, |k, n| factorial(n)/(factorial(k)*factorial(n-k))),
            "p" => println!("[{}]", stack.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),
            "c" => stack.clear(),
            "q" => break,
            x => {
                match x.parse::<i32>() {
                    Ok(num) => stack.push(num),
                    Err(_) => eprintln!("Invalid number or an unknown op"),
                }
            }
        }
    }
}
