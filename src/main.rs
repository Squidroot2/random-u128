use std::env;

struct ParsedArgs {
    no_new_line: bool,
}

impl ParsedArgs {
    fn parse() -> Self {
        let mut out = Self::default();
        for argument in env::args() {
            if argument == "-n" {
                out.no_new_line = true;
            }
        }
        out
    }
}

impl Default for ParsedArgs {
    fn default() -> Self {
        Self { no_new_line: false }
    }
}

fn main() {
    let args = ParsedArgs::parse();
    let random_u128 = rand::random::<u128>();
    if args.no_new_line {
        print!("0x{:x}", random_u128);
    } else {
        println!("0x{:x}", rand::random::<u128>());
    }
}
