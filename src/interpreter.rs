use std::string;
use std::rc::Rc;
use std::cell::RefCell;


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