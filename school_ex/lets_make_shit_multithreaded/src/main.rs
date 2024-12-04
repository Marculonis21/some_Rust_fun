use std::io;
use std::thread;

enum Ops {
    Add,
    Sub,
    Div,
    Mul,
    Print,
    Push(i32),
    Res(String),
}

fn pair_op<F>(stack: &mut Vec<i32>, f: F) -> Result<(), String>
    where 
        F: FnOnce(i32,i32) -> i32
{
    if stack.len() < 2 {
        return Err("I'm not enough!!!!!!".into());
    }

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    let res = f(a,b);
    stack.push(res);

    Ok(())
}

fn main() {
    let stdin = io::stdin();
    let mut stack: Vec<i32> = vec![];

    let (compute_sender, compute_receiver) = flume::unbounded();
    let (output_sender, output_receiver) = flume::unbounded();

    let compute_thread = thread::spawn(move || {
        while let Ok(message) = compute_receiver.recv() {
            // if let Err(e) = process_message(&mut stack, output_sender, message) {
            //     eprintln!("{e}");
            // }
            let res = match message {
                Ops::Add => pair_op(&mut stack, |a, b| a + b),
                Ops::Sub => pair_op(&mut stack, |a, b| a - b),
                Ops::Div => pair_op(&mut stack, |a, b| a / b),
                Ops::Mul => pair_op(&mut stack, |a, b| a * b),
                Ops::Push(num) => {
                    stack.push(num);
                    Ok(())
                }
                Ops::Print => {
                    output_sender.send(Ops::Res(format!("{stack:?}")));
                    Ok(())
                }
                Ops::Res(_) => {
                    Err("invalid message".into())
                }
            };

            if let Err(e) = res {
                eprintln!("{e}");
            }
        }
    });

    let output_thread = thread::spawn(move || {
        while let Ok(message) = output_receiver.recv() {
            if let Ops::Res(msg) = message {
                println!("{msg}")
            }
            else {
                eprintln!("INVALID SOMETHING BS")
            }
        }
    });


    for line in stdin.lines() {
        let line = line.expect("Could not read line").trim().to_string();

        // println!("{line}");
        let res = match line.as_str() {
            "p" => compute_sender.send(Ops::Print),
            "+" => compute_sender.send(Ops::Add),
            "-" => compute_sender.send(Ops::Sub),
            "/" => compute_sender.send(Ops::Div),
            "*" => compute_sender.send(Ops::Mul),
            x => {
                let Ok(num) = x.parse::<i32>() else {
                    eprintln!("invalid number or an unknown op");
                    continue;
                };
                compute_sender.send(Ops::Push(num))
            }
        };
        if let Err(e) = res {
            eprintln!("{e}");
        }
    }

    compute_thread.join();
    output_thread.join();
}
