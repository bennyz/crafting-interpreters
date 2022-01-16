use std::collections::HashMap;

use crate::token::{Literal, Token, TokenType};
use anyhow::Result;
use lazy_static::lazy_static;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));

        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().unwrap();

        match c {
            ')' => self.add_token(TokenType::LeftParen),
            '(' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                // TODO macrofy
                let token = if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '=' => {
                // TODO macrofy
                let token = if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '<' => {
                // TODO macrofy
                let token = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                // TODO macrofy
                let token = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance().unwrap();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            char if char.is_ascii_digit() => {
                self.number();
            }
            char if char.is_ascii_alphabetic() => {
                // Exercises the Maximal Munch, if two grammar rules can match a chunk of
                // code, the one that can match the most wins
                self.identifier();
            }
            _ => panic!("unrecognized token!"), // TODO: we can't panic here, Lox::error() needs to be used
        }

        Ok(())
    }

    fn identifier(&mut self) {
        // Alphanumeric, as variables may start with a letter and continue with digits.
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let val = &self.source[self.start as usize..self.current as usize];
        match KEYWORDS.get(val) {
            Some(token_type) => {
                self.add_token(token_type.clone());
            }
            None => {
                self.add_token(TokenType::Identifier);
            }
        };
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // peek_next, because we only want to peek at most 2 characters ahead,
        // taking a param would have allowed for peeking ahead an arbitrary amount
        // of characters
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consumes the '.'

            // Consume the rest
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let val = self.source[self.start as usize..self.current as usize]
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))
    }

    fn peek_next(&self) -> char {
        if self.current as usize + 1 >= self.source.len() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Uterminated string"); // TODO can't panic here
        }

        // Consume the closing "
        self.advance();

        let val = self.source[self.start as usize..self.current as usize]
            .trim_matches('"')
            .to_owned();
        self.add_token_literal(TokenType::String, Some(Literal::String(val)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let mut it = self.source.chars().peekable();
        let c = it.nth(self.current as usize).unwrap();
        return c;
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let curr = self.source.chars().nth(self.current as usize).unwrap(); // Safe because of previous check
        if curr != expected {
            return false;
        }

        self.advance();

        return true;
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }
}
