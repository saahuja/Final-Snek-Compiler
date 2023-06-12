#![allow(dead_code)]


use im::{HashMap, HashSet};
use crate::compiler::{Definition::Fun0, Definition::Fun1, Definition::Fun2, Definition::FunN};

//Diamondback completed by Sahil Ahuja


//Program type stores all Definitions with Function abstract types
#[derive(Debug)]
pub struct Program {
  pub defs: Vec<Definition>,
  pub main: Expr,
  pub names: HashMap<String, i32>,
}

//Definitions for 0, 1, 2, or > 2 args
#[derive(Debug)]
pub enum Definition {
  Fun0(String, Expr),
  Fun1(String, String, Expr),
  Fun2(String, String, String, Expr),
  FunN(String, Vec<String>, Expr), //for 3 or more parameters
}

#[derive(Debug)]
pub enum Val {
    Reg(Reg),
    Imm(i64),
    SysImm(i32),
    RegOffset(Reg, i32),
    RegRegOffset(Reg, Reg),
    Label(String),
}

#[derive(Debug)]
pub enum Reg {
    RAX,
    RSP,
    RSI,
    RDI,
    RBX,
    RCX,
    RDX,
    R15,
}

#[derive(Debug)]

pub enum Instr {
    IMov(Val, Val),
    IMovQWord(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    NEG(Val),
    SAR(Val, Val),
    SHL(Val, Val),
    SHR(Val, Val),
    AND(Val, Val),
    OR(Val, Val),
    XOR(Val, Val),
    TEST(Val, Val),
    JMP(Val),
    JE(Val),
    JO(Val),
    JNE(Val),
    JG(Val),
    JL(Val),
    JZ(Val),
    JNZ(Val),
    CMP(Val, Val),
    CMOVE(Val, Val),
    CMOVG(Val, Val),
    CMOVL(Val, Val),
    CMOVGE(Val, Val),
    CMOVLE(Val, Val),
    Label(String),
    Call(String),
    Ret(),
}

#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
    Divide2,
    IsNum, 
    IsBool,
    Print,
}

#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    EqualArr,
    Equal,
    Greater, 
    GreaterEqual, 
    Less, 
    LessEqual,
}

#[derive(Debug)]
pub enum Expr {
    Nil,
    Number(i64),
    Boolean(bool),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Call0(String),
    Call1(String, Box<Expr>),
    Call2(String, Box<Expr>, Box<Expr>),
    CallN(String, Vec<Expr>),
    Array(Box<Expr>, Vec<Expr>),  //First is an expression that should evaluate to a size, second is for a vector of the values to store on heap (optinal, sice default is going to be nil)
    Index(Box<Expr>, Box<Expr>), //first expression evaluates to heap-allocated value and second evaluates to a number (index) -> look up using index at heap-allocated value
    SetArr(Box<Expr>, Box<Expr>, Box<Expr>) //1st position - heap val, 2nd position index, 3rd position, new value to be set
    //SetIndex(Box<Expr>, Box<Expr>), //set the heap allocated value at a specific index
}

//Adapted from course code
//Gets depth for when we need to create pre-allocate stack for function definition or evaluation
//TODO: could this be causing error with recursive factorial?
fn depth(e: &Expr) -> i32 {
  match e{
    Expr::Nil => 0,
    Expr::Array(size, vals) => { //max of all expressions + number of vals computed
        let mut to_return:i32 = depth(size);
        for i in 0..vals.len() {
          to_return = to_return.max(depth(&vals[i]) + 1)
        }
        to_return = to_return.max(vals.len() as i32); //needs at least args.len()
        to_return + 1
    },
    Expr::Index(heap_val, index) => {
        return depth(heap_val).max(depth(index) + 1) + 1;
    },
    Expr::SetArr(heap_val, index, new_val) => {
        return depth(heap_val).max(depth(index) + 1).max(depth(new_val) + 1) + 1;
    },
    Expr::Number(_) => 0,
    Expr::Boolean(_) => 0,
    Expr::BinOp(_,expr1 , expr2) => depth(expr1).max(depth(expr2) + 1),
    Expr::UnOp(_, expr) => depth(expr),
    Expr::Let(bindings, body) => {
      if bindings.is_empty(){
        eprintln!("Invalid bindings expression - depth func compile error");
        std::process::exit(1);
      }
      let mut to_return:i32 = depth(&bindings[0].1);
      for i in 1..bindings.len() {
        to_return = to_return.max(depth(&bindings[i].1) + 1)
      }
      to_return = to_return.max(depth(body) + 1);
      to_return = to_return.max(bindings.len() as i32);
      return to_return + 1;
    }, 
    Expr::Id(_) => 0,
    Expr::If(expr1, expr2, expr3) => depth(expr1).max(depth(expr2)).max(depth(expr3)) + 1,
    Expr::Loop(expr) => depth(expr) + 1,
    Expr::Block(exprs) => exprs.iter().map(|expr| depth(expr)).max().unwrap_or(0) + 1,
    Expr::Break(expr) => depth(expr) + 1,
    Expr::Set(_, expr) => depth(expr) + 1,
    Expr::Call0(_) => 0,
    Expr::Call1(_, expr) => depth(expr).max(1), //needs at least 1
    Expr::Call2(_, expr1, expr2) => depth(expr1).max(depth(expr2) + 1).max(2), //needs at least 2
    Expr::CallN(_, args) => {
        if args.is_empty(){
            eprintln!("Invalid number of args (no args)");
            std::process::exit(1);
        }
        let mut to_return:i32 = depth(&args[0]);
        for i in 1..args.len() {
          to_return = to_return.max(depth(&args[i]) + 1)
        }
        to_return = to_return.max(args.len() as i32); //needs at least args.len()
        to_return + 1
    }
  }
}

