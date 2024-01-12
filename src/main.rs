// Title: lexer
//
use std::fs;
#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Equal,
    LessThan,
    GreaterThan,
    LParen,
    RParen,
    Print,
    EOF,
    StringLiteral(String),
    TypeConversion(TypeConversion),
}
#[derive(Debug, PartialEq)]
enum TypeConversion {
    IntToFloat,
    FloatToInt,
    StringToInt(String),
    StringToFloat(String),
    IntToString,
    FloatToString,
}
struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn consume(&mut self) {
        self.position += 1;
    }
    fn quit_whitespace(&mut self) {
        while self.input.as_bytes()[self.position] == b' ' {
            self.consume();
        }
    }
    fn next_token(&mut self) -> Token {
        self.quit_whitespace();

        if let Some(c) = self.current_char() {
            match c {
                // handling operators
                b'+' => {
                    self.consume();
                    Token::Plus
                }
                b'-' => {
                    self.consume();
                    Token::Minus
                }
                b'*' => {
                    self.consume();
                    Token::Asterisk
                }

                b'!' => {
                    self.consume();
                    Token::Bang
                }
                b'=' => {
                    self.consume();
                    Token::Equal
                }
                b'<' => {
                    self.consume();
                    Token::LessThan
                }
                b'>' => {
                    self.consume();
                    Token::GreaterThan
                }
                b'(' => {
                    self.consume();
                    Token::LParen
                }
                b')' => {
                    self.consume();
                    Token::RParen
                }
                b'/' => {
                    self.consume();
                    Token::Slash
                }
                // handling type conversions
                b'i' => {
                    if self.input[self.position..].starts_with("int") {
                        self.position += 4; // Move position here
                        let start = self.position;
                        let token =
                            if let Some(first_char) = self.input[self.position..].chars().next() {
                                if first_char == '"' {
                                    while let Some(c) = self.current_char() {
                                        if c == b'"' {
                                            self.consume();
                                            break;
                                        }
                                        self.consume();
                                    }
                                    let input_str = &self.input[start + 1..self.position + 1];
                                    Token::TypeConversion(TypeConversion::StringToInt(
                                        input_str.to_string(),
                                    ))
                                } else if first_char.is_ascii_digit() {
                                    Token::TypeConversion(TypeConversion::FloatToInt)
                                } else {
                                    println!("{}", first_char);
                                    panic!("Unknown token: {}", first_char);
                                }
                            } else {
                                panic!("Unexpected end of input");
                            };
                        token
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }

                b'f' => {
                    if self.input[self.position..].starts_with("float") {
                        self.position += 6; //Move position here
                        let start = self.position;
                        let token =
                            if let Some(first_char) = self.input[self.position..].chars().next() {
                                if first_char == '"' {
                                    while let Some(c) = self.current_char() {
                                        if c == b'"' {
                                            self.consume();
                                            break;
                                        }
                                        self.consume();
                                    }
                                    let input_str = &self.input[start + 1..self.position + 1];
                                    println!("{}", input_str);

                                    Token::TypeConversion(TypeConversion::StringToFloat(
                                        input_str.to_string(),
                                    ))
                                } else if first_char.is_ascii_digit() {
                                    Token::TypeConversion(TypeConversion::IntToFloat)
                                } else {
                                    println!("{}", first_char);
                                    panic!("Unknown token: {}", first_char);
                                }
                            } else {
                                panic!("Unexpected end of input");
                            };
                        token
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }

                b's' => {
                    if self.input[self.position..].starts_with("str") {
                        self.position += 3;
                        Token::TypeConversion(TypeConversion::IntToString)
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }

                // handling print statement
                b'p' => {
                    if self.input[self.position..].starts_with("print") {
                        self.position += 5;
                        Token::Print
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }
                // handling single line comments
                b'#' => {
                    while let Some(c) = self.current_char() {
                        if c == b'\n' {
                            break;
                        }
                        self.consume();
                    }

                    self.next_token()
                }
                // handling new line
                b'\n' => {
                    while let Some(c) = self.current_char() {
                        if c == b'\n' {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    self.next_token()
                }

                //escape characters
                b'\\' => {
                    self.consume();
                    self.next_token()
                }

                // handling multi line comments
                b'"' => {
                    if self.input[self.position..].starts_with("\"\"\"") {
                        self.consume();
                        self.position += 3;
                        while let Some(c) = self.current_char() {
                            if c == b'"' {
                                if self.input[self.position..].starts_with("\"\"\"") {
                                    self.position += 3;
                                    break;
                                }
                            }
                            self.consume();
                        }
                        self.next_token()
                    } else if self.input[self.position..].starts_with("\"") {
                        let start = self.position;
                        let end = self.input[self.position + 1..].find("\"").unwrap();
                        while let Some(c) = self.current_char() {
                            if c == b'"' {
                                self.consume();
                                break;
                            }
                            self.consume();
                        }
                        self.position += end + 2;
                        let input = &self.input[start + 1..self.position - 2].to_string();

                        Token::StringLiteral(input.to_string())
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }
                // adding type conversions

                //handing variables
                _ => {
                    if c.is_ascii_digit() {
                        Token::Number(self.read_number())
                    } else if c.is_ascii_alphabetic() {
                        Token::Identifier(self.read_identifier())
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }
            }
        } else {
            Token::EOF
        }
    }
    fn current_char(&self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.position])
        }
    }
    fn read_number(&mut self) -> i32 {
        let mut number = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number.push(c as char);
                self.consume();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphabetic() {
                identifier.push(c as char);
                self.consume();
            } else {
                break;
            }
        }
        identifier
    }
}
fn main() {
    let input = read_file("test_files/main.py");
    println!("{}", input);

    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
    };

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
