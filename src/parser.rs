use std::collections::HashSet;

use im::HashMap;
use sexp::*;
use crate::compiler::{Expr, Op1, Op2, Program, Definition, Definition::Fun0, Definition::Fun1, Definition::Fun2, Definition::FunN};

//Diamondback completed by Sahil Ahuja

//Notes for compiler static error checking
//There is a call to a function name that doesn't exist - Done
//Multiple functions are defined with the same name - Done
//A function's parameter list has a duplicate name - Done
//There is a call to a function with the wrong number of arguments - Done 
//input is used in a function definition (rather than in the expression at the end). It's worth thinking of that final expression as the main function or method - Done



//Public method for parsing a string into a Program (checks for valid s-expression)
pub fn parse_program_string(s:&str) -> Program{

    let prog = "(".to_owned() + &s + ")";
    let sexp = match parse(&prog) {
        Ok(s) => s,
        Err(e) => panic!("Invalid Parse error: {}", e),
    };
    return parse_program(&sexp);
}


// Public interface for parsing a string into an Expr
/* 
pub fn parse_string(s: &str) -> Expr {
    let sexp = match parse(s) {
        Ok(s) => s,
        Err(e) => panic!("Invalid Parse error: {}", e),
    };
    return parse_expr(&sexp, &HashSet::new());
}
*/


//parses s-expression into a Program type (adapted from course code)
fn parse_program(s: &Sexp) -> Program {
    match s {
        Sexp::List(vec) => {
            let mut defs_new: Vec<Definition> = vec![];
            let mut names_new: HashMap<String, i32> = HashMap::new();
            let mut counter = 0;
            for def_or_exp in vec {
                if is_def(def_or_exp) {
                    let (def, name, num_args) = parse_definition(def_or_exp);
                    if names_new.contains_key(&name){
                        panic!("Invalid - function name already defined")
                    }
                    defs_new.push(def);
                    names_new.insert(name, num_args);
                } else {
                    if counter != vec.len() - 1{
                        panic!("Invalid - all definitions should be before main or there should only be one main program")
                    }
                    return Program {
                        defs: defs_new,
                        main: parse_expr(def_or_exp, &false), //should only be one main in final position
                        names: names_new,
                    };
                }
                counter += 1;
            }
            panic!("Invalid - Only found definitions");
        }
        _ => panic!("Invalid - Program should be a list")
    }
}



//Checks if List type starts with keyword fun (from course code)
fn is_def(s: &Sexp) -> bool {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(Atom::S(keyword)), Sexp::List(_), _] if keyword == "fun" => true,
            _ => false
        }
        _ => false,
    }
}

//checks for duplicate arg names
fn check_duplicate_args(args:&Vec<String>) {
    let mut seen = HashSet::new();
    for arg in args {
        if seen.contains(arg) {
            panic!("Invalid - Parameter list has duplicate name")
        }
        seen.insert(arg);
    }
}