//Adapted from course code
//compiles the definition into assembly 
fn compile_definition(d: &Definition, labels: &mut i32, names: &HashMap<String, i32>) -> Vec<Instr> {
  match d {
      Fun0(name, body) => {
        let mut depth = depth(body);

        //ensures 16 byte alignment always (for caller registers)
        if depth % 2 != 0{
            depth += 1;
        }

        //depth = (depth*2);
        let offset = depth * 8;
        
        let body_env:HashMap<String, i32> = {
            let temp_map = HashMap::new();
            temp_map
        };

        let body_is = compile_to_instrs(body, 0, &body_env, labels, &String::from(""), names);
        let mut instrs = Vec::new();

        instrs.push(Instr::Label(name.to_string()));
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.extend(body_is);
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.push(Instr::Ret());

        return instrs;
      },
      Fun1(name, arg, body) => {
          let mut depth = depth(body);
          //let offset = depth * 8;
          /* 
          let body_env = hashmap! {
              arg.to_string() => depth + 1
          };
          */

          //ensures 16 byte alignment always
          if depth % 2 != 0{
            depth += 1;
          }

          //depth = (depth*2);
          let offset = depth * 8;

          let body_env:HashMap<String, i32> = {
              let mut temp_map = HashMap::new();
              temp_map.insert(arg.to_string(), depth + 1);
              temp_map
          };
          let body_is = compile_to_instrs(body, 0, &body_env, labels, &String::from(""), names);
          let mut instrs = Vec::new();

          instrs.push(Instr::Label(name.to_string()));
          instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
          instrs.extend(body_is);
          instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
          instrs.push(Instr::Ret());
          /*
          format!(
              "{name}:
              sub rsp, {offset}
              {body_is}
              add rsp, {offset}
              ret
          ")
          */
          return instrs;
      }
      Fun2(name, arg1, arg2, body) => {
          let mut depth = depth(body);
  
          //ensures 16 byte alignment always
          if depth % 2 != 0{
            depth += 1;
          }
          //depth = (depth*2); //rsp-128 maximum
          let offset = depth * 8;
       
          let body_env:HashMap<String, i32> = {
              let mut temp_map = HashMap::new();
              temp_map.insert(arg1.to_string(), depth + 1);
              temp_map.insert(arg2.to_string(), depth + 2);
              temp_map
          };
          let body_is = compile_to_instrs(body, 0, &body_env, labels, &String::from(""), names);
          let mut instrs = Vec::new();
          instrs.push(Instr::Label(name.to_string()));
          instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
          instrs.extend(body_is);
          instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
          instrs.push(Instr::Ret());
          return instrs;
      }
      FunN(name, args, body) => {
        let mut depth = depth(body);

        //ensures 16 byte alignment always
        if depth % 2 != 0{
            depth += 1;
        }
        //depth = (depth*2); //rsp-128 maximum
        //ensures 16 byte alignment always (for caller registers)
        let offset = depth * 8;

        let body_env:HashMap<String, i32> = {
            let mut temp_map = HashMap::new();
            let mut counter = 1;
            for arg in args{
                temp_map.insert(arg.to_string(), depth + counter);
                counter += 1;
            }
            temp_map
        };

        let body_is = compile_to_instrs(body, 0, &body_env, labels, &String::from(""), names);
        let mut instrs = Vec::new();
        instrs.push(Instr::Label(name.to_string()));
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.extend(body_is);
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.push(Instr::Ret());
        return instrs;
      }
  }
}


//Adapted from course code
//public function for compiling the full program including definitions and main 
pub fn compile_program(p: &Program) -> (String, String) {
  let mut labels : i32 = 0;
  //let mut defs : String = String::new();
  let mut defs: Vec<Instr> = Vec::new();

  for def in &p.defs[..] {
      //defs.push_str(&compile_definition(&def, &mut labels));
      defs.extend(compile_definition(def, &mut labels, &p.names));
  }
  let mut depth = depth(&p.main);
  
  let si = 0;
  let env: HashMap<String, i32> = HashMap::new();
  let brake = "".to_string();
  //let main_instrs = compile_to_instrs(&p.main, si, &env, &brake, &mut labels);

 //ensures 16 byte alignment always
  if depth % 2 != 0{
    depth += 1;
  }
  let offset = depth * 8;


  let mut main_with_offsetting_vec:Vec<Instr> = Vec::new();
  main_with_offsetting_vec.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
  main_with_offsetting_vec.extend(compile_to_instrs(&p.main, si, &env, &mut labels, &brake, &p.names));
  main_with_offsetting_vec.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));


  let defs_string = defs.iter().map(instr_to_str).collect::<Vec<String>>().join("\n");
  let main_with_offsetting_string = main_with_offsetting_vec.iter().map(instr_to_str).collect::<Vec<String>>().join("\n  ");
  return (defs_string, main_with_offsetting_string)
}





