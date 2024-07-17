

use std::rc::Rc;
use std::cell::RefCell;
use crate::tokens::{self, Token, TokenType};

// use crate::expressions::*;


#[derive(Debug,Clone)]
pub struct AssignmentImpl {
    pub target: VariableImpl,
    pub value: Box<Expr>,
}
#[derive(Debug,Clone)]
pub struct DoubleImpl {
    pub target: VariableImpl,
    pub op: TokenType,
    pub value: Box<Expr>,
}
#[derive(Debug,Clone)]
pub struct BinaryOpImpl {
   pub  left: Box<Expr>,
   pub  operation: Token,
    pub right: Box<Expr>
}
#[derive(Debug,Clone)]
pub struct UnaryImpl {
    pub  operation: Token,
    pub right: Box<Expr>
}
#[derive(Debug,Clone)]
pub struct NumberImpl {
    pub value: f32,
    token: Token
}
#[derive(Debug,Clone)]
pub struct VariableImpl {
    pub name: Token
}
#[derive(Debug,Clone)]
pub struct FnImpl {
    pub name: Token,
    pub params: Vec<VariableImpl>,
    pub expr: Vec<Rc<RefCell<Expr>>>
}

#[derive(Debug,Clone)]
pub struct FncallImpl {
    pub name: Token,
    pub args: Vec<Rc<RefCell<Expr>>>,
}
#[derive(Debug,Clone)]
pub struct ForImpl {
    pub index: VariableImpl,
    pub start: VariableImpl,
    pub end: VariableImpl,
    pub expr: Vec<Rc<RefCell<Expr>>>
}

#[derive(Debug,Clone)]
pub struct VecImpl {
    pub value: Vec<VariableImpl>,
}

#[derive(Debug,Clone)]
pub struct VecCall {
    pub name: Token,
    pub index: VariableImpl
}
#[derive(Debug,Clone)]
pub enum Expr {
    Variable(VariableImpl),
    Assignment(AssignmentImpl),
    Fnassign(FnImpl),
    Number(NumberImpl),
    Vector(VecImpl),
    BinaryOperation(BinaryOpImpl),
    Unary(UnaryImpl),
    Fncall(FncallImpl),
    Veccall(VecCall),
    Forloop(ForImpl),
    Double(DoubleImpl),
    Useless
    
}

fn match_token(expected: TokenType, tokens: &mut Vec<Token>) {
    match tokens.pop() {
        None => {},
        Some(token) => {
            if expected != token.kind {
                panic!("match_token error {:?} {} ", token.kind, expected);
            }
        },
    }
}


fn find_token(expected1: TokenType,expected2: TokenType, tokens: & Vec<Token>) -> bool {
    let mut a = false;
    
    for i in 1..(tokens.len() -1) {
        if tokens[tokens.len() - i].kind == TokenType::Eof {
            break;
        }
        if tokens[tokens.len() - i].kind == expected1 && tokens[tokens.len() - i + 1].kind == expected2{
            a = true;
        }
        
    }
    return a;
}
fn separate_tokens(typetok: TokenType, tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();
    while &tokens[&tokens.len() -1 ].kind != &typetok {
        if &tokens[&tokens.len() -1 ].kind == &TokenType::RightParen {
            result.reverse();
            return result;
        } else {
            
            result.push(tokens.pop().unwrap());
        }  
    }
    match_token(typetok, tokens);
    result.reverse();
    result
}


pub fn parse(mut tokens: Vec<Token>) -> Vec<Expr> {
    tokens.reverse();
    let mut result = Vec::new();

    while tokens.len() > 0 {
        let expr = parse_expr(&mut tokens);
        
        
        result.push(expr) ;
    }
    
    return result;
}

fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    return parse_assigment(tokens);

}

