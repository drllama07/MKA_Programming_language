use crate::tokens::{Token, TokenType};


pub fn is_num(c: char) -> bool {
    let nums = "1234567890";
    nums.contains(c)
}

pub fn is_letter(c: char) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters.contains(c)
}

pub fn is_two_op(st: String) -> TokenType {
    match st.as_str() {
        
        "!=" => TokenType::BangEqual,
        "==" => TokenType::EqualEqual,
        "-=" => TokenType::MinusEqual,
        "+="=> TokenType::PlusEqual,
        _ => todo!()
    }
}

pub fn is_identifier(input: &mut String) -> Token {
    let keywords = vec!["return", "to", "in", "for", "while", "var"];
    let types = vec!["f64"];

    let mut name = String::new();
    let temp = input.clone();
    for c in temp.chars(){
        if is_num(c) || is_letter(c) {
            name.push(c);
            input.remove(0);
        }
        else {
            break
        }
    }

    if keywords.contains(&&*name) {
        match name.as_str() {
            "return" => Token{kind: TokenType::Return, value: name},
            "to" => Token{kind: TokenType::To, value: name},
            "in" => Token{kind: TokenType::In, value: name},
            "for" => Token{kind: TokenType::For, value: name},
            "while" => Token{kind: TokenType::While, value: name},
            "var" => Token{kind: TokenType::Var, value: name},
            &_ => todo!()
        }   
    }
    else if types.contains(&&*name) {
        Token {kind: TokenType::Type(name), value: String::from("f64")}
    }
    else {
        Token {kind: TokenType::Id, value: String::from(name)}
    }

}

pub fn read_number (input : &mut String) -> Token {
    let mut name = String::new();
    let temp = input.clone();
    for c in temp.chars() {
        if !c.is_whitespace() && is_num(c) {
            name.push(c);
            input.remove(0);
        }
        else if !c.is_whitespace() && c == '.'{
            name.push(c);
            input.remove(0);
        } else {
            break;
        }
    }

    Token {kind: TokenType::Number(name.parse::<f32>().unwrap()), value : name}
}





pub fn matcher(c: char) -> TokenType {
   
   match c {
       '(' => TokenType::LeftParen,
       ')' => TokenType::RightParen,
       '{' => TokenType::LeftBrace,
       '}' => TokenType::RightBrace,
       '!' => TokenType::Bang,
       '+' => TokenType::Plus,
       '-' => TokenType::Minus,
       '=' => TokenType::Equal,
       '*' => TokenType::Star,
       '/' => TokenType::Slash,
       ',' => TokenType::Comma,
       ';' => TokenType::Semicolon,
       '.' => TokenType::Dot,
       '<' => TokenType::Less,
       '>' => TokenType::Greater,
       '^' => TokenType::Exp,
       _ => todo!()
   }
}

pub fn lexer(input: &mut String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut c: char;
    let double_chars = "+-=!";
    while input.len() > 0 {
        c = input.chars().next().unwrap();
        if !c.is_whitespace() {
            
            if input.len() > 1 {
              let mut tempo = input.clone();
              tempo.remove(0);
              let second_char: char = tempo.chars().next().unwrap();
              if  second_char == '=' && double_chars.contains(c){
                let second_val = c.to_string() + second_char.to_string().as_str();
                tokens.push(Token {kind: is_two_op(second_val), value: "operation".to_string()});
                input.remove(0);
                input.remove(0);
                continue;
              } 

              if is_letter(c) {
                 tokens.push(is_identifier(input));
               }
              else if is_num(c) {
                 tokens.push(read_number(input));
               }
              else {
                let tok = matcher(c);
                tokens.push(Token { kind: tok, value: "operation or punctuation".to_string() });
                
                input.remove(0);
               }
            } else {
                if is_letter(c) {
                    tokens.push(is_identifier(input));
                  }
                 else if is_num(c) {
                    tokens.push(read_number(input));
                  }
                 else {
                   let tok = matcher(c);
                   tokens.push(Token { kind: tok, value: "operation or punctuation".to_string() });
                   
                   input.remove(0);
                  }
            }
           }
           else {
            if c == '\n' {
                tokens.push(Token { kind: TokenType::Eof, value: "end".to_string() });
                input.remove(0);
                
            } else if c == '\t' || c == ' '{
                input.remove(0);
            } else {
                print!("hell noooo");
            }
            
           }
        } 

    tokens
}
    