use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64, memory : *mut i64) -> i64;
}

#[no_mangle]
#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    match errcode {
        1 => eprintln!("Runtime Error invalid type: {errcode}"),
        2 => eprintln!("Runtime Error overflow: {errcode}"),
        3 => eprintln!("Runtime Error out of bounds index: {errcode}"),
        4 => eprintln!("Runtime Error too many elements provided: {errcode}"),
        5 => eprintln!("Runtime Error tag check failed: {errcode}"),
        6 => eprintln!("Runtime Error divide by 0 error: {errcode}"),
        7 => eprintln!("Runtime Error not equal array size: {errcode}"),
        _ => eprintln!("Runtime Error unknown: {errcode}"),
        // //6 => eprintln!("Runtime Error not equal size (equal array) {errcode}"),
    }
    std::process::exit(1);
}

/* 
#[no_mangle]
#[export_name = "\x01print_func"]
fn print_func(val: i64) -> i64 {
  if val == 3 {println!("true"); }
  else if val == 1 {println!("false"); }
  else if val % 2 == 0 {println!("{}", val >> 1) }
  else {
    eprintln!("Unknown value: {}", val);
    std::process::exit(1);
  }
  return val
}
*/



// let's change snek_str to print ... when it visits a cyclic value
fn snek_str(val : i64, seen : &mut Vec<i64>) -> String {
  if val == 7 { "true".to_string() }
  else if val == 3 { "false".to_string() }
  else if val % 2 == 0 { format!("{}", val >> 1) }
  else if val == 1 { "nil".to_string() }
  else if val & 1 == 1 { //indicates pair
    if seen.contains(&val)  { return "(array <cyclic>)".to_string() }
    seen.push(val);
    let addr = (val - 1) as *const i64;
    let array_size = unsafe { *addr } >> 1;

    //println!("{}", array_size);

    let mut to_return = String::new();
    to_return.push_str(&format!("(array {} (", array_size));

    //let snd = unsafe { *addr.offset(1) }; //.offset(1) by 1 word
    for i in 1..(array_size+1) {
      to_return.push_str(&format!("{}", snek_str(unsafe {*addr.offset(i as isize)}, seen)));
      if i != array_size {
        to_return.push_str(&format!(" "));
      }
    }

    to_return.push_str(&format!("))"));
    //let result = format!("(array {} {})", snek_str(array_size, seen), snek_str(snd, seen));
    seen.pop();
    return to_return;
  }
  else {
    format!("Unknown value: {}", val)
  }
}



fn snek_eq_mut(val1: i64, val2: i64, seen: &mut Vec<(i64, i64)>) -> bool {
 
  if val1 == val2 { //if equal values return true
    return true;
  }

  if (val1 & 3 == 1) && (val2 & 3 == 1) { //if both are arrays
    if seen.contains(&(val1, val2)) { //if array pair had been compared before (then return true (don't compare cyclic again))
      return true;
    } else {
        seen.push((val1, val2)); //otherwise push this pair
    }
    
    let addr1 = (val1 - 1) as *const i64;
    let addr2 = (val2 - 1) as *const i64;

    let array_size1 = unsafe { *addr1 } >> 1;
    let array_size2 = unsafe { *addr2 } >> 1;

    if array_size1 != array_size2 { //unequal size (return false)
        return false;
    }

    for i in 1..(array_size1 + 1) { //iterating through every element
        let subval1 = unsafe { *addr1.offset(i as isize) };
        let subval2 = unsafe { *addr2.offset(i as isize) };

        if !snek_eq_mut(subval1, subval2, seen) {
            return false;
        }
    }

    seen.pop();
    return true;
  }
  return false;

}

#[no_mangle]
#[export_name = "\x01snek_print"]
fn snek_print(val : i64) -> i64 {
  let mut seen = Vec::<i64>::new();
  println!("{}", snek_str(val, &mut seen));
  return val;
}


#[no_mangle]
#[export_name = "\x01snek_eq"]
fn snek_eq(val1: i64, val2: i64) -> i64 {
  let mut seen = Vec::<(i64, i64)>::new();
  let result = snek_eq_mut(val1, val2, &mut seen);
  if result == true{
    return 7;
  } 
  //println!("Result: {result}"); 
  return 3;
}




fn parse_input(input: &str) -> i64 {


    if input == "true" { 7 }
    else if input == "false" { 3 }
    else if input == "nil" { 1 }
    else { 
      let parsed = input.parse::<i64>().expect("Invalid");
      if parsed < -4611686018427387904 || parsed > 4611686018427387903 {
        eprintln!("overflow");
        std::process::exit(1);
      }
      else { parsed << 1}
    }
    // TODO: parse the input string into internal value representation
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::<i64>::with_capacity(1000000); //1M words (8 million bytes)
    let buffer : *mut i64 = memory.as_mut_ptr();

    let i: i64 = unsafe { our_code_starts_here(input, buffer) };
    snek_print(i);
}
