use std::convert::TryFrom;

#[derive(Debug)]
pub enum Token {
    Comment,
    Return,
    Accessor,
    Unknown,
    DefaultArg,
    Equals,
    Colon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBoxBrace,
    RBoxBrace,
    ArrayExclusive,
    ArrayInclusive,

    In,
    For,
    Is,
    IsNot,

    If,
    Else,
    ElIf,
    IfNot,

    TString,
    StringScope,
    FormatStringScope,
    TTickString,
    TickStringScope,
    EnvVar,
    TInt,
    TFloat,
    TAny,

    KeySelf,
    Var,

    TBool,
    False,
    True,

    Numeric, // float, int
    Def,
}

impl Default for Token {
    fn default() -> Self {
        Token::Comment
    }
}

impl Token {
    pub fn from_word(word: String) -> Token {
        match word.as_str() {
            "#" => Token::Comment,
            "::" => Token::Return,
            "." => Token::Accessor,
            "_" => Token::DefaultArg,
            ":" => Token::Colon,
            "," => Token::Comma,
            "=" => Token::Equals,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "[" => Token::LBoxBrace,
            "]" => Token::RBoxBrace,
            ".." => Token::ArrayExclusive,
            "..." => Token::ArrayInclusive,
            "String" => Token::TString,
            "\"" => Token::StringScope,
            "f\"" => Token::FormatStringScope,
            "TickString" => Token::TTickString,
            "`" => Token::TickStringScope,
            "TBool" => Token::TBool,
            "true" => Token::True,
            "false" => Token::False,
            "Any" => Token::TAny,
            "in" => Token::In,
            "for" => Token::For,
            "is" => Token::Is,
            "is!" => Token::IsNot,
            "$" => Token::EnvVar,
            "if" => Token::If,
            "if!" => Token::IfNot,
            "else" => Token::Else,
            "elif" => Token::ElIf,
            "Int" => Token::TInt,
            "Float" => Token::TFloat,
            "self" => Token::KeySelf,
            "var" => Token::Var,
            _ => Token::Unknown,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Default)]
pub struct TokenDef {
    start: u64,
    end: u64,
    token: Token,
    meta: String,
}

impl PartialEq for TokenDef {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl TokenDef {
    pub fn token(&self) -> &Token {
        &self.token
    }
}

// pub struct Statement {
//     lhs: String,
//     rhs: String,
//     statement_arg: Token,
// }

pub struct Lexer {
    pos: u64,
    contents: String,
}

impl Lexer {
    pub fn from(file: std::path::PathBuf) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(file.as_path())?;
        Ok(Self { pos: 0, contents })
    }

    pub fn from_contents(contents: String) -> Self {
        Self { pos: 0, contents }
    }

    fn get_pos_usize(&self) -> usize {
        usize::try_from(self.pos).unwrap()
    }

    fn consume(&mut self) -> Result<char, bool> {
        if self.pos != 0 {
            self.pos += 1;
        }

        let curr_pos = self.get_pos_usize();
        if curr_pos >= self.contents.len() - 1 {
            return Err(false);
        }

        let c = self.contents.chars().nth(curr_pos).unwrap();
        Ok(c)
    }

    fn peek(&self) -> Result<char, ()> {
        let next_pos = self.get_pos_usize() + 1;
        if next_pos >= self.contents.len() {
            return Err(());
        }

        let c = self.contents.chars().nth(next_pos).unwrap();
        Ok(c)
    }

    fn consume_until_whitespace(&mut self) -> String {
        let mut delta = String::new();
        let curr_pos = self.get_pos_usize();
        println!("{}", curr_pos);
        let mut curr_char = self.contents.chars().nth(curr_pos).unwrap();
        while !curr_char.is_whitespace() {
            println!("curr_char: {}", curr_char);
            delta.push_str(curr_char.to_string().as_str());
            self.pos += 1;
            if self.get_pos_usize() >= self.contents.len() {
                curr_char = ' ';
            } else {
                curr_char = self.contents.chars().nth(self.get_pos_usize()).unwrap();
            }
        }

        delta
    }

    fn consume_until_newline(&mut self) -> String {
        let mut delta = String::new();
        let curr_pos = self.get_pos_usize();
        let mut curr_char = self.contents.chars().nth(curr_pos).unwrap();
        while curr_char != '\n' {
            delta.push_str(curr_char.to_string().as_str());
            self.consume_pos(1);
            if self.get_pos_usize() >= self.contents.len() {
                curr_char = ' ';
            } else {
                curr_char = self.contents.chars().nth(self.get_pos_usize()).unwrap();
            }
        }

        delta
    }

    fn consume_until_char(&mut self, c: char) -> String {
        let mut delta = String::new();
        let mut curr_pos = self.get_pos_usize();
        while self.peek().unwrap() == c {
            let curr_char = self.contents.chars().nth(curr_pos).unwrap().to_string();
            println!("until char: {}", curr_char);
            delta.push_str(curr_char.as_str());
            self.consume_pos(1);
            curr_pos = self.get_pos_usize();
            if curr_pos >= self.contents.len() {
                break;
            }
        }

        delta
    }

    fn consume_pos(&mut self, num: u8) {
        self.pos += u64::try_from(num).unwrap();
    }

    pub fn lex(&mut self) -> Vec<TokenDef> {
        let mut token_defs = Vec::new();
        let mut tokenize = true;

        while tokenize {
            let mut ch = char::default();
            match self.consume() {
                Ok(c) => {
                    ch = c;
                }
                Err(b) => {
                    tokenize = b;
                }
            };

            println!("{}", ch.to_string());

            if ch.is_whitespace() {
                self.consume_pos(1);
            } else if ch.is_numeric() {
                let start_pos = self.pos.clone();
                let delta_str = self.consume_until_whitespace();
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::Numeric,
                    meta: delta_str,
                };
                token_defs.push(def);
            } else if ch == '#' {
                let start_pos = self.pos.clone();
                self.consume_pos(1);
                let delta_str = self.consume_until_newline();
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::Comment,
                    meta: delta_str.trim().to_owned(),
                };
                token_defs.push(def);
            } else if ch == 'f' && self.peek().unwrap() == '"' {
                println!("coming inside f: {}", ch);
                let start_pos = self.pos.clone();
                self.consume_pos(1);
                let delta_str = self.consume_until_char('"');
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::FormatStringScope,
                    meta: delta_str,
                };
                token_defs.push(def);
            } else if ch == '"' {
                let start_pos = self.pos.clone();
                let delta_str = self.consume_until_char('"');
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::StringScope,
                    meta: delta_str,
                };
                token_defs.push(def);
            } else if ch == '=' {
                let start_pos = self.pos.clone();
                self.consume_pos(1);
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::Equals,
                    meta: String::from("="),
                };
                token_defs.push(def);
            } else if ch.is_alphabetic() {
                let start_pos = self.pos.clone();
                let delta_str = self.consume_until_whitespace();
                let def = TokenDef {
                    start: start_pos,
                    end: self.pos.clone(),
                    token: Token::Def,
                    meta: delta_str,
                };
                token_defs.push(def);
            }
        }

        println!("exited from while loop");

        token_defs
    }
}
