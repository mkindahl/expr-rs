//! Tokenizer to read character from an iterator over some text and
//! produce tokens for the shunting-yard algorithm.

use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Error,
    Float(f64),
    Symbol(String),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Open,
    Close,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Power => write!(f, "^"),
            Token::Open => write!(f, "("),
            Token::Close => write!(f, ")"),
            Token::Symbol(ref n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            chars: text.chars(),
        }
    }

    #[allow(dead_code)]
    fn take(&mut self, count: usize) -> &'a str {
        let start = self.chars.as_str();
        self.skip(count);
        &start[..start.len() - self.chars.as_str().len()]
    }

    fn skip(&mut self, count: usize) {
        for _ in 0..count {
            self.chars.next();
        }
    }

    fn take_while<P>(&mut self, pred: P) -> &'a str
    where
        P: Copy + FnMut(char) -> bool,
    {
        let start = self.chars.as_str();
        self.skip_while(pred);
        &start[..start.len() - self.chars.as_str().len()]
    }

    fn skip_while<P>(&mut self, pred: P)
    where
        P: Copy + FnMut(char) -> bool,
    {
        while self.chars.clone().next().map_or(false, pred) {
            self.chars.next();
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_while(|ch| ch.is_whitespace());
        match self.chars.clone().next() {
            Some(ch) if ch.is_digit(10) => self
                .take_while(|ch| ch.is_digit(10) || ch == '.')
                .parse::<f64>()
                .map(|num| Token::Float(num))
                .ok(),
            Some(ch) if ch.is_alphabetic() => {
                let name = self.take_while(|c| c.is_alphabetic() || c == '_' || c.is_digit(10));
                Some(Token::Symbol(name.to_string()))
            }
            Some('+') => {
                self.skip(1);
                Some(Token::Plus)
            }
            Some('-') => {
                self.skip(1);
                Some(Token::Minus)
            }
            Some('*') => {
                self.skip(1);
                Some(Token::Star)
            }
            Some('/') => {
                self.skip(1);
                Some(Token::Slash)
            }
            Some('^') => {
                self.skip(1);
                Some(Token::Power)
            }
            Some('(') => {
                self.skip(1);
                Some(Token::Open)
            }
            Some(')') => {
                self.skip(1);
                Some(Token::Close)
            }
            None | Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token::{Float, Minus, Plus, Slash, Star, Symbol};
    use super::*;
    //    use test::{black_box, Bencher};

    fn assert_tokens(text: &str, expected: Vec<Token>) {
        let tokenizer = Tokenizer::new(text);
        let tokens: Vec<Token> = tokenizer.collect();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn basic() {
        assert_tokens("", vec![]);
        assert_tokens("   ", vec![]);
        assert_tokens("\t  \n1", vec![Float(1.0)]);
        assert_tokens("\t  \n-1", vec![Minus, Float(1.0)]);
    }

    #[test]
    fn operators() {
        let x = "x".to_string();
        let y = "y".to_string();
        assert_tokens("10+", vec![Float(10.0), Plus]);
        assert_tokens("-12", vec![Minus, Float(12.0)]);
        assert_tokens("+*-/", vec![Plus, Star, Minus, Slash]);
        assert_tokens("12.0+y", vec![Float(12.0), Plus, Symbol(y.clone())]);
        assert_tokens("12/y", vec![Float(12.0), Slash, Symbol(y.clone())]);
        assert_tokens("x+12.0", vec![Symbol(x.clone()), Plus, Float(12.0)]);
    }

    #[test]
    fn variables() {
        let x = "x".to_string();
        assert_tokens("x12", vec![Symbol("x12".to_string())]);
        assert_tokens("12x", vec![Float(12.0), Symbol(x)]);
        assert_tokens("12.0zzz", vec![Float(12.0), Symbol("zzz".to_string())]);
        assert_tokens("12.a_3", vec![Float(12.0), Symbol("a_3".to_string())]);
        assert_tokens("a_12", vec![Symbol("a_12".to_string())]);
    }

    #[test]
    fn numbers() {
        assert_tokens("12", vec![Float(12.0)]);
        assert_tokens("13.", vec![Float(13.0)]);
        assert_tokens("13.1", vec![Float(13.1)]);
        assert_tokens("0.2", vec![Float(0.2)]);
        assert_tokens("0.2", vec![Float(0.2)]);
        assert_tokens("012.2", vec![Float(12.2)]);
        assert_tokens("00000012.2", vec![Float(12.2)]);
        assert_tokens("000012.2", vec![Float(12.2)]);
    }

    // #[bench]
    // fn benchmark(b: &mut Bencher) {
    //     let input = r#"12 56 df + asfd ++1jksdf+-**//485.0956 9812.983"#;
    //     b.iter(|| {
    //         let tokenizer = Tokenizer::new(input);
    //         black_box(tokenizer.collect::<Vec<_>>());
    //     });
    // }
}
