use std::fs;
use chumsky::Parser;

mod isa;
use isa::parser::*;


fn main() {
   
    let test_program = fs::read_to_string("src/isa/tests/simple.s").expect("Couldn't open file simple.s");

    let parsed = program().parse(&test_program).into_result();

    match parsed {
        Ok(ast) => println!("{:#?}", ast),
        Err(errs) => {
            for e in errs { eprintln!("{e:?}"); }
        }
    }

}