// Adapted from compiler #14
// Compile and Expr to a vector of instructions
fn compile_to_instrs(e: &Expr, si: i32, env: &HashMap<String, i32>, label_count: &mut i32, break_target: &String, names: &HashMap<String, i32>) -> Vec<Instr> {
  match e {
      Expr::Nil => {
        let mut instrs:Vec<Instr> = Vec::new();
        instrs.push(Instr::XOR(Val::Reg(Reg::RAX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::SysImm(1)));
        return instrs;
      }
      Expr::Array(size, vec) => {

        let mut instrs:Vec<Instr> = Vec::new();
        instrs.extend(compile_to_instrs(size, si, env, label_count, break_target, names));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)));  //save size variable at RSP+si


        //check if number of array elements provided (vec.len() is greater than what size is)
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(vec.len() as i32)));
        instrs.push(Instr::IMul(Val::Reg(Reg::RBX), Val::SysImm(2))); //multiply by 2 since size is also a multiple of 2
        instrs.push(Instr::CMP(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX))); //rax has the index_val and R15 has the heap val has the size as a val at [R15]
        instrs.push(Instr::JG(Val::Label("throw_too_many_elements_error".to_string())));




        instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::SysImm(8))); //multiply the index value by 8
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::SAR(Val::Reg(Reg::RBX), Val::SysImm(1))); //divide by 2 since we don't loop to loop double the maount

        //initialize all [R15 + 1*8] to [R15 + (1+ size)*8] to 1 
        let start_label = new_label(label_count, "arr_loopstart");
        //let end_label = new_label(label_count, "arr_loopend");

        instrs.push(Instr::Label(start_label.clone()));
        instrs.push(Instr::IMovQWord(Val::RegRegOffset(Reg::R15, Reg::RBX), Val::SysImm(1)));
        instrs.push(Instr::ISub(Val::Reg(Reg::RBX), Val::SysImm(8))); //rbx has the size so from rsp + size to 1
        instrs.push(Instr::JNZ(Val::Label(start_label.clone()))); //jump back to start_label if not 0

        if vec.len() > 0 {
            let num_elements: i32 = vec.len() as i32;

            for i in 0..vec.len() {
                instrs.extend(compile_to_instrs(&vec[i], si + (i as i32) + 1, env, label_count, break_target, names));
                if i != vec.len() - 1{
                    instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + (i as i32) + 1), Val::Reg(Reg::RAX)));
                } else{
                    instrs.push(Instr::IMov(Val::RegOffset(Reg::R15, num_elements), Val::Reg(Reg::RAX)))
                }
            }

            //values to initialize with 

            

            for i in 0..vec.len()-1{ //everything except last element vec element 
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si + (i as i32) + 1)));
                instrs.push(Instr::IMov(Val::RegOffset(Reg::R15, (i as i32) + 1), Val::Reg(Reg::RAX)))
            }  //[R15 will have the size]
        }

        //for size 
        instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::R15, 0), Val::Reg(Reg::RAX)));


        instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::R15))); //move R15 into RAX (toReturn)
        instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::SysImm(1))); //add 1 to indicate its an array type

        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)));  //save size variable at RSP+si
        instrs.push(Instr::IAdd(Val::Reg(Reg::RBX), Val::SysImm(1))); //add 1
        instrs.push(Instr::IMul(Val::Reg(Reg::RBX), Val::SysImm(8))); //multiply the index value by 8
        instrs.push(Instr::IAdd(Val::Reg(Reg::R15), Val::Reg(Reg::RBX))); //add (1 + size of elements * 8) to R15
        return instrs;
      },
      Expr::Index(heap_val, index_val) => {
        let mut instrs:Vec<Instr> = Vec::new();
        //let new_label = new_label(label_count, "nil_lbl");

        instrs.extend(compile_to_instrs(heap_val, si, env, label_count, break_target, names));

        //instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(1))); //if its equal to 1
        //instrs.push(Instr::JE(Val::Label(new_label.clone())));
      

        //check that heap_val ends in 01
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::AND(Val::Reg(Reg::RBX), Val::SysImm(3))); //get last two bits only
        instrs.push(Instr::CMP(Val::Reg(Reg::RBX), Val::SysImm(1))); //if its equal to 1
        instrs.push(Instr::JNE(Val::Label("throw_tag_check_error".to_string()))); //jump to throw_tag_check if not equal to 1

        //proceed with code
        instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::SysImm(1))); //sub 1 to indicate its an array type
        instrs.push(Instr::IMov(Val::Reg(Reg::R15), Val::Reg(Reg::RAX)));

       

        instrs.extend(compile_to_instrs(index_val, si+1, env, label_count, break_target, names)); //in rax
     
        //instrs.push(Instr::SAR(Val::Reg(Reg::RAX), Val::SysImm(1))); //divide by 2 since we don't want size to be stored shifted by 2
        instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::RegOffset(Reg::R15, 0))); //rax has the index_val and R15 has the heap val has the size as a val at [R15]
        instrs.push(Instr::JG(Val::Label("throw_bounds_error".to_string()))); //index can't be greater than the size specified

        instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(0)));  //index can't be less than 0 (where index 0 is the size)
        instrs.push(Instr::JL(Val::Label("throw_bounds_error".to_string()))); 




        //otherwise now get the heap value at the associated index
        instrs.push(Instr::SAR(Val::Reg(Reg::RAX), Val::SysImm(1)));
        instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::SysImm(8))); //multiply the index value by 8
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX))); //move index * 8 into RBX and move into RAX
        instrs.push(Instr::IMovQWord(Val::Reg(Reg::RAX), Val::RegRegOffset(Reg::R15, Reg::RBX))); //load [R15 + (index_val * 8) into RAX]
        
        
        //instrs.push(Instr::Label(new_label.clone()));
        //instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(0))); //move 0 into the register (represents 0 size)
        
        return instrs;

        //instrs.push(Instr::IMov(Val::Reg(Reg::R15), Val::Reg)
        //instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::RegOffset(Reg::R15, 0));
        //need to check if index_val given is out of bounds or not by comparing to R15 size (also ensure that R15's val is actually a size)
      },
      Expr::SetArr(heap_val, index_val, new_val) => {

        //1. check if provided index_val is greater than heap_val size
        let mut instrs:Vec<Instr> = Vec::new();

        instrs.extend(compile_to_instrs(heap_val, si, env, label_count, break_target, names));

        //check that heap_val ends in 01
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::AND(Val::Reg(Reg::RBX), Val::SysImm(3))); //get last two bits only
        instrs.push(Instr::CMP(Val::Reg(Reg::RBX), Val::SysImm(1))); //if its equal to 1
        instrs.push(Instr::JNE(Val::Label("throw_tag_check_error".to_string()))); //jump to throw_tag_check if not equal to 1

        //proceed with code
        instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::SysImm(1))); //sub 1 to indicate its an array type
        instrs.push(Instr::IMov(Val::Reg(Reg::R15), Val::Reg(Reg::RAX))); //set the starting address in R15



        instrs.extend(compile_to_instrs(index_val, si+1, env, label_count, break_target, names)); //compute the index value
        //instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::SysImm(8))); //multiply the index value by 8
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + 1), Val::Reg(Reg::RAX))); //move to [RSP + (si + 1)*8]
     
        //cmp [R15 + 0] with RAX
        instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::RegOffset(Reg::R15, 0))); //rax has the index_val and R15 has the heap val has the size as a val at [R15]
        instrs.push(Instr::JG(Val::Label("throw_bounds_error".to_string())));

        instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(0))); 
        instrs.push(Instr::JL(Val::Label("throw_bounds_error".to_string()))); //can't be less than 0 (where 0 is the size index)

        //2. compute new value which now overwrites RAX
        instrs.extend(compile_to_instrs(new_val, si + 2, env, label_count, break_target, names));
       

        //RAX has new val
        //R15 has the starting address of the heap array


        //3. move the index value into RBX
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si+1))); //store into RBX
        instrs.push(Instr::SAR(Val::Reg(Reg::RBX), Val::SysImm(1))); //divide by 2 (sar by 1)
        instrs.push(Instr::IMul(Val::Reg(Reg::RBX), Val::SysImm(8))); //multiply by 8

        //mov [R15 + index_val * 8], register with new val
        instrs.push(Instr::IMovQWord(Val::RegRegOffset(Reg::R15, Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::R15))); //move R15 into RAX
        instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::SysImm(1))); //add 1 to indicate its an array type
        return instrs;
      },
      Expr::Call0(funname) => {
        if names.contains_key(funname) == false{
            eprintln!("Invalid - no function defintion named {}", funname);
            std::process::exit(1);
        }
        if names[funname] != 0{
            eprintln!("Invalid - there should be 0 args");
            std::process::exit(1);
        }
        let offset = 1 * 8;
        let mut instrs:Vec<Instr> = Vec::new();
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RDI))); //[rsp+0]
        instrs.push(Instr::Call(funname.to_string()));
        instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 0))); //[rsp + 0]
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        return instrs
      },
      Expr::Call1(funname, arg) => {
        if names.contains_key(funname) == false{
            eprintln!("Invalid - no function defintion named {}", funname);
            std::process::exit(1);
        }
        if names[funname] != 1{
            eprintln!("Invalid - there should be 1 arg");
            std::process::exit(1);
        }
        let mut instrs = compile_to_instrs(arg, si, env, label_count, break_target, names);
        let offset = 2 * 8; //one word for rdi, one for arg
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RAX))); //[rsp]
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 1), Val::Reg(Reg::RDI))); //[rsp+8]
        instrs.push(Instr::Call(funname.to_string()));
        instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 1))); //[rsp + 8]
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)));
        return instrs
      },
      Expr::Call2(funname, arg1, arg2) => {
        if names.contains_key(funname) == false{
            eprintln!("Invalid - no function defintion named {}", funname);
            std::process::exit(1);
        }
        if names[funname] != 2{
            eprintln!("Invalid - there should be 2 args");
            std::process::exit(1);
        }
        //instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(offset)));
        let mut instrs = compile_to_instrs(arg1, si, env, label_count, break_target, names);
        let offset: i32 = 3 * 8; //one word rdi, one for arg1, one for arg2
        //let curr_word:i32 = si * 8;
        //let curr_word_after_sub = offset + curr_word;
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX))); //save at arg1 at RSP-{si*8}
        instrs.extend(compile_to_instrs(arg2, si+1, env, label_count, break_target, names)); //save arg2 in RAX
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset))); //create new stack
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, 3+si)));           //need to move from RSP-si*8 to the new RSP address (use RBX intermediate)
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RBX)));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 1), Val::Reg(Reg::RAX)));
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 2), Val::Reg(Reg::RDI)));
        instrs.push(Instr::Call(funname.to_string()));
        instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 2))); //restore rdi
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset))); //pop off new stack
        return instrs
      },
      Expr::CallN(funname, args) => {
        if names.contains_key(funname) == false{
            eprintln!("Invalid - no function defintion named {}", funname);
            std::process::exit(1);
        }
        if names[funname] != (args.len() as i32){
            eprintln!("Invalid - there should be {} args", names[funname]);
            std::process::exit(1);
        }
        let num_args = args.len();
        if num_args < 3{
            eprintln!("Invalid number of arguments - must be greater than 2 for CallN");
            std::process::exit(1);
        }
        let mut instrs = Vec::new();
        let offset: i32 = ((1 + num_args) * 8) as i32; //one for rdi + rest for args

        
        for i in 0..num_args{ //from 0 to num_args - 1
            instrs.extend(compile_to_instrs(&args[i], si+(i as i32), env, label_count, break_target, names));
            if i != num_args - 1{
                instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si+(i as i32)), Val::Reg(Reg::RAX))); //save at arg_i at RSP+{(si+i)*8}
            }
        }
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset))); //create new stack
        for i in 0..(num_args-1){
            instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, (num_args as i32 + 1) + si + (i as i32))));           //need to move from RSP-si*8 to the new RSP address (use RBX intermediate)
            instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, i as i32), Val::Reg(Reg::RBX)));
        }
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, (num_args - 1) as i32), Val::Reg(Reg::RAX))); //final arg is in RAX move to [rsp + (num_args-1 * 8)]
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, num_args as i32), Val::Reg(Reg::RDI))); //store rdi at [rsp+ (num_args * 8)]
        instrs.push(Instr::Call(funname.to_string())); //call func funname
        instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, num_args as i32))); //restore rdi
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset))); //pop off new stack
        return instrs;


        /* 
        instrs.extend(compile_to_instrs(&args[0], si, env, label_count, break_target, names)); //first arg
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX))); //save at arg1 at RSP+{si*8}
        instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset))); //create new stack
        instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, (num_args as i32 + 1) + si))); 
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RBX))); 
        for i in 1..(num_args-1){
            instrs.extend(compile_to_instrs(&args[i], si + (num_args as i32 + 1), env, label_count, break_target, names));
            instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, (num_args as i32 + 1) + si)));           //need to move from RSP-si*8 to the new RSP address (use RBX intermediate)
            instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, i as i32), Val::Reg(Reg::RBX)));
        }
        instrs.extend(compile_to_instrs(&args[num_args - 1], si, env, label_count, break_target, names)); //last argument
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, num_args as i32 - 1), Val::Reg(Reg::RAX))); //final arg is in RAX move to [rsp + (num_args-1 * 8)]
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, num_args as i32), Val::Reg(Reg::RDI))); //store rdi at [rsp+ (num_args * 8)]
        instrs.push(Instr::Call(funname.to_string())); //call func funname
        instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, num_args as i32))); //restore rdi
        instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset))); //pop off new stack
        return instrs;

        */

        /*
        for (i as i32) in 0..num_args{ //from 0 to num_args - 1
            instrs.extend(compile_to_instrs(&args[i], si + i, env, label_count, break_target, names));
            if i != num_args - 1{
                instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + i), Val::Reg(Reg::RAX))); //save at arg_i at RSP+{(si+i)*8} except for the last arg index
            }
        } */        
      }
      // Numbers, Booleans, and Ids move their corresponding value into RAX
      Expr::Number(n) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n))],
      Expr::Boolean(b) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(if *b { 7 } else { 3 }))],
      Expr::Id(s) => match s.as_str() {
          "input" => {
              vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI))]  
          }
          _ => {
              vec![Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, match env.get(s) {
                  Some(i) => *i,
                  None => {
                    eprintln!("Unbound variable identifier {}", s);
                    std::process::exit(1);
                  },

              }))]
          }
      }
      Expr::Let(bindings, body) => {
          check_duplicate_bindings(bindings);
          let mut instrs = Vec::new();
          let mut new_env = env.clone();
          for (i, (s, e)) in bindings.iter().enumerate() {
              instrs.extend(compile_to_instrs(e, si + i as i32, &new_env, label_count, break_target, names));     // Compile each binding
              instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + i as i32), Val::Reg(Reg::RAX)));       // Store the result on the stack
              new_env.insert(s.clone(), si + i as i32);                                                    // Store the stack index in the environment
          }
          instrs.extend(compile_to_instrs(body, si + bindings.len() as i32, &new_env, label_count, break_target, names));
          instrs
      }
      Expr::UnOp(op, e) => { //isbool (last two bits are 11) isarray(last two bits are 01) isnum(last two bits are 10 or 00)
          let mut instrs = compile_to_instrs(e, si, env, label_count, break_target, names);
          instrs.extend(match op {
              Op1::Add1 | Op1::Sub1 | Op1::Divide2 => vec![Instr::TEST(Val::Reg(Reg::RAX), Val::SysImm(1)),  // Checks if argument is numeric
                                            Instr::JNE(Val::Label("throw_type_error".to_string()))],
              Op1::IsBool | Op1::IsNum | Op1::Print => vec![] // No type checking needed
          });



          //if last bit is 0 -> flip to 1 (then left shift twice) 100 and add 3
          instrs.extend(match op {
              Op1::Add1 => vec![Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1))],
              Op1::Sub1 => vec![Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1))],
              Op1::Divide2 => {

                let odd_label = new_label(label_count, "odd");
                let end_label = new_label(label_count, "end_odd");
                
                vec![
                    Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),
                    Instr::SAR(Val::Reg(Reg::RBX), Val::SysImm(1)),
                    Instr::TEST(Val::Reg(Reg::RBX), Val::SysImm(1)),
                    Instr::JNZ(Val::Label(odd_label.clone())),
                    Instr::JMP(Val::Label(end_label.clone())),
                    Instr::Label(odd_label),
                    Instr::IAdd(Val::Reg(Reg::RAX), Val::SysImm(2)),
                    Instr::Label(end_label),
                    Instr::SAR(Val::Reg(Reg::RAX), Val::SysImm(1))]
              }
              Op1::IsNum => vec![Instr::AND(Val::Reg(Reg::RAX), Val::SysImm(1)),   // Mask out all but the tag bit //last bit needs to be 0 to be a num
                                 Instr::XOR(Val::Reg(Reg::RAX), Val::SysImm(1)),   // Flip the tag bit
                                 Instr::SHL(Val::Reg(Reg::RAX), Val::SysImm(2)),   // Shift the tag bit once to the left
                                 Instr::IAdd(Val::Reg(Reg::RAX), Val::SysImm(3))], // Make the new tag bit 1
              Op1::IsBool => {
                
                                  let else_label = new_label(label_count, "last_11");
                                  let end_label = new_label(label_count, "end_11");
                                  vec![
                                  Instr::AND(Val::Reg(Reg::RAX), Val::SysImm(3)),     // Mask out all but the two tag bits  
                                  Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                  Instr::JE(Val::Label(else_label.clone())),
                                  Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                  Instr::JMP(Val::Label(end_label.clone())),
                                  Instr::Label(else_label.clone()),
                                  Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(7)),
                                  Instr::Label(end_label.clone())]
              }
              Op1::Print => vec![]

            // Instr::XOR(Val::Reg(Reg::RCX), Val::Reg(RCX)), //clear RCX
              //boolean means last two bits is 11 
              //array means last  two bits is 01
              //number means last two bits is 00

          });
          instrs.extend(match op {
              Op1::Add1 | Op1::Sub1 | Op1::Divide2 => vec![Instr::JO(Val::Label("throw_overflow_error".to_string()))],
              Op1::IsBool | Op1::IsNum | Op1::Print => vec![] 
          });
          if let Op1::Print = op { //if Print
            let index = if si % 2 == 1 {si + 2} else {si + 1}; //rdi is 16 byte aligned, so this condition ensures multiple of 2 words
            let offset = index * 8;
            instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
            instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RDI)));          //mov [RSP], RDI (push RDI)
            instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX))); //mov RDI RAX
            instrs.push(Instr::Call("snek_print".to_string()));                                                                            //call snek_print
            instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 0)));          //mov RDI, [RSP] (pop RDI)
            instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset))); //add RSP, si*8
          }
          instrs
      }
      Expr::BinOp(op, e1, e2) => {
          let mut instrs = compile_to_instrs(e1, si, env, label_count, break_target, names);                       // Calculate the left expression
          instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)));                                     //save the left expression in memory
          instrs.extend(compile_to_instrs(e2, si + 1, env, label_count, break_target, names));                     // Calculate the right expression, and save it in RAX
          instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + 1), Val::Reg(Reg::RAX))); 
        
          
          instrs.extend(match op {
              Op2::Equal => vec![Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),                       // Checks if both arguments are of the same type
                                 Instr::XOR(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)),
                                 Instr::AND(Val::Reg(Reg::RBX), Val::SysImm(3)),
                                 Instr::CMP(Val::Reg(Reg::RBX), Val::SysImm(0)), //2 LSB bits should be equal to 0 if their xors are the same 
                                 Instr::JNE(Val::Label("throw_type_error".to_string()))],
              Op2::Greater | Op2::GreaterEqual | 
              Op2::Less | Op2::LessEqual | Op2::Minus | 
              Op2::Plus | Op2::Times => vec![Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),           // Checks if both arguments are numeric
                                             Instr::OR(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)),
                                             Instr::TEST(Val::Reg(Reg::RBX), Val::SysImm(1)),
                                             Instr::JNE(Val::Label("throw_type_error".to_string())),
                                             //Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(0)),
                                             //Instr::JE(Val::Label("throw_divide_by_0_error".to_string())),
                                            ], 
              Op2::EqualArr => vec![
                Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)),
                Instr::XOR(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)),
                Instr::AND(Val::Reg(Reg::RBX), Val::SysImm(3)),
                Instr::CMP(Val::Reg(Reg::RBX), Val::SysImm(0)), //2 LSB bits should be equal to 0 if their xors are the same 
                Instr::JNE(Val::Label("throw_type_error".to_string())),

                //Instr::TEST(Val::Reg(Reg::RBX), Val::SysImm(3)),
                //Instr::JNE(Val::Label("throw_type_error".to_string()))

              ],
          });
          instrs.extend(match op {
              Op2::Plus => vec![Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si))],
              Op2::Minus => vec![Instr::IMov(Val::Reg(Reg::RCX), Val::Reg(Reg::RAX)), Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)), Instr::ISub(Val::Reg(Reg::RAX), Val::Reg(Reg::RCX))], 
              Op2::Times => vec![Instr::SAR(Val::Reg(Reg::RAX), Val::SysImm(1)),                            // Divide operand by two to counteract the effect of the multiplication
                                 Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si))],                                      
              Op2::Equal => vec![Instr::CMP(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)),              // Compare the two arguments
                                 Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(7)),                           // Prepare the true result in a register
                                 Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),                           // Premptively set the result to false
                                 Instr::CMOVE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))],                     // If the arguments are equal, set the result to true
              Op2::Greater => vec![Instr::CMP(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)),            // Each of the following comparison operations follow the same pattern as the equal operation
                                   Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(7)),
                                   Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                   Instr::CMOVG(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))],
              Op2::Less => vec![Instr::CMP(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)),
                                Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(7)),
                                Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                Instr::CMOVL(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))],
              Op2::GreaterEqual => vec![Instr::CMP(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)),
                                        Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(7)),
                                        Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                        Instr::CMOVGE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))],
              Op2::LessEqual => vec![Instr::CMP(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)),
                                     Instr::IMov(Val::Reg(Reg::RBX), Val::SysImm(7)),
                                     Instr::IMov(Val::Reg(Reg::RAX), Val::SysImm(3)),
                                     Instr::CMOVLE(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))],
              Op2::EqualArr => {
                
                let start_label = new_label(label_count, "arr_loopstart");
                let break_label = new_label(label_count, "break_loop");
                let end_label = new_label(label_count, "end_loop");

                let offset: i32 = 3 * 8; //one word rdi, one for arg1, one for arg2
                


                vec![
                    /* 
                //add the case if both are nil or one is nil
                
                //CHECKS PURELY FOR VALUE EQUIVALENCY (ALL COMPILE TIME)
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si + 1)),
                Instr::AND(Val::Reg(Reg::RAX), Val::SysImm(3)), // Mask out all but the two tag bits 
                Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(1)),  //Ensure ends with 1
                Instr::JNE(Val::Label("throw_type_error".to_string())),
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)),
                Instr::AND(Val::Reg(Reg::RAX), Val::SysImm(3)),
                Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(1)), //Ensure ends with 1
                Instr::JNE(Val::Label("throw_type_error".to_string())),
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si + 1)), //check that sizes are equivalent
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)),
                Instr::ISub(Val::Reg(Reg::RAX), Val::SysImm(1)),
                Instr::ISub(Val::Reg(Reg::RBX), Val::SysImm(1)),
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RAX, 0)), //dereference the address value
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RBX, 0)), //dereference the address value
                Instr::CMP(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                Instr::JNE(Val::Label(break_label.clone())), //ensure equal sizes 

                Instr::IMov(Val::Reg(Reg::RCX), Val::Reg(Reg::RAX)), //save size
                Instr::IMul(Val::Reg(Reg::RCX), Val::SysImm(8)), //multiply by 40
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)),
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si + 1)),

                
                Instr::IMov(Val::Reg(Reg::RDX), Val::SysImm(7)), //initialize return value with true

                Instr::Label(start_label.clone()),
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)),
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si + 1)), //check that sizes are equivalent
                Instr::ISub(Val::Reg(Reg::RAX), Val::SysImm(1)),
                Instr::ISub(Val::Reg(Reg::RBX), Val::SysImm(1)),
                Instr::IAdd(Val::Reg(Reg::RAX), Val::Reg(Reg::RCX)),
                Instr::IAdd(Val::Reg(Reg::RBX), Val::Reg(Reg::RCX)),   
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RAX, 0)), //dereference the address value
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RBX, 0)), //dereference the address value
                Instr::CMP(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)),
                Instr::JNE(Val::Label(break_label.clone())), //break loop if any two elements are not equivalent
                Instr::ISub(Val::Reg(Reg::RCX), Val::SysImm(8)),
                Instr::CMP(Val::Reg(Reg::RCX), Val::SysImm(0)), //if offset is 0
                Instr::JNZ(Val::Label(start_label.clone())), //if offset is not 0 continue loop
                Instr::JZ(Val::Label(end_label.clone())), //if offset is 0 break loop
                Instr::Label(break_label.clone()),
                Instr::IMov(Val::Reg(Reg::RDX), Val::SysImm(3)), //set to false
                Instr::Label(end_label.clone()),
                Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDX)),
                        */





                //CHECKS FOR CYCLIC (RUNTIME)
                Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si+1)), 
                Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, si)),
                Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)),
                Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RDI)),
                Instr::IMov(Val::RegOffset(Reg::RSP, 1), Val::Reg(Reg::RSI)),
                Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)),
                Instr::IMov(Val::Reg(Reg::RSI), Val::Reg(Reg::RBX)),
                Instr::Call("snek_eq".to_string()),
                Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 0)),
                Instr::IMov(Val::Reg(Reg::RSI), Val::RegOffset(Reg::RSP, 1)),
                Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset)),

              ]
              /*
                    let index = if si % 2 == 1 {si + 2} else {si + 1}; //rdi is 16 byte aligned, so this condition ensures multiple of 2 words
                    let offset = index * 8;
                    instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::SysImm(offset)));
                    instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, 0), Val::Reg(Reg::RDI)));          //mov [RSP], RDI (push RDI)
                    instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX))); //mov RDI RAX
                    instrs.push(Instr::Call("snek_print".to_string()));                                                                            //call snek_print
                    instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, 0)));          //mov RDI, [RSP] (pop RDI)
                    instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::SysImm(offset))); //add RSP, si*8
               */
            
              }
              
          });
          instrs.push(Instr::JO(Val::Label("throw_overflow_error".to_string())));
          instrs
      }
      Expr::If(cond, then, els) => {
          let end_label = new_label(label_count, "ifend");
          let else_label = new_label(label_count, "ifelse");
          let mut instrs = compile_to_instrs(cond, si, env, label_count, break_target, names); // Calculate the condition
          instrs.push(Instr::CMP(Val::Reg(Reg::RAX), Val::SysImm(3)));                  // Check if the condition is false
          instrs.push(Instr::JE(Val::Label(else_label.clone())));                       // Jump to else if the condition is false
          instrs.extend(compile_to_instrs(then, si, env, label_count, break_target, names));   // Calculate the then expression
          instrs.push(Instr::JMP(Val::Label(end_label.clone())));                       // Jump to the end
          instrs.push(Instr::Label(else_label));                                        // Else label
          instrs.extend(compile_to_instrs(els, si, env, label_count, break_target, names));    // Calculate the else expression
          instrs.push(Instr::Label(end_label));                                         // End label
          instrs  
      }
      Expr::Loop(body) => {
          let start_label = new_label(label_count, "loopstart");
          let end_label = new_label(label_count, "loopend");
          let mut instrs = vec![Instr::Label(start_label.clone())];                 // Start label
          instrs.extend(compile_to_instrs(body, si, env, label_count, &end_label, names)); // Calculate the body
          instrs.push(Instr::JMP(Val::Label(start_label)));                         // Jump to the start
          instrs.push(Instr::Label(end_label));                                     // End label
          instrs
      }
      Expr::Break(break_expr) => {
          if break_target == "" { 
            eprintln!("A break must be inside a loop");
            std::process::exit(1);
          }
          let mut instrs = compile_to_instrs(break_expr, si, env, label_count, break_target, names); // Calculate the break expression
          instrs.push(Instr::JMP(Val::Label(break_target.clone())));                          // Jump to the break target
          instrs
      }
      Expr::Set(var, val) => {
          if !env.contains_key(var) { 
            eprintln!("Unbound variable identifier {}", var); 
            std::process::exit(1);
          }
          let mut instrs = compile_to_instrs(val, si, env, label_count, break_target, names);      // Calculate the value
          instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, env[var]), Val::Reg(Reg::RAX))); // Move the value to the variable
          instrs
      }
      Expr::Block(exprs) => {
          let mut instrs = vec![];
          for e in exprs {
              instrs.extend(compile_to_instrs(e, si, env, label_count, break_target, names)); // Calculate each expression
          }
          instrs
      }
  }
}

