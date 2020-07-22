use crate::leptjson::LeptType::LeptNumber;
use crate::Result;
use crate::Result::LeptParseInvalidValue;

#[derive(Debug)]
struct Request {
    context: String,
}

impl Request {
    fn new(context: String) -> Self {
        Self { context }
    }
}

#[derive(Debug, PartialEq)]
pub enum LeptType {
    LeptNull,
    LeptFalse,
    LeptTrue,
    LeptNumber(f64),
    LeptString,
    LeptArray,
    LeptObject,
}

pub struct LeptValue {
    pub value: LeptType,
}

/// parse a json string
impl LeptValue {
    pub fn new(value: LeptType) -> Self {
        Self { value }
    }

    pub fn parse(&mut self, json: String) -> Result {
        let mut request = Request::new(json);
        self.value = LeptType::LeptNull;
        parse_whitespace(&mut request);
        let result = self.parse_value(&mut request);
        if let Result::LeptParseOk = result {
            parse_whitespace(&mut request);
            if !request.context.is_empty() {
                return Result::LeptpParseRootNotSingular;
            }
        };
        result
    }

    fn parse_literal(&mut self, literal: &str, request: &mut Request) -> Result {
        let true_value = literal;
        if !request.context.starts_with(true_value) {
            return Result::LeptParseInvalidValue;
        }
        request.context = request.context.trim_start_matches(true_value).to_string();
        self.value = LeptType::LeptNull;
        Result::LeptParseOk
    }

    fn parse_number(&mut self, request: &mut Request) -> Result {
        let chars = request.context.as_bytes();
        let mut i: usize = 0;
        if chars[i].eq(&(b'-')) {
            i += 1;
        }
        if chars[i].eq(&(b'0')) {
            i += 1;
        } else {
            if !is_digit_1_to_9(&chars[i]) {
                return LeptParseInvalidValue;
            }
            loop {
                i += 1;
                if !chars[i].is_ascii_digit() {
                    break;
                }
            }
        }

        if chars[i].eq(&(b'.')) {
            i += 1;
            if !chars[i].is_ascii_digit() {
                return LeptParseInvalidValue;
            }
            loop {
                i += 1;
                if !chars[i].is_ascii_digit() {
                    break;
                }
            }
        }

        if chars[i].eq(&(b'e')) || chars[i].eq(&(b'E')) {
            i += 1;
            if chars[i].eq(&(b'+')) || chars[i].eq(&(b'-')) {
                i += 1;
            }
            if !chars[i].is_ascii_digit() {
                return LeptParseInvalidValue;
            }
            loop {
                i += 1;
                if !chars[i].is_ascii_digit() {
                    break;
                }
            }
        }

        match request.context.parse::<f64>() {
            Ok(v) => {
                self.value = LeptNumber(v);
                return Result::LeptParseOk
            }
            Err(_) => return Result::LeptParseNumberTooBig,
        };
    }

    fn parse_value(&mut self, request: &mut Request) -> Result {
        match request.context.chars().next().unwrap_or('\0') {
            'n' => self.parse_literal(&String::from("null"), request),
            't' => self.parse_literal(&String::from("true"), request),
            'f' => self.parse_literal(&String::from("false"), request),
            '\0' => Result::LeptParseExpectValue,
            _ => self.parse_number(request),
        }
    }
}

fn parse_whitespace(request: &mut Request) {
    let pattern: &[_] = &[' ', '\n', '\r', '\t'];
    request.context = request.context.trim_start_matches(pattern).parse().unwrap()
}

fn is_digit_1_to_9(ch: &u8) -> bool {
    ch.is_ascii_digit() && ch.ne(&(b'0'))
}
