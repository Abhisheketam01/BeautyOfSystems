use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus, Minus, Multiply, Divide,
    LParen, RParen, EOF,
}

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { chars: input.chars().peekable() }
    }

    fn next_token(&mut self) -> Token {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() { break; }
            self.chars.next();
        }

        if let Some(c) = self.chars.next() {
            match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some(&nc) = self.chars.peek() {
                        if nc.is_ascii_digit() || nc == '.' {
                            num_str.push(self.chars.next().unwrap());
                        } else { break; }
                    }
                    Token::Number(num_str.parse().unwrap_or(0.0))
                }
                _ => Token::EOF,
            }
        } else {
            Token::EOF
        }
    }
}

fn precedence(op: &Token) -> i32 {
    match op {
        Token::Plus | Token::Minus => 1,
        Token::Multiply | Token::Divide => 2,
        _ => 0,
    }
}

fn apply_op(ops: &mut Vec<Token>, values: &mut Vec<f64>) {
    if values.len() < 2 { return; } 
    let op = ops.pop().unwrap();
    let right = values.pop().unwrap();
    let left = values.pop().unwrap();
    let res = match op {
        Token::Plus => left + right,
        Token::Minus => left - right,
        Token::Multiply => left * right,
        Token::Divide => left / right,
        _ => 0.0,
    };
    values.push(res);
}

pub fn evaluate(input: &str) -> f64 {
    let mut lexer = Lexer::new(input);
    let mut values = Vec::new();
    let mut ops = Vec::new();
    
    // Principal Insight: We need to know if we are at the start or after an operator
    // to determine if '-' is unary or binary.
    let mut expects_operand = true;

    loop {
        let token = lexer.next_token();
        if token == Token::EOF { break; }

        match token {
            Token::Number(n) => {
                values.push(n);
                expects_operand = false;
            }
            Token::LParen => {
                ops.push(token);
                expects_operand = true;
            }
            Token::RParen => {
                while let Some(top) = ops.last() {
                    if *top == Token::LParen { 
                        ops.pop(); 
                        break; 
                    }
                    apply_op(&mut ops, &mut values);
                }
                expects_operand = false;
            }
            Token::Minus if expects_operand => {
                // UNARY MINUS DETECTED
                // We push a 0 to the values stack so "-5" becomes "0 - 5"
                values.push(0.0);
                ops.push(Token::Minus);
                expects_operand = false; 
            }
            _ => {
                // Regular binary operators (+, -, *, /)
                while let Some(top) = ops.last() {
                    if precedence(top) >= precedence(&token) {
                        apply_op(&mut ops, &mut values);
                    } else { break; }
                }
                ops.push(token);
                expects_operand = true;
            }
        }
    }

    while !ops.is_empty() {
        apply_op(&mut ops, &mut values);
    }

    values.pop().unwrap_or(0.0)
}

fn main() {
    println!("--- Rust Unix-Style Calculator (v2.0: Unary Support) ---");
    println!("Type an expression (e.g., -5 + 3 * -2) or 'exit' to quit.");
    
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        
        if io::stdin().read_line(&mut input).is_err() || input.trim() == "exit" {
            break;
        }

        let trimmed = input.trim();
        if !trimmed.is_empty() {
            let result = evaluate(trimmed);
            println!("Result: {}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus() {
        assert_eq!(evaluate("-5 + 3"), -2.0);
        assert_eq!(evaluate("5 + -3"), 2.0);
        assert_eq!(evaluate("-(5 + 5)"), -10.0);
    }

    #[test]
    fn test_complex_precedence() {
        assert_eq!(evaluate("2 + 3 * (8 / 4)"), 8.0);
        assert_eq!(evaluate("-2 * 3 + 4"), -2.0);
    }
}