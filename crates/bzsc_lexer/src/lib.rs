/*
   Copyright 2021 BlazifyOrg
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#![allow(unused_assignments)]
use bzs_shared::{DynType, Error, Position, Token, Tokens};

pub fn get_keywords() -> Vec<String> {
    vec![
        string("val"),
        string("var"),
        string("and"),
        string("or"),
        string("not"),
        string("if"),
        string("else"),
        string("for"),
        string("to"),
        string("step"),
        string("while"),
        string("fun"),
        string("return"),
        string("class"),
        string("new"),
    ]
}

pub fn get_number() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn string(str: &str) -> String {
    return String::from(str);
}

pub fn get_ascii_letters() -> Vec<&'static str> {
    "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .split("")
        .collect::<Vec<&str>>()
}

pub fn get_ascii_letters_and_digits() -> Vec<&'static str> {
    "_0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .split("")
        .collect::<Vec<&str>>()
}

pub struct Lexer {
    pub file_name: String,
    pub text: String,
    pub current_char: Option<char>,
    pub position: Position,
}

impl Lexer {
    pub fn new(file_name: &'static str, text: &'static str) -> Lexer {
        let lexer = Lexer {
            file_name: String::from(file_name),
            text: String::from(text),
            current_char: Some(text.chars().collect::<Vec<char>>()[0]),
            position: Position::new(0, file_name, text),
        };
        lexer
    }

    fn advance(&mut self) {
        self.position.advance();
        if self.text.len() > self.position.index {
            let split: Vec<char> = self.text.chars().collect::<Vec<char>>();
            self.current_char = Some(split[self.position.index]);
        } else {
            self.current_char = None;
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![];

        while self.current_char.is_some() {
            let start = self.position.clone();
            let mut end = self.position.clone();
            end.advance();

            if [' ', '\t', '\r'].contains(&self.current_char.unwrap()) {
                self.advance();
                continue;
            }

            if ['\n', ';'].contains(&self.current_char.unwrap()) {
                let pos_start = self.position.clone();
                self.advance();
                tokens.push(Token::new(
                    Tokens::Newline,
                    pos_start,
                    self.position.clone(),
                    DynType::None,
                ));
                continue;
            }

            let token = match self.current_char.unwrap() {
                '+' => Tokens::Plus,
                '-' => Tokens::Minus,
                '*' => Tokens::Multiply,
                '/' => Tokens::Divide,
                '(' => Tokens::LeftParenthesis,
                ')' => Tokens::RightParenthesis,
                '{' => Tokens::LeftCurlyBraces,
                '}' => Tokens::RightCurlyBraces,
                '[' => Tokens::LeftSquareBraces,
                ']' => Tokens::RightSquareBraces,
                '^' => Tokens::Power,
                ':' => Tokens::Colon,
                ',' => Tokens::Comma,
                '.' => Tokens::Dot,
                _ => Tokens::Unknown,
            };

            let mut token_is_unknown = false;
            if token == Tokens::Unknown {
                match self.current_char.unwrap() {
                    '@' => self.skip_comment(),
                    '"' => tokens.push(self.make_string()),
                    '!' => tokens.push(self.make_not()),
                    '<' => tokens.push(self.make_less_than()),
                    '>' => tokens.push(self.make_greater_than()),
                    '=' => tokens.push(self.make_equals()),
                    '\'' => {
                        let result = self.make_char();
                        match result {
                            Ok(token) => tokens.push(token),
                            Err(e) => {
                                return Err(e);
                            }
                        };
                    }
                    '|' => {
                        let result = self.make_or();
                        match result {
                            Ok(token) => tokens.push(token),
                            Err(e) => {
                                return Err(e);
                            }
                        };
                    }
                    '&' => {
                        let result = self.make_and();
                        match result {
                            Ok(token) => tokens.push(token),
                            Err(e) => {
                                return Err(e);
                            }
                        };
                    }
                    _ => {
                        let no = self.current_char.unwrap().to_digit(36);
                        if no.is_some() {
                            if get_number().contains(&no.unwrap())
                                || self.current_char.unwrap() == '.'
                            {
                                tokens.push(self.make_number());
                            } else if get_ascii_letters()
                                .contains(&self.current_char.unwrap().to_string().as_str())
                            {
                                tokens.push(self.make_identifiers());
                            } else {
                                token_is_unknown = true;
                            }
                        } else {
                            token_is_unknown = true;
                        }
                    }
                }
            } else {
                tokens.push(Token::new(token, start, end, DynType::None));
                self.advance();
            }

            if token_is_unknown {
                let start_1 = self.position.clone();
                self.position.advance();
                let char = self.current_char.unwrap().to_string();
                return Err(Error::new(
                    "Illegal Character",
                    start_1,
                    self.position.clone(),
                    Box::leak(
                        format!("Unexpected Character '{}'", char)
                            .to_owned()
                            .into_boxed_str(),
                    ),
                ));
            }
        }

        tokens.push(Token::new(
            Tokens::EOF,
            self.position.clone(),
            self.position.clone(),
            DynType::None,
        ));
        Ok(tokens)
    }

    fn make_number(&mut self) -> Token {
        let mut str_num = String::new();
        let mut dot_count = 0;
        let start = self.position.clone();

        while self.current_char.is_some() {
            if self.current_char.unwrap().to_digit(36).is_none()
                && self.current_char.unwrap() != '.'
            {
                break;
            }
            if self.current_char.unwrap() == '.' {
                dot_count += 1;
            }
            str_num.push(self.current_char.unwrap());
            self.advance();
        }

        return if dot_count > 0 {
            Token::new(
                Tokens::Float,
                start,
                self.position.clone(),
                DynType::Float(str_num.parse::<f64>().unwrap()),
            )
        } else {
            Token::new(
                Tokens::Int,
                start,
                self.position.clone(),
                DynType::Int(str_num.parse::<i128>().unwrap()),
            )
        };
    }

    fn make_string(&mut self) -> Token {
        let mut str_raw = String::new();
        let start = self.position.clone();
        let mut escape = true;
        self.advance();

        while self.current_char.is_some() || escape {
            if self.current_char.unwrap() == '"' {
                break;
            }
            if escape {
                if self.current_char.unwrap_or(' ') == '\n' {
                    str_raw.push('\n');
                } else if self.current_char.unwrap_or(' ') == '\t' {
                    str_raw.push('\t');
                } else {
                    str_raw.push(self.current_char.unwrap());
                }
            } else {
                if self.current_char.unwrap_or(' ') == '\\' {
                    escape = true;
                } else {
                    str_raw.push(self.current_char.unwrap());
                }
            }

            self.advance();
            escape = false;
        }

        self.advance();

        Token::new(
            Tokens::String,
            start,
            self.position.clone(),
            DynType::String(str_raw),
        )
    }

    fn make_char(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();

        self.advance();
        let new_char = self.current_char;
        self.advance();

        if self.current_char.unwrap_or(' ') != '\'' {
            return Err(Error::new(
                "Expected Character",
                start,
                self.position.clone(),
                "Expected Character \"'\" because chars are unicode characters.",
            ));
        }

        self.advance();

        Ok(Token::new(
            Tokens::Char,
            start,
            self.position.clone(),
            DynType::Char(new_char.unwrap()),
        ))
    }

    fn make_equals(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(
                Tokens::DoubleEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        } else if self.current_char.unwrap_or(' ') == '>' {
            self.advance();
            return Token::new(Tokens::Arrow, start, self.position.clone(), DynType::None);
        }

        Token::new(Tokens::Equals, start, self.position.clone(), DynType::None)
    }

    fn make_less_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            return Token::new(
                Tokens::LessThanEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::LessThan,
            start,
            self.position.clone(),
            DynType::None,
        )
    }

    fn make_greater_than(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            return Token::new(
                Tokens::GreaterThanEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::GreaterThan,
            start,
            self.position.clone(),
            DynType::None,
        )
    }

    fn make_not(&mut self) -> Token {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '=' {
            self.advance();
            return Token::new(
                Tokens::NotEquals,
                start,
                self.position.clone(),
                DynType::None,
            );
        }

        Token::new(
            Tokens::Keyword,
            start,
            self.position.clone(),
            DynType::String("not".to_string()),
        )
    }

    fn make_or(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '|' {
            self.advance();
            return Ok(Token::new(
                Tokens::Keyword,
                start,
                self.position.clone(),
                DynType::String("or".to_string()),
            ));
        }

        Err(Error::new(
            "Expected Character",
            start,
            self.position.clone(),
            "Expected one more '|'",
        ))
    }

    fn make_and(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();
        self.advance();

        if self.current_char.unwrap_or(' ') == '&' {
            self.advance();
            return Ok(Token::new(
                Tokens::Keyword,
                start,
                self.position.clone(),
                DynType::String("and".to_string()),
            ));
        }

        Err(Error::new(
            "Expected Character",
            start,
            self.position.clone(),
            "Expected one more '&'",
        ))
    }

    fn make_identifiers(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();

        while self.current_char.is_some() {
            if !get_ascii_letters_and_digits()
                .contains(&self.current_char.unwrap().to_string().as_str())
            {
                break;
            }
            identifier.push(self.current_char.unwrap());
            self.advance();
        }

        let identifier_type = if get_keywords().contains(&identifier) {
            Tokens::Keyword
        } else if identifier == "true".to_string() || identifier == "false".to_string() {
            Tokens::Boolean
        } else {
            Tokens::Identifier
        };
        Token::new(
            identifier_type,
            start,
            self.position.clone(),
            if identifier_type != Tokens::Boolean {
                DynType::String(identifier)
            } else {
                DynType::Boolean(identifier.parse::<bool>().unwrap())
            },
        )
    }

    pub fn skip_comment(&mut self) {
        self.advance();

        if self.current_char.unwrap() == '@' {
            while self.current_char.is_some() {
                self.advance();
                if self.current_char.unwrap() == '@' {
                    self.advance();
                    if self.current_char.unwrap() == '@' {
                        break;
                    }
                }
            }
        }

        while self.current_char.is_some() {
            if self.current_char.unwrap() == '\n' {
                break;
            }
            self.advance();
        }

        self.advance();
    }
}