//parses function definition to a Definition type, and returns the definition name and number of args
fn parse_definition(s: &Sexp) -> (Definition, String, i32) {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(Atom::S(keyword)), Sexp::List(name_vec), body] if keyword == "fun" => match &name_vec[..] {
                
                [Sexp::Atom(Atom::S(funname))] => {
                    if !valid_fun_name(funname.as_str()) || reserved_keyword_check_bool(funname.as_str()){ //if not valid function name or a reserved keyword, args are also not reserved keywords
                        panic!("Invalid fun name , {funname}")
                    }
                    return (Fun0(funname.to_string(), parse_expr(body, &true)), funname.to_string(), 0)
                }
                [Sexp::Atom(Atom::S(funname)), Sexp::Atom(Atom::S(arg))] => {        
                    if !valid_fun_name(funname.as_str()) || reserved_keyword_check_bool(funname.as_str()){ //if not valid function name or a reserved keyword, args are also not reserved keywords
                        panic!("Invalid fun name , {funname}")
                    }
                    if reserved_keyword_check_bool(arg.as_str()){
                        panic!("Invalid arg name {arg}")
                    }
                    return (Fun1(funname.to_string(), arg.to_string(), parse_expr(body, &true)), funname.to_string(), 1) //indicate indef is true (so that any input keywords will be handled with a panic error)
                }
                [Sexp::Atom(Atom::S(funname)), Sexp::Atom(Atom::S(arg1)), Sexp::Atom(Atom::S(arg2))] => {
                    if !valid_fun_name(funname.as_str()) || reserved_keyword_check_bool(funname.as_str()){ //if not valid function name or a reserved keyword, args are also not reserved keywords
                        panic!("Invalid fun name , {funname}")
                    }
                    if reserved_keyword_check_bool(arg1.as_str()) || reserved_keyword_check_bool(arg2.as_str()){   //if valid function name and not a reserved keyword, args also are not reserved keywords
                        panic!("Invalid arg name pair {arg1} {arg2}") 
                    }
                    if arg1.to_string() == arg2.to_string(){ 
                        panic!("Invalid - Parameter list has duplicate name")
                    }
                    return (Fun2(funname.to_string(), arg1.to_string(), arg2.to_string(), parse_expr(body, &true)), funname.to_string(), 2) //indicate indef is true (so that any input keywords will be handled with a panic error)
                }, 
                _ => { //Case with N parameters (3 or more)
                    //match first Atom with 
                    let mut function_name = &"".to_string();
                    let mut counter = 0;

                    let mut args:Vec<String> = Vec::new();

                    if name_vec.len() <= 3 {
                        panic!("Invalid - Bad fundef")
                    }
                    for fun_param in name_vec{
                        if let Sexp::Atom(Atom::S(fun_or_arg)) = fun_param {
                            if counter == 0{
                                if !valid_fun_name(fun_or_arg.as_str()) || reserved_keyword_check_bool(fun_or_arg.as_str()){ //if not valid function name or a reserved keyword, args are also not reserved keywords
                                    panic!("Invalid fun name , {fun_or_arg}")
                                }
                                function_name = fun_or_arg;
                            } else{
                                if reserved_keyword_check_bool(fun_or_arg.as_str()){   //args can not be a reserved keyword
                                    panic!("Invalid arg name pair {fun_or_arg}") 
                                }
                                args.push(fun_or_arg.clone())
                            }
                        } else{
                            panic!("Invalid - Bad fundef")
                        }
                        counter+=1
                    }
                    check_duplicate_args(&args);

                    return (FunN(function_name.to_string(), args, parse_expr(&body, &true)), function_name.to_string(), counter-1);
                    /* 
                    if let Sexp::Atom(Atom::S(funname)) = &name_vec[0]{
                        if !valid_fun_name(funname) || reserved_keyword_check_bool(funname){ //if not valid function name or a reserved keyword, args are also not reserved keywords
                            panic!("Invalid fun name , {funname}")
                        }
                        function_name = funname.clone();
                    }
                    */
                },
            },
            _ => panic!("Invalid - Bad fundef"),
        },
        _ => panic!("Invalid - Bad fundef"),
    }
}

//(tuple <expr>+)
//


