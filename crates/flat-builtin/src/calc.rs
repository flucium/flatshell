/*

    1: Correct Error::DUMMY to the appropriate Error and ErrorKind.

    2: Reimplement using flat-parser. Currently, it is self-contained in the code for this context.

*/

use flat_common::error::Error;
use flat_common::result::Result;

fn pre_process(input: &str) -> Result<Vec<String>> {
    let mut tokens: Vec<String> = Vec::with_capacity(input.len());

    let mut buffer = String::with_capacity(input.len() + 1024);

    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        if c.is_digit(10) {
            buffer.push(c);
            continue;
        }

        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            if !buffer.is_empty() {
                tokens.push(buffer);
                buffer = String::new();
            }
            tokens.push(c.to_string());
            continue;
        }

        Err(Error::DUMMY)?;
    }

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    Ok(tokens)
}

fn shunting_yard(tokens: Vec<String>) -> Result<Vec<String>> {
    let mut output: Vec<String> = Vec::with_capacity(tokens.len());
    let mut stack: Vec<String> = Vec::with_capacity(tokens.len());

    for token in tokens {
        if token.chars().all(char::is_numeric) {
            output.push(token);
            continue;
        }

        if token == "(" {
            stack.push(token);
            continue;
        }

        if token == ")" {
            while let Some(top) = stack.pop() {
                if top == "(" {
                    break;
                }
                output.push(top);
            }
            continue;
        }

        if token == "+" || token == "-" {
            while let Some(top) = stack.last() {
                if top == "(" {
                    break;
                }
                output.push(stack.pop().unwrap());
            }
            stack.push(token);
            continue;
        }

        if token == "*" || token == "/" {
            while let Some(top) = stack.last() {
                if top == "*" || top == "/" {
                    output.push(stack.pop().unwrap());
                } else {
                    break;
                }
            }
            stack.push(token);
            continue;
        }

        Err(Error::DUMMY)?;
    }

    while let Some(top) = stack.pop() {
        output.push(top);
    }

    Ok(output)
}

fn calculate(tokens: Vec<String>) -> Result<i32> {
    let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());

    for token in tokens {
        if token.chars().all(char::is_numeric) {
            stack.push(token.parse().unwrap());
            continue;
        }

        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();

        match token.as_str() {
            "+" => stack.push(left + right),
            "-" => stack.push(left - right),
            "*" => stack.push(left * right),
            "/" => stack.push(left / right),
            _ => Err(Error::DUMMY)?,
        }
    }

    Ok(stack.pop().unwrap())
}

pub fn eval(input: &str) -> Result<i32> {
    let tokens = pre_process(input)?;
    let tokens = shunting_yard(tokens)?;
    calculate(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_process() {
        let input = "1 + 2 * 3";

        let result = pre_process(input).unwrap();

        assert_eq!(result, vec!["1", "+", "2", "*", "3"]);
    }

    #[test]
    fn test_shunting_yard() {
        let input = vec!["1", "+", "2", "*", "3"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = shunting_yard(input).unwrap();

        assert_eq!(result, vec!["1", "2", "3", "*", "+"]);
    }

    #[test]
    fn test_calculate() {
        let input = vec!["1", "2", "3", "*", "+"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = calculate(input).unwrap();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_eval() {
        let input = "1 + 2 * 3";

        let result = eval(input).unwrap();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_eval_paren() {
        let input = "(1 + 2) * 3";

        let result = eval(input).unwrap();

        assert_eq!(result, 9);
    }
}

/*

use super::token::*;
use flat_ast::Expr;
use flat_common::{error::*, result::*};

fn calc_shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut buffer: Vec<Token> = Vec::with_capacity(tokens.len());

    let mut s_vec: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens {
        match token {
            Token::USize(_) | Token::ISize(_) => buffer.push(token),

            Token::Plus | Token::Minus => {
                while let Some(op) = s_vec.last() {
                    match op {
                        Token::Star | Token::Slash | Token::Plus | Token::Minus => {
                            if let Some(op) = s_vec.pop() {
                                buffer.push(op);
                            } else {
                                Err(Error::DUMMY)?;
                            }
                        }
                        _ => break,
                    }
                }

                s_vec.push(token);
            }

            Token::Star | Token::Slash => {
                while let Some(op) = s_vec.last() {
                    match op {
                        Token::Star | Token::Slash => {
                            if let Some(op) = s_vec.pop() {
                                buffer.push(op);
                            } else {
                                Err(Error::DUMMY)?;
                            }
                        }
                        _ => break,
                    }
                }
                s_vec.push(token);
            }

            Token::LeftParen => s_vec.push(token),

            Token::RightParen => {
                while let Some(op) = s_vec.pop() {
                    if let Token::LeftParen = op {
                        break;
                    }
                    buffer.push(op);
                }
            }

            _ => {
                Err(Error::DUMMY)?;
            }
        }
    }

    while let Some(op) = s_vec.pop() {
        buffer.push(op);
    }

    Ok(buffer)
}

fn calc_rpn_eval(tokens: Vec<Token>) -> Result<isize> {
    let mut s_vec: Vec<isize> = Vec::with_capacity(tokens.len());

    for token in tokens {
        if matches!(token, Token::USize(_) | Token::ISize(_)) {
            s_vec.push(match token {
                Token::USize(n) => n.try_into().unwrap(),
                Token::ISize(n) => n,
                _ => Err(Error::DUMMY)?,
            });
            continue;
        }

        let b = match s_vec.pop() {
            Some(b) => b,
            None => Err(Error::DUMMY)?,
        };

        let a = match s_vec.pop() {
            Some(a) => a,
            None => Err(Error::DUMMY)?,
        };

        match token {
            Token::Plus => {
                s_vec.push(a + b);
            }
            Token::Minus => {
                s_vec.push(a - b);
            }
            Token::Star => {
                s_vec.push(a * b);
            }
            Token::Slash => {
                s_vec.push(a / b);
            }
            _ => {
                Err(Error::DUMMY)?;
            }
        }
    }

    if let Some(n) = s_vec.pop() {
        Ok(n)
    } else {
        Err(Error::DUMMY)?
    }
}

fn calc_eval(tokens: Vec<Token>) -> Result<Expr> {
    let rpn = calc_shunting_yard(tokens)?;
    let result = calc_rpn_eval(rpn)?;

    if result.is_positive() {
        Ok(Expr::USize(result as usize))
    } else {
        Ok(Expr::ISize(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shunting_yard() {
        let tokens = vec![
            Token::LeftParen,
            Token::USize(1),
            Token::Plus,
            Token::USize(2),
            Token::RightParen,
            Token::Star,
            Token::USize(3),
        ];

        let result = calc_shunting_yard(tokens).unwrap();

        assert_eq!(
            result,
            vec![
                Token::USize(1),
                Token::USize(2),
                Token::Plus,
                Token::USize(3),
                Token::Star
            ]
        );
    }

    #[test]
    fn test_rpn_eval() {
        let tokens = vec![
            Token::USize(1),
            Token::USize(2),
            Token::Plus,
            Token::USize(3),
            Token::Star,
        ];

        let result = calc_rpn_eval(tokens).unwrap();

        assert_eq!(result, 9);
    }

    #[test]
    fn test_eval() {
        let tokens = vec![
            Token::LeftParen,
            Token::USize(1),
            Token::Plus,
            Token::USize(2),
            Token::RightParen,
            Token::Star,
            Token::USize(3),
        ];

        let result = calc_eval(tokens).unwrap();

        assert_eq!(result, Expr::USize(9));
    }
}


*/