fn parse_assigment(tokens: &mut Vec<Token>) -> Expr {

    if  tokens.len() > 1 && &tokens[tokens.len() -  2 ].kind == &TokenType::Equal {

        let variable = parse_variable(tokens);
        match_token(TokenType::Equal, tokens);

        let value = parse_expr(tokens);

        return Expr::Assignment(AssignmentImpl{ target: variable,value:  Box::new(value)});
    } else if tokens.len() > 1 && &tokens[tokens.len() -  2 ].kind == &TokenType::PlusEqual {
        let variable = parse_variable(tokens);
        match_token(TokenType::PlusEqual, tokens);

        let value = parse_expr(tokens);

        return Expr::Double(DoubleImpl{ target: variable,op: TokenType::PlusEqual,value:  Box::new(value)});
    } 
    else if tokens.len() > 1 && &tokens[tokens.len() -  2 ].kind == &TokenType::MinusEqual {
        let variable = parse_variable(tokens);
        match_token(TokenType::MinusEqual, tokens);

        let value = parse_expr(tokens);

        return Expr::Double(DoubleImpl{ target: variable,op: TokenType::MinusEqual,value:  Box::new(value)});
    }
    else if tokens.len() > 4 && find_token(TokenType::Equal,TokenType::RightParen, &tokens) {

        let fnnew = parse_fn(tokens);

        return Expr::Fnassign(fnnew);
    }
    else if tokens.len() > 6 &&  &tokens[tokens.len() -  1 ].kind == &TokenType::For {
        let for_loop = parse_for_loop(tokens);

        return Expr::Forloop(for_loop);
    }
    else {

        return parse_term(tokens);
    }
} 


fn parse_variable(tokens: &mut Vec<Token>) -> VariableImpl {
    let token = tokens.pop().unwrap();

    if token.kind == TokenType::Id {
           return VariableImpl { name: token};
    } else {
        panic!("UFO -> from the AST");
    }
}

