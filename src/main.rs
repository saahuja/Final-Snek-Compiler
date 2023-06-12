mod parser;
mod compiler;

//use compiler::compile;
//use sexp::parse;
use parser::parse_program_string;
use compiler::compile_program;
use std::env;
use std::fs::File;
use std::io::prelude::*;

//Diamondback completed by Sahil Ahuja
//Borrowed cobra code and structure from compiler #14 extensively, and from the CSE231 lecture #1 - stack_alloc in class code base
//Extended function call, parameter, definition error checking, parsing, and compiling with my own code


fn main() -> std::io::Result<()> {
    // Parse arguments and read input file
    let args: Vec<String> = env::args().collect();
    let in_name = &args[1];
    let out_name = &args[2];
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Compile and format assembly instructions
    //let result = compile(&parse_string(&in_contents));

    //let prog = "(".to_owned() + &in_contents + ")";
    //let prog = parse_program(&parse(&prog).unwrap());
    let prog = parse_program_string(&in_contents);
    let (defs, main) = compile_program(&prog);


    let asm_program = format!(
        "
section .text
extern snek_error
extern snek_print
extern snek_eq
global our_code_starts_here
throw_overflow_error:
  mov rdi, 2
  push rsp
  call snek_error
  ret
throw_type_error:
  mov rdi, 1
  push rsp
  call snek_error
  ret
throw_bounds_error:
  mov rdi, 3
  push rsp
  call snek_error
  ret
throw_too_many_elements_error:
  mov rdi, 4
  push rsp
  call snek_error
  ret
throw_tag_check_error:
  mov rdi, 5
  push rsp
  call snek_error
  ret
throw_divide_by_0_error:
  mov rdi, 6
  push rsp
  call snek_error
  ret
throw_unequal_size_error:
  mov rdi, 7
  push rsp
  call snek_error
  ret
{}
our_code_starts_here:
  mov r15, rsi
  {}
  ret
",
        defs, main
    );

    // Write assembly instructions to output file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

