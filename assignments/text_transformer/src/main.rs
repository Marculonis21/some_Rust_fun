use core::{panic, str};
use std::env;

// V1 STRATEGY Approach
trait Transmutation {
    fn transmute(&self, data: &str) -> String;
}

struct LowerCaseTransmutation;
impl Transmutation for LowerCaseTransmutation {
    fn transmute(&self, data: &str) -> String {
        data.to_lowercase()
    }
}
struct UpperCaseTransmutation;
impl Transmutation for UpperCaseTransmutation {
    fn transmute(&self, data: &str) -> String {
        data.to_uppercase()
    }
}
struct SlugifyTransmutation;
impl Transmutation for SlugifyTransmutation{
    fn transmute(&self, data: &str) -> String {
        let output = data.to_string()
                         .to_lowercase()
                         .chars()
                         .map(|c| {
                             match c {
                                 'a'..='z' | '0'..='9' | '-' | '_' => c,
                                 ' ' => '-',
                                 _ => ' '
                             }
                         }).filter(|c| !c.is_whitespace()).collect();
        output
    }
}
struct NospaceTransmutation;
impl Transmutation for NospaceTransmutation{
    fn transmute(&self, data: &str) -> String {
        data.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

fn transmute_input<T: Transmutation>(data: &str, strategy: T) -> String {
    strategy.transmute(data)
}

// V2 Trait on strings
// trait TransmutationTypes {
//     fn transmute_lowercase(&self) -> Self;
//     fn transmute_uppercase(&self) -> Self;
//     fn transmute_slugify(&self) -> Self;
//     fn transmute_nospace(&self) -> Self;
// }

// impl TransmutationTypes for String {
//     fn transmute_lowercase(&self) -> Self { self.to_lowercase() }
//     fn transmute_uppercase(&self) -> Self { self.to_uppercase() }
//     fn transmute_slugify(&self) -> Self { slugify(self) }
//     fn transmute_nospace(&self) -> Self { self.replace(" ", "") }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 3, "We expect the caller + 2 arguments!");

    // V1
    let output = match args[1].as_str() {
        "lowercase" => transmute_input(&args[2], LowerCaseTransmutation),
        "uppercase" => transmute_input(&args[2], UpperCaseTransmutation),
        "no-spaces" => transmute_input(&args[2], NospaceTransmutation),
        "slugify"   => transmute_input(&args[2], SlugifyTransmutation),
        _ => panic!("Unknown transmutation parameter? Should I implement more UwU 👉👈 ...")
    };
    
    // V2
    // let output = match args[1].as_str() {
    //     "lowercase" => args[2].transmute_lowercase(),
    //     "uppercase" => args[2].transmute_uppercase(),
    //     "no-spaces" => args[2].transmute_nospace(),
    //     "slugify" => args[2].transmute_slugify(),
    //     _ => panic!("Unknown transmutation parameter")
    // };

    println!("{}", output)
}