// Check a list of let bindings for duplicate bindings
fn check_duplicate_bindings(bindings: &Vec<(String, Expr)>) {
    let mut seen = HashSet::new();
    for (s, _) in bindings {
        if seen.contains(s) {
            eprintln!("Duplicate binding: {}", s);
            std::process::exit(1);
        }
        seen.insert(s);
    }
}

// Create a new label with a given name
fn new_label(label_count: &mut i32, label_name: &str) -> String {
    let label = format!("{}{}", label_name, label_count);
    *label_count += 1;
    label
}
    
// Instr type to string assembly
fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(dst, src) => format!("mov {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::IMovQWord(dst, src) => format!("mov qword {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::IAdd(dst, src) => format!("add {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::ISub(dst, src) => format!("sub {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::IMul(dst, src) => format!("imul {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::NEG(dst) => format!("neg {}", val_to_str(dst)),
        Instr::SAR(dst, src) => format!("sar {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::SHL(dst, src) => format!("shl {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::SHR(dst, src) => format!("shr {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::AND(dst, src) => format!("and {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::OR(dst, src) => format!("or {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::XOR(dst, src) => format!("xor {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::TEST(dst, src) => format!("test {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::JNE(dst) => format!("jne {}", val_to_str(dst)),
        Instr::CMP(dst, src) => format!("cmp {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::CMOVE(dst, src) => format!("cmove {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::CMOVG(dst, src) => format!("cmovg {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::CMOVL(dst, src) => format!("cmovl {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::CMOVGE(dst, src) => format!("cmovge {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::CMOVLE(dst, src) => format!("cmovle {}, {}", val_to_str(dst), val_to_str(src)),
        Instr::JMP(dst) => format!("jmp {}", val_to_str(dst)),
        Instr::JE(dst) => format!("je {}", val_to_str(dst)),
        Instr::JO(dst) => format!("jo {}", val_to_str(dst)),
        Instr::JG(dst) => format!("jg {}", val_to_str(dst)),
        Instr::JL(dst) => format!("jl {}", val_to_str(dst)),
        Instr::JZ(dst) => format!("jz {}", val_to_str(dst)),
        Instr::JNZ(dst) => format!("jnz {}", val_to_str(dst)),
        Instr::Label(l) => format!("{}:", l),
        Instr::Call(fun) => format!("call {}", fun),
        Instr::Ret() => format!("ret"),
    }
}

// Val enum to its corresponding x86 assembly string
fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(r) => match r {
            Reg::RAX => "rax".to_string(),
            Reg::RSP => "rsp".to_string(),
            Reg::RBX => "rbx".to_string(),
            Reg::RDI => "rdi".to_string(),
            Reg::RCX => "rcx".to_string(),
            Reg::RDX => "rdx".to_string(),
            Reg::R15 => "r15".to_string(),
            Reg::RSI => "rsi".to_string(),
        },
        Val::Imm(i) => (i<<1).to_string(),
        Val::SysImm(i) => i.to_string(),
        Val::RegOffset(r, i) => {
            format!("[{}+{}]", {
                match r {
                    Reg::RAX => "rax".to_string(),
                    Reg::RBX => "rbx".to_string(),
                    Reg::RSP => "rsp".to_string(),
                    Reg::RDI => "rdi".to_string(),
                    Reg::RCX => "rcx".to_string(),
                    Reg::RDX => "rdx".to_string(),
                    Reg::R15 => "r15".to_string(),
                    Reg::RSI => "rsi".to_string(),
                }
            }, i * 8)
        },
        Val::RegRegOffset(r, r2) => { 
            format!("[{}+{}]", {
                match r {
                    Reg::RAX => "rax".to_string(),
                    Reg::RBX => "rbx".to_string(),
                    Reg::RSP => "rsp".to_string(),
                    Reg::RDI => "rdi".to_string(),
                    Reg::RCX => "rcx".to_string(),
                    Reg::RDX => "rdx".to_string(),
                    Reg::R15 => "r15".to_string(),
                    Reg::RSI => "rsi".to_string(),
                }
            }, {
                match r2 {
                    Reg::RAX => "rax".to_string(),
                    Reg::RBX => "rbx".to_string(),
                    Reg::RSP => "rsp".to_string(),
                    Reg::RDI => "rdi".to_string(),
                    Reg::RCX => "rcx".to_string(),
                    Reg::RDX => "rdx".to_string(),
                    Reg::R15 => "r15".to_string(),
                    Reg::RSI => "rsi".to_string(),
                }
            })
        },
        Val::Label(l) => l.to_string(),

        /*
        Val::RegOffset(r, i) => {
            if *i >= 0 {
                format!("[{}-{}]", {
                    match r {
                        Reg::RAX => "rax".to_string(),
                        Reg::RBX => "rbx".to_string(),
                        Reg::RSP => "rsp".to_string(),
                        Reg::RDI => "rdi".to_string(),
                        Reg::RCX => "rcx".to_string(),
                        Reg::RDX => "rdx".to_string(),
                    }
                }, i * 8)
            } else{
                format!("[{}+{}]", {
                    match r {
                        Reg::RAX => "rax".to_string(),
                        Reg::RBX => "rbx".to_string(),
                        Reg::RSP => "rsp".to_string(),
                        Reg::RDI => "rdi".to_string(),
                        Reg::RCX => "rcx".to_string(),
                        Reg::RDX => "rdx".to_string(),
                    }
                }, -i * 8)
            }
          
        }
        */
        
    }
}  


/*
// Public interface to compile Expr to x86 assembly
pub fn compile(e: &Expr) -> String {
    let instrs = compile_to_instrs(e, 2, &HashMap::new(), &mut 0, &"".to_string());
    instrs.iter().map(instr_to_str).collect::<Vec<String>>().join("\n  ")
}
*/