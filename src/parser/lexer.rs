use bytes::BytesMut;
use crate::parser::{LiteralValue, Token, TokenType};

pub struct Lexer {
    source: BytesMut,
    pub tokens: Vec<Token>,
    line: i32,
    curr: usize,
    start: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            source: BytesMut::from(source.as_str()),
            line: 0,
            curr: 0,
            start: 1,
        }
    }


    pub fn tokenize(&mut self) {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, String::from(""), LiteralValue::Null, self.line));
    }

    fn is_at_end(&self) -> bool {
        if self.curr >= self.source.len() {
            true
        } else {
            false
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => { self.add_token(TokenType::LEFT_PAREN) }
            b')' => { self.add_token(TokenType::RIGHT_PAREN) }
            b'{' => { self.add_token(TokenType::LEFT_BRACE) }
            b'}' => { self.add_token(TokenType::RIGHT_BRACE) }
            b',' => { self.add_token(TokenType::COMMA) }
            b'.' => { self.add_token(TokenType::DOT) }
            b'-' => { self.add_token(TokenType::MINUS) }
            b'+' => { self.add_token(TokenType::PLUS) }
            b';' => { self.add_token(TokenType::SEMICOLON) }
            b'*' => { self.add_token(TokenType::STAR) }
            _ => todo!()
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.curr];
        let text: String = String::from_utf8(text.to_vec()).unwrap();
        self.tokens.push(Token::new(token_type, text, LiteralValue::Null, self.line))
    }

    fn advance(&mut self) -> u8 {
        let temp = self.source[self.curr];
        self.curr += 1;
        temp
    }
}