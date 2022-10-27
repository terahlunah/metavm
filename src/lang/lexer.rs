use crate::lang::{
    parser::{ParseError, ParseErrorKind},
    span::Span,
    token::{Token, TokenKind},
};
use std::{collections::VecDeque, str::FromStr};
use tap::TapOptional;

pub struct Lexer {
    source: Vec<char>,
    start_pos: usize,
    current_pos: usize,
    queue: VecDeque<Token>,
    indents: Vec<usize>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start_pos: 0,
            current_pos: 0,
            queue: VecDeque::new(),
            indents: vec![0],
        }
    }

    fn error(&self, error: ParseErrorKind) -> ParseError {
        ParseError::new(self.span().into(), error)
    }

    fn indent_level(&self) -> usize {
        self.indents.last().copied().unwrap_or(0)
    }

    fn has_ident_level(&self, level: usize) -> bool {
        self.indents.contains(&level)
    }

    fn push_indent_level(&mut self, level: usize) {
        self.indents.push(level);
    }

    fn pop_indent_level(&mut self, level: usize) -> usize {
        let mut count = 0;
        while self.indent_level() != level {
            self.indents.pop();
            count += 1;
        }
        count
    }

    fn span(&self) -> Span {
        Span::new(self.start_pos, self.current_pos - self.start_pos)
    }

    fn token(&mut self, mut token: Token) {
        token.span = self.span();
        println!("{:?}", token);
        self.queue.push_back(token);
        self.start_pos = self.current_pos;
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.current_pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current_pos + 1).copied()
    }

    fn advance(&mut self) {
        self.current_pos += 1
    }

    fn is_line_start(&self) -> bool {
        if self.current_pos > 1 {
            self.source
                .get(self.current_pos - 1)
                .map_or(false, |c| *c == '\n')
        } else {
            true
        }
    }

    fn trace(&self) {
        //return;
        let src: String = self
            .source
            .iter()
            .map(|c| match *c {
                '\n' => '↵',
                c => c,
            })
            .collect();

        print!("[");
        for level in &self.indents {
            print!("{},", level);
        }
        println!("]");
        println!("{}", src);
        println!("{}↑", " ".repeat(self.current_pos));
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.queue.pop_front() {
            return Ok(token);
        }

        self.read_token()?;

        Ok(self
            .queue
            .pop_front()
            .unwrap_or(Token::new(self.span(), TokenKind::Eof)))
    }

    fn read_token(&mut self) -> Result<(), ParseError> {
        self.trace();

        if self.is_line_start() {
            self.read_indent()?;
            if let Some(' ') = self.current() {
                return self.read_token();
            } else {
                self.trace();
            }
        }

        if let Some(c) = self.current() {
            match c {
                '0'..='9' => {
                    self.read_number()?;
                }
                '\'' => {
                    self.read_char()?;
                }
                '"' => {
                    self.read_string()?;
                }
                '{' => {
                    self.advance();
                    self.token(Token::of(TokenKind::LBrace));
                }
                '}' => {
                    self.advance();
                    self.token(Token::of(TokenKind::RBrace));
                }
                '\\' => {
                    self.advance();
                    self.token(Token::of(TokenKind::Backslash));
                }
                '#' => {
                    self.skip_comment()?;
                }
                c if is_term_lead(c) => {
                    self.read_term()?;
                }
                c => return Err(self.error(ParseErrorKind::UnexpectedChar(c))),
            }
            self.skip_whitespace();
        }

        Ok(())
    }

    // ---

    fn read(&mut self) -> Result<char, ParseError> {
        match self.current() {
            None => Err(self.error(ParseErrorKind::UnexpectedEof)),
            Some(c) => {
                self.advance();
                Ok(c)
            }
        }
    }

    fn try_read_fn(&mut self, f: impl Fn(char) -> bool) -> Option<char> {
        match self.current() {
            Some(c) if f(c) => {
                self.advance();
                Some(c)
            }
            _ => None,
        }
    }

    fn try_read_exact(&mut self, c: char) -> bool {
        match self.current() {
            Some(cur) if cur == c => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn read_exact(&mut self, c: char) -> Result<(), ParseError> {
        self.try_read_exact(c)
            .then_some(())
            .ok_or_else(|| self.error(ParseErrorKind::UnexpectedChar(c)))
    }

    fn try_read_digit(&mut self) -> Option<char> {
        match self.current() {
            Some(c @ '0'..='9') => {
                self.advance();
                Some(c)
            }
            _ => None,
        }
    }

    fn read_digit(&mut self) -> Result<char, ParseError> {
        self.try_read_digit()
            .ok_or_else(|| self.error(ParseErrorKind::ExpectedDigit))
    }

    fn read_integer(&mut self) -> Result<String, ParseError> {
        let mut num = String::new();
        num.push(self.read_digit()?);
        while let Some(c) = self.try_read_digit() {
            num.push(c);
        }
        Ok(num)
    }

    fn read_number(&mut self) -> Result<(), ParseError> {
        let num = self.read_integer()?;

        if self.try_read_exact('.') {
            let fract = self.read_integer()?;
            let num = format!("{}.{}", num, fract);
            self.token(Token::with_float(
                TokenKind::Float,
                f64::from_str(&num).map_err(|_| self.error(ParseErrorKind::ParseFloat(num)))?,
            ));
            Ok(())
        } else {
            self.token(Token::with_int(
                TokenKind::Int,
                i64::from_str(&num).map_err(|_| self.error(ParseErrorKind::ParseInt(num)))?,
            ));
            Ok(())
        }
    }

    fn read_char(&mut self) -> Result<(), ParseError> {
        self.read_exact('\'')?;
        let c = self.read()?;
        self.read_exact('\'')?;
        self.token(Token::with_char(TokenKind::Char, c));
        Ok(())
    }

    fn read_string(&mut self) -> Result<(), ParseError> {
        let mut s = String::new();

        self.read_exact('"')?;
        loop {
            let c = self.read()?;
            if c == '"' {
                break;
            } else {
                s.push(c);
            }
        }

        self.token(Token::with_string(TokenKind::String, s));

        Ok(())
    }

    fn read_term(&mut self) -> Result<(), ParseError> {
        let mut id = String::new();

        while let Some(c) = self.try_read_fn(is_term) {
            id.push(c);
        }

        match id.as_str() {
            "def" => self.token(Token::of(TokenKind::Def)),
            "=" => self.token(Token::of(TokenKind::Eq)),
            _ => self.token(Token::with_string(TokenKind::Term, id)),
        }

        Ok(())
    }

    fn read_indent(&mut self) -> Result<(), ParseError> {
        let mut count = 0;

        while self.try_read_exact(' ') {
            count += 1;
        }

        match self.current() {
            Some('\r') | Some('\n') => {
                self.skip_whitespace();
            }
            _ => {
                if count > self.indent_level() {
                    self.push_indent_level(count);
                    self.token(Token::of(TokenKind::Indent));
                } else if count < self.indent_level() {
                    if self.has_ident_level(count) {
                        let count = self.pop_indent_level(count);
                        for _ in 0..count {
                            self.token(Token::of(TokenKind::Dedent));
                        }
                    } else {
                        return Err(self.error(ParseErrorKind::IndentationError));
                    }
                }
            }
        }
        Ok(())
    }

    fn skip_comment(&mut self) -> Result<(), ParseError> {
        self.read_exact('#')?;
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current() {
                Some(' ') | Some('\r') => {
                    self.advance();
                }
                Some('\n') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }
        self.start_pos = self.current_pos;
    }
}

fn is_term_lead(c: char) -> bool {
    matches!(c,
        'a'..='z'
        | 'A'..='Z'
        | '='
        | '+'
        | '-'
        | '*'
        | '/'
        | '>'
        | '<'
        | '_'
        | '|'
        | '^'
        | '%'
        | '?'
        | '!'
        | ':'
        | '~')
}

fn is_term(c: char) -> bool {
    matches!(c,
        'a'..='z'
        | 'A'..='Z'
        | '0'..='9'
        | '='
        | '+'
        | '-'
        | '*'
        | '/'
        | '>'
        | '<'
        | '_'
        | '|'
        | '^'
        | '%'
        | '?'
        | '!'
        | ':'
        | '~')
}