//Adapted from compiler #14
// Parse Sexp into Expr (if indef is true, ensures that no input keyword is parsed)
fn parse_expr(s: &Sexp, indef: &bool) -> Expr {
    match s {
        // Atomic string
        Sexp::Atom(Atom::S(s)) => {
            if s == "input" && *indef{
                panic!("Invalid, input cannot be in function definition")
            }
            match s as &str{
                "true" => Expr::Boolean(true),
                "false" => Expr::Boolean(false),
                "nil" => Expr::Nil,
                _ => Expr::Id(s.to_string()),
            }
        }
        // Atomic integer
        Sexp::Atom(Atom::I(i)) => {
            if *i < -4611686018427387904 || *i > 4611686018427387903 {
                panic!("Invalid");
            }
            return Expr::Number(i64::try_from(*i).unwrap())
        },
        // Lists of Sexp
        Sexp::List(v) => {
            if v.len() == 0 { panic!("Invalid: Zero length list"); }
            match &v[0] { 
                Sexp::Atom(Atom::S(s)) => { // First element must be an atomic string which denotes some operation
                    match s.as_str() {
                        "array" => {
                            if v.len() != 3 && v.len() != 2{
                                panic!("Invalid: Array has an unexpected number of arguments");
                            }
                            //let mut size = 0;
                            //2nd arg should be the size
                            /* 
                            if let Sexp::Atom(Atom::I(n)) = v[1]{
                                size = n;
                            } else {
                                panic!("Invalid: Array needs to specify the size in 1st argument")
                            }
                            */
                            let mut exprs:Vec<Expr> = Vec::new();
                            if v.len() == 3 {
                                if let Sexp::List(vec) = &v[2] {
                                    for expr in vec{
                                        exprs.push(parse_expr(expr, indef));
                                    }
                                }
                            } else if v.len() == 2{
                                if let Sexp::Atom(Atom::I(_)) = &v[1] {
                    
                                } else {
                                    panic!("Invalid: Array needs to specify an integer size");
                                }
                                
                            }  

                            return Expr::Array(Box::new(parse_expr(&v[1], indef)), exprs);
                        }, 
                        "index" => { //index, heap val, offset
                            if v.len() != 3 {
                                panic!("Invalid: Index has unexpected number of arguments");
                            }
                            return Expr::Index(Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)));
                        },
                        "setarr!" => {
                            if v.len() != 4 {
                                panic!("Invalid: setarr has unexpected number of arguments");
                            }
                            return Expr::SetArr(Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)), Box::new(parse_expr(&v[3], indef)));
                        },
                        "print" => {
                            if v.len() != 2{
                                panic!("Invalid: Print has an unexpected number of arguments");
                            }
                            return Expr::UnOp(Op1::Print, Box::new(parse_expr(&v[1], indef)));
                        }
                        "let" => {
                            if v.len() != 3 { panic!("Invalid: Let has an unexpected number of arguments"); }
                            let bindings = parse_binds(&v[1], indef);
                            let body = parse_expr(&v[2], indef);
                            Expr::Let(bindings, Box::new(body))
                        }
                        "set!" => {
                            if v.len() != 3 { panic!("Invalid: Set has an unexpected number of arguments"); }
                            let name = match &v[1] {
                                Sexp::Atom(Atom::S(s)) => s.to_string(),
                                _ => panic!("Invalid: Set has an unexpected argument"),
                            };
                            let body = parse_expr(&v[2], indef);
                            Expr::Set(name, Box::new(body))
                        }
                        "add1" => {
                            if v.len() != 2 { panic!("Invalid: Add1 has an unexpected number of arguments"); }
                            Expr::UnOp(Op1::Add1, Box::new(parse_expr(&v[1], indef)))
                        }
                        "sub1" => {
                            if v.len() != 2 { panic!("Invalid: Sub1 has an unexpected number of arguments"); }
                            Expr::UnOp(Op1::Sub1, Box::new(parse_expr(&v[1], indef)))
                        }
                        "+" => {
                            if v.len() != 3 { panic!("Invalid: Addition has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Plus, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "-" => {
                            if v.len() != 3 { panic!("Invalid: Subtraction has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Minus, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "*" => {
                            if v.len() != 3 { panic!("Invalid: Multiplication has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Times, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "div2" => {
                            if v.len() != 2 { panic!("Invalid: Division has an unexpected number of arguments"); }
                            Expr::UnOp(Op1::Divide2, Box::new(parse_expr(&v[1], indef)))
                        }
                        "isnum" => {
                            if v.len() != 2 { panic!("Invalid: Isnum has an unexpected number of arguments"); }
                            Expr::UnOp(Op1::IsNum, Box::new(parse_expr(&v[1], indef)))
                        }
                        "isbool" => {
                            if v.len() != 2 { panic!("Invalid: Isbool has an unexpected number of arguments"); }
                            Expr::UnOp(Op1::IsBool, Box::new(parse_expr(&v[1], indef)))
                        }
                        "<" => {
                            if v.len() != 3 { panic!("Invalid: Less than has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Less, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        ">" => {
                            if v.len() != 3 { panic!("Invalid: Greater than has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Greater, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        ">=" => {
                            if v.len() != 3 { panic!("Invalid: Greater than or equal to has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "<=" => {
                            if v.len() != 3 { panic!("Invalid: Less than or equal to has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "=" => {
                            if v.len() != 3 { panic!("Invalid: Equal to has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::Equal, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "==" => {
                            if v.len() != 3 { panic!("Invalid: Structural Equal to has an unexpected number of arguments"); }
                            Expr::BinOp(Op2::EqualArr, Box::new(parse_expr(&v[1], indef)), Box::new(parse_expr(&v[2], indef)))
                        }
                        "if" => {
                            if v.len() != 4 { panic!("Invalid: If has an unexpected number of arguments"); }
                            let cond = parse_expr(&v[1], indef);
                            let then = parse_expr(&v[2], indef);
                            let els = parse_expr(&v[3], indef);
                            Expr::If(Box::new(cond), Box::new(then), Box::new(els))
                        }
                        "block" => {
                            if v.len() < 2 { panic!("Invalid: Block must contain at least one expression"); }
                            let mut exprs = Vec::new();
                            for i in 1..v.len() {
                                exprs.push(parse_expr(&v[i], indef));
                            }
                            Expr::Block(exprs)
                        }
                        "loop" => {
                            if v.len() != 2 { panic!("Invalid: Loop has an unexpected number of arguments"); }
                            Expr::Loop(Box::new(parse_expr(&v[1], indef)))
                        }
                        "break" => {
                            if v.len() != 2 { panic!("Invalid: Break has an unexpected number of arguments"); }
                            Expr::Break(Box::new(parse_expr(&v[1], indef)))
                        }
                        _ => {
                            match &v[..] {
                                [Sexp::Atom(Atom::S(funname))] => {
                                    if reserved_keyword_check_bool(funname.as_str()) || !valid_fun_name(funname.as_str()){ //if reserved keyword
                                        panic!("Invalid")
                                    } else{
                                        return Expr::Call0(funname.to_string())
                                    }
                                },
                                [Sexp::Atom(Atom::S(funname)), arg] => {
                                    if reserved_keyword_check_bool(funname.as_str()) || !valid_fun_name(funname.as_str()){ //if reserved keyword
                                        panic!("Invalid")
                                    } else{
                                        return Expr::Call1(funname.to_string(), Box::new(parse_expr(arg, indef)))
                                    }
                                }, 
                                [Sexp::Atom(Atom::S(funname)), arg1, arg2] => {
                                    if reserved_keyword_check_bool(funname.as_str()) || !valid_fun_name(funname.as_str()){ //if reserved keyword
                                        panic!("Invalid")
                                    } else{
                                        return Expr::Call2(funname.to_string(), Box::new(parse_expr(arg1, indef)), Box::new(parse_expr(arg2, indef)))
                                    }
                                },
                                _ => {
                                    let mut args:Vec<Expr> = Vec::new();
                                    let mut counter = 0;
                                    let mut funname = "".to_string();

                                    for name_or_arg in v{
                                        if counter == 0{
                                            if let Sexp::Atom(Atom::S(function_name)) = name_or_arg{
                                                funname = function_name.to_string()
                                            }
                                            else{
                                                panic!("Invalid: Unrecognized operation")
                                            }
                                        } else{
                                            args.push(parse_expr(name_or_arg, indef)); //vec already is like a box type for expr (vector manages the allocation and resizing of buffer automatically)
                                        }
                                        counter += 1;
                                    }
                                    if args.len() < 3{ //this case is for 3 or greater args
                                        panic!("Invalid: Unrecognized operation")
                                    }
                                    return Expr::CallN(funname, args);
                                }
                                //[Sexp::Atom(Atom::S(funname))]
                                //_ => panic!("Invalid: Unrecognized operation"),
                            }
                        }
                        //_ => panic!("Invalid: Unrecognized operation")
                    }
                }
                _ => panic!("Invalid - parse error"),
            }
        },
        _ => panic!("Invalid - parse error"),
    }
}

//Borrowed from compiler #14
// Parse a list of Sexp bindings into Expr spec bindings
fn parse_binds(s: &Sexp, indef: &bool) -> Vec<(String, Expr)> {
    match s {
        Sexp::List(v) => {
            let mut bindings = Vec::new();
            if v.len() == 0 { panic!("Invalid: Let statement must contain at least one binding");   }
            for i in 0..v.len() {
                bindings.push(parse_bind(&v[i], indef));
            }
            bindings
        }
        _ => panic!("Invalid: Let bindings must be enclosed in parentheses"),
    }
}

//Borrowed from compiler #14
// Parse single Sexp binding into Expr spec binding
fn parse_bind(s: &Sexp, indef: &bool) -> (String, Expr) {
    match s {
        Sexp::List(v) => {
            if v.len() != 2 {
                panic!("Invalid: Bindings must have exactly two elements");
            }
            match &v[0] {
                Sexp::Atom(Atom::S(s)) => {
                    reserved_keyword_check(&s);
                    (s.clone(), parse_expr(&v[1], indef))
                }
                _ => panic!("Invalid Variable Name: Must be a string"),
            }
        }
        _ => panic!("Invalid"),
    }
}

//Borrowed from compiler #14
// Check if a string is a reserved keyword
fn reserved_keyword_check(s: &str) {
    match s {
        "let" => panic!("Invalid: Reserved keyword"),
        "add1" => panic!("Invalid: Reserved keyword"),
        "sub1" => panic!("Invalid: Reserved keyword"),
        "isnum" => panic!("Invalid: Reserved keyword"),
        "isbool" => panic!("Invalid: Reserved keyword"),
        "if" => panic!("Invalid: Reserved keyword"),
        "block" => panic!("Invalid: Reserved keyword"),
        "loop" => panic!("Invalid: Reserved keyword"),
        "break" => panic!("Invalid: Reserved keyword"),
        "true" => panic!("Invalid: Reserved keyword"),
        "false" => panic!("Invalid: Reserved keyword"),
        "input" => panic!("Invalid: Reserved keyword"),
        "fun" => panic!("Invalid: Reserved keyword"),
        _ => (),
    }
}

// Check if a string is a reserved keyword
fn reserved_keyword_check_bool(s: &str) -> bool {
    match s {
        "let" => true,
        "add1" => true,
        "sub1" => true,
        "isnum" => true,
        "isbool" => true,
        "if" => true,
        "block" => true,
        "loop" => true,
        "break" => true,
        "true" => true,
        "false" => true,
        "input" => true,
        "fun" => true,
        _ => false,
    }
}

// Check if a function name is a valid one (must start with letter (upper or lower and can have numbers or underscores afer))
fn valid_fun_name(s: &str) -> bool {
    let letters: Vec<char> = s.chars().collect();
    if let Some(&first) = letters.first() {
        if !first.is_alphabetic(){
            return false
        }
    } else {
        return false;
    }

    for i in 1..letters.len(){
        if letters[i] != '_' && !letters[i].is_alphanumeric(){
            return false
        }
    }
    return true
}