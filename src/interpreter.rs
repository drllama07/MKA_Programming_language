
use std::string;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;

use crate::{parser, scanner};
use crate::tokens::{Token, TokenType};
use crate::parser::{Expr, ForImpl};
use crate::environments::Environment;

pub fn f32_to_usize(value: f32) -> usize {
    if value >= 0.0 && value <= usize::MAX as f32 {
        Some(value as usize).unwrap()
    } else {
       panic!("You need a non_negative int value as index");
    }
}


#[derive(Debug)]
pub struct Interpreter {
    expr: Box<Expr>,
    return_val: f32,
    vec_assign: bool,
    fn_name: String,
}
impl Interpreter {
    pub fn new(expr: Expr, fn_name: String) -> Interpreter {
        Interpreter {expr: Box::new(expr), return_val: 0.0, vec_assign:false, fn_name: fn_name}
    }
    pub fn run(self,gl: &mut Environment) {
        self.evaluate(gl, &Token { kind: TokenType::Eof, value: "place holder".to_string() });
    }
    pub fn evaluate(mut self, gl: &mut Environment, assignment_token: &Token) -> f32 {
        match *self.expr {
            Expr::Import(x) => {
                let file_path = x.file_name.clone() + ".mka";
                if gl.check_import(&x.file_name) {
                    let mut file_content: String = fs::read_to_string(file_path.clone()).unwrap();
                    println!("Importing From: {}", file_path);
                    let tks = scanner::lexer(&mut file_content);
                    let mut pars = parser::parse(tks);
                    while pars.len() > 0{
                        let exp = pars.pop().unwrap();
                        let inter = Interpreter::new(exp, "_".to_string());
                        inter.evaluate(gl,  &Token { kind: TokenType::Eof, value: "place holder".to_string() });
                    }
                    return 4321.0;
                } else {
                    panic!("Error: Import_Error -> You cannot import modules more than onces or import circular: To import a file that is importing the file you are in")
                }
                
            }
            Expr::Number(x) => {
                return x.value;
            },
            Expr::BinaryOperation(x) => {
                let lf  = Interpreter::new(*x.left, self.fn_name.clone());
                let rh = Interpreter::new(*x.right, self.fn_name);

                match x.operation.kind {
                    TokenType::Plus => {
                        return lf.evaluate(gl, assignment_token) + rh.evaluate(gl, assignment_token);
                    },
                    TokenType::Minus => {
                        return lf.evaluate(gl, assignment_token) - rh.evaluate(gl, assignment_token);
                    },
                    TokenType::Star => {
                        return lf.evaluate(gl, assignment_token) * rh.evaluate(gl, assignment_token);
                    },
                    TokenType::Slash => {
                        return lf.evaluate(gl,assignment_token) / rh.evaluate(gl, assignment_token);
                    },
                    TokenType::Exp => {
                        return lf.evaluate(gl, assignment_token).powf(rh.evaluate(gl, assignment_token));
                    },
                    _ => panic!("Error -> Interpreter_BO: No operation like that??")
                }
            },
            Expr::Unary(x) => {
                let rh = Interpreter::new(*x.right, self.fn_name);
                return -1.0 * rh.evaluate(gl, assignment_token);

            }
            Expr::Variable(x) => {
                match x.name.kind { 
                    TokenType::Number(y) => return y,
                    TokenType::Id => {
                        return *gl.find_var_value(&x.name.value,&self.fn_name)
                    },
                    _ => panic!("Error -> Interpreter_Var"),

                }
            },
            Expr::Assignment(x) => {
                let expr = Interpreter::new(*x.value, self.fn_name);
                let val = expr.evaluate(gl, &x.target.name);
                if !self.vec_assign {
                    gl.add_var(x.target.name.value, val);
                    self.vec_assign = false;
                }
                return val;
            },
            Expr::Double(x) => {
                let expr = Interpreter::new(*x.value, self.fn_name.clone());
                let mut val: f32;
                if x.op == TokenType::PlusEqual {
                     val = expr.evaluate(gl, &x.target.name) + gl.find_var_value(&x.target.name.value, &self.fn_name);
                } else if  x.op == TokenType::MinusEqual{
                     val = expr.evaluate(gl, &x.target.name) - gl.find_var_value(&x.target.name.value, &self.fn_name);
                } else {
                    panic!("Not implemented yet other double operations!");
                }
                if !self.vec_assign {
                    gl.add_var(x.target.name.value, val);
                    self.vec_assign = false;
                }
                return val;
            },
            Expr::Fncall(x) => {
                let mut result: f32 = 1234.0;
                match x.name.value.as_str() {
                     "print" => {
                        for var in x.args.iter() {
                            let ex = Interpreter::new(var.borrow().clone(), self.fn_name.clone());
                            println!("-> {} ", ex.evaluate(gl, assignment_token));
                        }
                    },
                    "sin" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  ex.evaluate(gl, assignment_token).sin();
                        } else {
                            panic!("You cannot pass no more than 1 value to sin()")
                        }
                    },
                    "cos" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  ex.evaluate(gl, assignment_token).cos();
                        } else {
                            panic!("You cannot pass no more than 1 value to cos()")
                        }
                    },
                    "tan" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  ex.evaluate(gl, assignment_token).tan();
                        } else {
                            panic!("You cannot pass no more than 1 value to tan()")
                        }
                    },
                    "cot" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  1.0 / ex.evaluate(gl, assignment_token).tan();
                        } else {
                            panic!("You cannot pass no more than 1 value to cot()")
                        }
                    },
                    "csc" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  1.0 / ex.evaluate(gl, assignment_token).sin();
                        } else {
                            panic!("You cannot pass no more than 1 value to csc()")
                        }
                    },
                    "sec" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  1.0 / ex.evaluate(gl, assignment_token).cos();
                        } else {
                            panic!("You cannot pass no more than 1 value to sec()")
                        }
                    },
                    "ln" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            result =  ex.evaluate(gl, assignment_token).ln();
                        } else {
                            panic!("You cannot pass no more than 1 value to sec()")
                        }
                    },
                    "log" => {
                        if x.args.len() == 2 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            let base = Interpreter::new(x.args[1].borrow().clone(), self.fn_name.clone());
                            result =  ex.evaluate(gl, assignment_token).log(base.evaluate(gl, assignment_token));
                        } else {
                            panic!("You cannot pass no more than 2 values to log() -> 1: input 2: Base")
                        }
                    },
                    "factorial" => {
                        if x.args.len() == 1 {
                            let ex = Interpreter::new(x.args[0].borrow().clone(), self.fn_name.clone());
                            let mut re = 1;
                            for n in 1..=ex.evaluate(gl, assignment_token) as i32 {
                                re *= n
                            }
                            result = re as f32;
                        } else {
                            panic!("You cannot pass no more than 1 value to factorial()")
                        }
                    },
                    "push" => {
                        if x.args.len() == 3 {
                            let mut vec_name: String;
                            let mut index: u32;
                            let mut value: f32;
                            match x.args[0].borrow().clone() {
                                Expr::Variable(var) => vec_name=var.name.value,
                                _ => panic!("You can only enter vector names to push()")
                            }
                            let ex =  Interpreter::new(x.args[1].borrow().clone(), self.fn_name.clone());
                            index = ex.evaluate(gl, assignment_token) as u32;
                            let ex =  Interpreter::new(x.args[2].borrow().clone(), self.fn_name.clone());
                            value = ex.evaluate(gl, assignment_token);
                            gl.push_vec_value(&vec_name, index as usize, value);
                            result = value;
                        } else {
                            panic!("You cannot pass no more than 3 values to push() 1: Name of the vec 2: index 3: the value you want to push")
                        }
                    },
                    "pop" => {
                        if x.args.len() == 2 {
                            let mut vec_name: String;
                            let mut index: u32;
                            let mut value: f32;
                            match x.args[0].borrow().clone() {
                                Expr::Variable(var) => vec_name=var.name.value,
                                _ => panic!("You can only enter vector names to pop()")
                            }
                            let ex =  Interpreter::new(x.args[1].borrow().clone(), self.fn_name.clone());
                            index = ex.evaluate(gl, assignment_token) as u32;
                            value = gl.pop_vec_value(&vec_name, index as usize);
                            result = value;
                        } else {
                            panic!("You cannot pass no more than 2 values to push() 1: Name of the vec 2: index")
                        }
                    },
                    "snap" => {
                        if x.args.len() == 2 {
                            let mut vec_name: String;
                            let mut value: f32;
                            match x.args[0].borrow().clone() {
                                Expr::Variable(var) => vec_name=var.name.value,
                                _ => panic!("You can only enter vector names to snap()")
                            }
                            let ex =  Interpreter::new(x.args[1].borrow().clone(), self.fn_name.clone());
                            value = ex.evaluate(gl, assignment_token);
                            gl.snap_vec_value(&vec_name, value.clone());
                            result = value;
                        } else {
                            panic!("You cannot pass no more than 2 values to snap() 1: Name of the vec 2: value")
                        }
                    },
                    "len" => {
                        if x.args.len() == 1 {
                            let mut vec_name: String;
                            match x.args[0].borrow().clone() {
                                Expr::Variable(var) => vec_name=var.name.value,
                                _ => panic!("You can only enter vector names to pop()")
                            }
                            result = gl.len_vec(&vec_name) as f32
                            
                        } else {
                            panic!("You cannot pass no more than 1 value to len() 1: Name of the vector")
                        }
                    },
                    
                    str => {
                        if gl.fn_exist(&str.to_string()) {
                               let mut agrsvec = Vec::new();
                               for arg in x.args {
                                   let ex = Interpreter::new(arg.borrow().clone(), self.fn_name.clone());
                                   agrsvec.push(ex.evaluate(gl, assignment_token));
                               }
                               self.fn_name = str.to_string();
                               gl.setup_local(str.to_string(),agrsvec);
                               let expr = gl.find_fn_expr(&str.to_string());
                               for exp in expr.iter() {
                                  let fn_inter = Interpreter::new(exp.borrow().clone(), self.fn_name.clone());
                                  result = fn_inter.evaluate(gl, assignment_token);
                               };
                               
                               
                        } else{
                            println!("Error -> Fncall no fn named {} is found", str);
                        }
                    }
                }
                self.fn_name = "_".to_string();
                return result;
            },
            Expr::Vector(x) => {
                let mut result_vec = Vec::new();
                for vars in x.value.iter() {
                    match vars.name.kind {
                        TokenType::Number(x) => result_vec.push(x),
                        TokenType::Id => {
                            result_vec.push(*gl.find_var_value(&vars.name.value, &self.fn_name));
                        },
                        _ => panic!("Error -> Vector_Assigment"),
                }
                };
                self.vec_assign = true;
                gl.add_vec(assignment_token.value.clone(), result_vec);
                return 0.0;
           },
           Expr::Veccall(x) => {
               
               match x.index.name.kind {
                 TokenType::Number(y) => *gl.find_vec_value(&x.name.value, f32_to_usize(y)),
                 TokenType::Id => {
                    let ind = gl.find_var_value(&x.index.name.value,&self.fn_name);
                    *gl.find_vec_value(&x.name.value, f32_to_usize(*ind))
                 },
                 _ => panic!("Error -> Interpreter -> Vec_call: Unrecognized value as index "),
               }
           },
           Expr::Fnassign(x) => {
               
                    let mut paramvec = Vec::new();
                    for par in x.params {
                        paramvec.push(par.name.value)
                    }
                    gl.add_fn(x.name.value, x.expr,paramvec );
                    return 0.0;
               
           },
           Expr::Forloop(x) => {
             let mut result: f32 = 0.0;
             let start = match x.start.name.kind {
                TokenType::Number(x) => x as i32,
                TokenType::Id => {
                    *gl.find_var_value(&x.start.name.value,&self.fn_name) as i32
                },
                _ => panic!("Error -> ForLoop Interpreter: Unknown token")
             };
             let end = match x.end.name.kind {
                TokenType::Number(x) => x as i32,
                TokenType::Id => {
                    *gl.find_var_value(&x.end.name.value,&self.fn_name) as i32
                },
                _ => panic!("Error -> ForLoop Interpreter: Unknown token")
             };
             if end - start < 1 {
                panic!("Error -> ForLoop: end: {} - start: {} should be at least 1! ", end , start);
             }
             for i in start..end {
                let mut tmp: f32 = 0.0;
                gl.add_var(x.index.name.value.clone(), i as f32);
                for exp in x.expr.clone().iter() {
                    let interprter = Interpreter::new(exp.borrow().clone(), self.fn_name.clone());
                    tmp = interprter.evaluate(gl, assignment_token);
                }
                result = tmp
             }
             gl.remove(&x.index.name.value);

             return result;
           }
           _ => panic!("not done yet")
    }
  } 
}