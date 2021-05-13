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
use crate::compiler::bytecode::bytecode::ByteCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tokens {
    Int,
    Float,
    String,
    Boolean,
    Char,
    Colon,
    Comma,
    Dot,
    Arrow,
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBraces,
    RightCurlyBraces,
    LeftSquareBraces,
    RightSquareBraces,
    Power,
    Keyword,
    Identifier,
    Equals,
    DoubleEquals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Newline,
    EOF,
    Unknown,
}

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
        string("soul"),
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

#[derive(Debug, PartialEq, Clone)]
pub enum DynType {
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    None,
}

impl DynType {
    pub fn into_int(&self) -> i128 {
        if let DynType::Int(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_float(&self) -> f64 {
        if let DynType::Float(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_string(&self) -> String {
        if let DynType::String(i) = self {
            i.to_string()
        } else {
            panic!()
        }
    }

    pub fn into_char(&self) -> char {
        if let DynType::Char(i) = self {
            *i
        } else {
            panic!()
        }
    }

    pub fn into_boolean(&self) -> bool {
        if let DynType::Boolean(i) = self {
            *i
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constants {
    None,
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Function(Vec<u16>, ByteCode),
    Array(Vec<Constants>),
    Object(HashMap<usize, Constants>),
}