fn parse_fn(tokens: &mut Vec<Token>) -> FnImpl {
    let mut name: Token = Token { kind: TokenType::Eof, value: "".to_string() };
    let mut params: Vec<VariableImpl> = Vec::new();
    let mut vecexpr: Vec<Rc<RefCell<Expr>>> = Vec::new();
    let token = tokens.pop().unwrap();
    if token.kind == TokenType::Id {
            name = token;    
         }
    else {
        panic!("You cannot define a function with {}", token.kind)
    }
    while &tokens[tokens.len() -1].kind != &TokenType::RightParen {
         let token = tokens.pop().unwrap();
         match token.kind {
            TokenType::LeftParen => {},
            TokenType::Id => params.push(VariableImpl {name: token}),
            TokenType::Comma => {
                if &tokens[tokens.len() -1 ].kind != &TokenType::Id {
                    panic!("expecting a variable for parameters after ,");
                }
            },
            _ => panic!("You cannot use {}", token.kind)
         }
    }

    match_token(TokenType::RightParen, tokens);
    match_token(TokenType::Equal, tokens);
    if &tokens[&tokens.len() - 1].kind == &TokenType::LeftBrace{

        match_token(TokenType::LeftBrace, tokens);
        while &tokens[&tokens.len() -1 ].kind != &TokenType::RightBrace {
            match &tokens[&tokens.len()-1].kind {
                &TokenType::Eof => match_token(TokenType::Eof, tokens),
                _ => vecexpr.push(Rc::new(RefCell::new(parse_expr(tokens))))
              }
        }
        match_token(TokenType::RightBrace, tokens);
    }
    else {
        vecexpr.push(Rc::new(RefCell::new(parse_expr(tokens))));
    }
    
    return FnImpl {name: name, params: params,expr: vecexpr};
}
fn parse_for_loop(tokens: &mut Vec<Token>) -> ForImpl {
    let _ = tokens.pop().unwrap();
    let index = parse_variable(tokens);
    let mut start:VariableImpl;
    let mut end:VariableImpl;
    let mut expr:Vec<Rc<RefCell<Expr>>> = Vec::new();
    match_token(TokenType::In, tokens);
    let token = tokens.pop().unwrap();
    match token.kind {
        TokenType::Number(x) => start = VariableImpl {name: token},
        TokenType::Id => start = VariableImpl {name: token},
        _ => panic!("You cannot use that in a loop")
    }
    match_token(TokenType::To, tokens);
    let token = tokens.pop().unwrap();
    match  token.kind {
        TokenType::Number(x) => end = VariableImpl {name: token},
        TokenType::Id => end = VariableImpl {name: token},
        _ => panic!("You cannot use that in a loop")
    }

    let token = tokens.pop().unwrap();
    if token.kind == TokenType::LeftBrace {
         while &tokens[&tokens.len()-1].kind != &TokenType::RightBrace {
              match &tokens[&tokens.len()-1].kind {
                &TokenType::Eof => match_token(TokenType::Eof, tokens),
                _ => expr.push(Rc::new(RefCell::new(parse_expr(tokens))))
              }
         }
         match_token(TokenType::RightBrace, tokens);
         
    } else {
        panic!("You must use braces for the for loop!! {}", token.kind)
    }
    return ForImpl { index: index, start: start, end: end, expr: expr };


}
fn parse_term(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_factor(tokens);

    while tokens.len() > 0 {
       let next = &tokens[tokens.len() -1 ];
       match next.kind {
        TokenType::Plus | TokenType::Minus => {
            let op = tokens.pop().unwrap();
            let right = parse_factor(tokens);

            result = Expr::BinaryOperation(BinaryOpImpl {left: Box::new(result), operation: op, right: Box::new(right)});

        },
        _ => break,
       }
    }
    return result;
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut result = parse_unary(tokens);

    while tokens.len() > 1 {
        let next = &tokens[tokens.len() - 1 ];
        match next.kind {
          TokenType::Star | TokenType::Slash | TokenType::Exp => {
             let op = tokens.pop().unwrap();
             let right = parse_unary(tokens);

             result = Expr::BinaryOperation( BinaryOpImpl {
                left: Box::new(result), 
                operation: op,
                right: Box::new(right)
             })
          },
          _ => break,
        }
    }
    return result;
}
pub fn parse_unary(tokens: &mut Vec<Token>)  -> Expr {
   match &tokens[&tokens.len()-1].kind {
      &TokenType::Minus => {
        let op = tokens.pop().unwrap();
        let right = parse_primary(tokens);

        Expr::Unary(UnaryImpl { operation: op, right: Box::new(right)})
      },
      _ => parse_primary(tokens)
   }
}
fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    
    let  token = tokens.pop().unwrap();
    if token.kind != TokenType::Eof {
    match token.kind {
        TokenType::Number(x) => {
            return Expr::Number(NumberImpl {value: x,token: token});
        },
        TokenType::Id => {
            if tokens.len() > 0 {
                let next = &tokens[tokens.len() - 1];
                if next.kind == TokenType::LeftParen {
                    let fn_name = token;

                    tokens.pop().unwrap();

                    let mut args: Vec<Rc<RefCell<Expr>>> = Vec::new();
                    while &tokens[tokens.len() -1].kind != &TokenType::RightParen {
                        let mut one_token = separate_tokens(TokenType::Comma, tokens);
                        let one_arg  = parse_expr(&mut one_token);
                        args.push(Rc::new(RefCell::new(one_arg)));
                   }
                    match_token(TokenType::RightParen, tokens);

                    return Expr::Fncall(
                        FncallImpl {
                            name: fn_name,
                            args: args

                        });
                } else if next.kind == TokenType::Less {
                    let vec_name = token;
                    let index: VariableImpl;
                    tokens.pop().unwrap();
                    let tok = tokens.pop().unwrap();
                    match tok.kind {
                        TokenType::Id => index = VariableImpl { name: tok},
                        TokenType::Number(x) => index = VariableImpl {name: tok},
                        _ => panic!("You cannot use that to index a vector element")
                    }
                    
                    match_token(TokenType::Greater, tokens);
                    
                    return Expr::Veccall(VecCall { name: vec_name, index:  index})

                }
            }
            return Expr::Variable(VariableImpl {name: token});
        },
        TokenType::LeftParen => {
            let expr = parse_expr(tokens);
            match_token(TokenType::RightParen, tokens);

            return expr;

        },
        TokenType::Less => {
            let mut expr: Vec<VariableImpl> = Vec::new();
            while &tokens[&tokens.len() -1].kind != &TokenType::Greater {
                let tok = tokens.pop().unwrap();
                match tok.kind {
                TokenType::Id => expr.push(VariableImpl {name: tok}),
                TokenType::Number(x) => expr.push(VariableImpl {name: tok}),
                TokenType::Comma => {
                       match &tokens[tokens.len() -1 ].kind {
                        &TokenType::Number(x) => {},
                        &TokenType::Id => {},
                        _ => panic!("You need a element after , "),
                       } 
                   },
                _ => panic!("error no use like this for vectors{:?}", tok)
                }
            }
            match_token(TokenType::Greater, tokens);
            return Expr::Vector(VecImpl { value: expr});
        }
        _ => {
                panic!("unknown token or syntax ? {}", token.kind);
        },
    }} else {
        return Expr::Useless;
    }
}

