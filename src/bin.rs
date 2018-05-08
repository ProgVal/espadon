extern crate espadon;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use espadon::{StrSpan, IResult, parse};

fn main() {
    let mut args = env::args_os();
    let prog = args.next().expect("argv[0] is missing.");
    let syntax = format!("Syntax: {:?} <source path>", prog);
    let src_path = args.next().expect(&syntax);

    let mut file = File::open(&src_path).expect(&format!("Invalid path: {:?}", &src_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    let input = StrSpan::new(&contents);
    match parse(input) {
        IResult::Done(remaining, ref parsed) if remaining.fragment == "" => {
            println!("{:?}", parsed);
            exit(0);
        },
        IResult::Done(remaining, _parsed) => {
            println!("Remaining:\n{}", remaining.fragment);
            exit(1);
        },
        x => {
            println!("{:?}", x);
            exit(2);
        },
    }
}
