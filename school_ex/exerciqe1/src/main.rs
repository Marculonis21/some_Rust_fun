#[derive(Default)]
struct Haha {
    a: i32,
    b: i32,
    c: i32,
}

impl Haha {
    fn cA(mut self) -> Self {
        self.a = 1;
        return self;
    }
    fn cB(mut self) -> Self {
        self.b = 1;
        return self;
    }
    fn cC(mut self) -> Self {
        self.c = 1;
        return self;
    }
}


enum QuadResult {
    NoRoot,
    One(f64),
    Two(f64, f64),
}

fn calc_quadr(a: f64, b: f64, c: f64 ) -> QuadResult {
    let D = b.powf(2.0) - 4.0 * a * c;

    if D > 0.0 {
        let first = (-b + D.sqrt()) / (2.0*a);
        let second = (-b - D.sqrt()) / (2.0*a);
        return QuadResult::Two(first, second);
    }
    else if D == 0.0 {
        let first = -b / (2.0*a);
        return QuadResult::One(first);
    }

    return QuadResult::NoRoot;
}

fn main() {
    let t = Haha::default()
        .cB()
        .cC();

    println!("{},{},{},", t.a, t.b, t.c);

    // // readline
    // let mut s = String::new();
    // match stdin().read_line(&mut s) {
    //     Ok(a) => println!("Hello {}!", s.trim()),
    //     Err(e) => println!("AHHH {}", e)
    // }

    // // read arguments
    // let args: Vec<String> = env::args().collect();
    // println!("Args:");
    // for arg in args {
    //     println!("- {}", arg);
    // }

    // ------------------------------------------------------------
    // let mut num = String::new();
    // let mut eq_args: Vec<f64> = vec![0.0,0.0,0.0];
    // for i in 0 .. eq_args.len() {
    //     num.clear();
    //     println!("Input arg n.{}", i+1);
    //     println!("{}!", num);
    //     let _ = stdin().read_line(&mut num);
    //     eq_args[i] = num.trim().parse().unwrap();
    // } 
    // println!("Equation {}x^2+{}x+{}=0", eq_args[0], eq_args[1], eq_args[2]);

    // match calc_quadr(eq_args[0], eq_args[1], eq_args[2]) {
    //     QuadResult::NoRoot => println!("No root"),
    //     QuadResult::One(a) => println!("One root {}", a),
    //     QuadResult::Two(a, b) => println!("Two root {}, {}", a, b),
    // }
    
    // ------------------------------------------------------------
    // splitting string

    // let mut num = String::new();
    // let _ = stdin().read_line(&mut num);

    // // bad for error handling
    // // let nums : Vec<u64> = num.split_whitespace().map(|a| a.parse().unwrap())
    //                                               // .collect();
                                                  
    // num.split_whitespace().map(|a| a.parse::<u32>().unwrap()).sorted().for_each(|b| println!("{b}"));
}
